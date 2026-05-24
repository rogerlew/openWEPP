---
contract_id: SC-SYSTEM-001
title: System Integration Boundary and Watershed Assembly Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 15
producer_scope:
  - Hillslope-to-watershed pass-file state/flux surfaces
  - Channel and impoundment boundary assembly surfaces
  - Cross-component event/daily closure and handoff semantics
consumer_scope:
  - Channel hydrology/erosion and impoundment routing consumers
  - Watershed outlet hydrograph/sediment-yield accounting consumers
  - Comparator/replay and governance-gate consumers
evidence_level: Static
last_reviewed: 2026-05-24
supersedes: []
superseded_by: []
---

# SC-SYSTEM-001 System Integration Boundary and Watershed Assembly Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for cross-component integration in WEPP
watershed simulation: hillslope pass-file payload completeness, channel/impoundment
runon-runoff assembly semantics, hydrograph merge behavior, and mass-continuity
constraints that must hold at integration boundaries.

## Scientific Scope

In scope:
- Handoff obligations between hillslope outputs and channel/impoundment inputs.
- Runon/runoff assembly, peak-runoff composition, and event-duration harmonization
  rules used when watershed elements are coupled.
- Impoundment stage-routing and sediment-mass continuity constraints required for
  valid downstream boundary publication.
- System-level guard obligations for explicit invalid-state failure (no silent
  boundary repair).

Out of scope:
- Internal kernel constitutive equations owned by domain contracts
  (`SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-SED-001`, `SC-IMPOUND-001`).
- Rust API naming finalization beyond canonical symbol-to-identity aliasing.
- Large-watershed extrapolation beyond chapter-declared applicability bounds.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-SYSTEM-CH1-WATERSHED | `references/50201000/chap1.pdf` §1.1 and Fig. 1.1.1 | Watershed decomposition into hillslopes, channels, and impoundments plus linkage semantics. | `[DIRECT][Static]` |
| REF-SYSTEM-CH1-COMPONENTS | `chap1.pdf` §1.4 and §1.4.11 | Continuous simulation component coupling and watershed extension assumptions. | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-PASSFILE | `references/50201000/chap13.pdf` intro paragraph before §13.2 | Required hillslope pass-file payload fields consumed by channel/impoundment components. | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-RUNON | `chap13.pdf` §13.2, Eq. [13.2.1]-[13.2.3] | Channel runon decomposition, depth conversion, and event-duration maximum rule. | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-TRANSLOSS | `chap13.pdf` §13.2, Eq. [13.2.4]-[13.2.6] | Case-partitioned transmission-loss and final-runoff assembly semantics. | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-PEAKIN | `chap13.pdf` §13.4.1, Eq. [13.4.1]-[13.4.2] | Triangular-hydrograph merge semantics when multiple watershed elements contribute runon. | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-PEAKOUT | `chap13.pdf` §13.4.2, Eq. [13.4.3]-[13.4.7], [13.4.25]-[13.4.26] | Channel/watershed outlet peak-runoff and duration semantics, including method options. | `[DIRECT][Static]` |
| REF-SYSTEM-WSHDRV-ORDER | `/workdir/wepp-forest_260430_baseline/src/wshdrv.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Watershed execution-order authority for channel lane (`wshcqi -> wshirs -> wshrun/wshpek`) including direct `wshchr` routing path when `ipeak > 2` and local channel runoff is absent. | `[DIRECT][Static]` |
| REF-SYSTEM-WSHPEK-IPEAK | `/workdir/wepp-forest_260430_baseline/src/wshpek.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | `ipeak`-selected outlet branch authority (`1` Rational, `2` CREAMS, `>=3` wave routing) and routed peak/duration publication semantics. | `[DIRECT][Static]` |
| REF-SYSTEM-WSHCHR-WAVE | `/workdir/wepp-forest_260430_baseline/src/wshchr.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy-equivalent channel wave-routing authority (linear kinematic-wave and Muskingum-Cunge branches, storage closure, routed `peakot`/`runvol`/`rundur` outputs). | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-SEDCONT | `chap13.pdf` §13.5.5, Eq. [13.5.17] | Channel sediment continuity with upstream/lateral inflow and flow detachment/deposition terms. | `[DIRECT][Static]` |
| REF-SYSTEM-CH14-HYDCONT | `references/50201000/chap14.pdf` §14.2, Eq. [14.2.1]-[14.2.5] | Impoundment hydraulic continuity and stage-discharge/stage-area coupling. | `[DIRECT][Static]` |
| REF-SYSTEM-CH14-ADAPT | `chap14.pdf` §14.2.3, Eq. [14.2.7]-[14.2.9] | Adaptive timestep semantics and mandatory minimum-step reset at regime transitions. | `[DIRECT][Static]` |
| REF-SYSTEM-CH14-OUTFLOW | `chap14.pdf` §14.3.8, Eq. [14.3.18] | Total outflow as explicit sum of active outlet-structure contributions. | `[DIRECT][Static]` |
| REF-SYSTEM-CH14-SEDCONT | `chap14.pdf` §14.6.1, Eq. [14.6.1] | Impoundment sediment mass continuity for effluent concentration computation. | `[DIRECT][Static]` |
| REF-SYSTEM-CH2-BRKPT | `references/50201000/chap2.pdf` §2.2, Table 2.2.1 convention text | Breakpoint rainfall sequence conventions required by channel infiltration/runoff assembly. | `[DIRECT][Static]` |
| REF-SYSTEM-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative volumetric fluxes, explicit branch behavior, and conservation-consistent handoff are required for physically valid integration. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `durstorm` | `s` | Hillslope storm duration exported in pass file. | hillslope component | channel/impoundment runon assembly |
| `tc_h` | `h` | Overland-flow time of concentration exported from hillslope. | hillslope component | channel peak-flow assembly |
| `alpha` | `fraction` | Dimensionless Rational-equation shape parameter from contributor element. | hillslope/channel/impoundment element | downstream channel peak-flow assembly |
| `qdepth` | `m` | Runoff depth exported by contributing element. | hillslope/channel/impoundment element | downstream runon assembly |
| `rof` | `m^3` | Runoff volume exported by contributing element. | hillslope/channel/impoundment element | downstream runon/hydrograph assembly |
| `qp` | `m^3 s^-1` | Peak runoff from contributing element. | hillslope/channel/impoundment element | downstream hydrograph merge |
| `det_hs` | `kg` | Total hillslope detachment at endpoint. | hillslope erosion component | channel sediment-load assembly |
| `dep_hs` | `kg` | Total hillslope deposition at endpoint. | hillslope erosion component | watershed sediment bookkeeping |
| `Csed,k` | `kg m^-3` | Sediment concentration for particle class `k` at handoff. | hillslope/channel/impoundment element | downstream sediment-routing component |
| `Fsize,k` | `fraction` | Fraction of particle class `k` in eroded sediment payload. | hillslope/channel/impoundment element | downstream sediment-routing component |
| `rov`, `rol`, `roi` | `m^3` | Total, lateral, and inlet runon volumes for channel inlet assembly. | channel integration routine | channel runoff assembly |
| `rod` | `m` | Runon depth computed from runon volume and channel area. | channel integration routine | channel runoff case logic |
| `Ach` | `m^2` | Physical channel area used for depth conversion. | channel geometry surface | channel runon depth conversion |
| `durc`, `durrunon`, `durchan`, `durirrig` | `s` | Channel event duration and duration contributors (runon max, local storm, irrigation). | integration/climate/irrigation surfaces | channel runon/runoff and peak-flow logic |
| `qci`, `qcf` | `m` | Initial and final channel runoff depth during case partition logic. | channel runoff routine | channel runoff volume/peak computation |
| `tl` | `m^3` | Channel transmission-loss volume. | channel runoff routine | channel water-balance accounting |
| `tb`, `tp` | `min` | Synthetic-hydrograph base time and time-to-peak for contributor hydrographs. | hydrograph merge routine | channel/impoundment peak assembly |
| `Aw`, `qa`, `qpi` | `m^2`, `m`, `m^3 s^-1` | Contributor area, average runoff depth, and peak flow used for synthetic hydrograph construction. | contributor element set | hydrograph merge routine |
| `qpo` | `m^3 s^-1` | Peak runoff at channel/watershed outlet. | channel outlet routine | downstream routing/reporting |
| `tc`, `tcc`, `tcs`, `tci` | `h` | Total time of concentration and channel/overland/impoundment components. | channel outlet routine | peak-flow method |
| `Qi`, `Qo` | `ft^3 s^-1` | Impoundment inflow and outflow rates in continuity expression. | impoundment routing routine | stage and sediment routing |
| `H`, `Aimp` | `ft`, `ft^2` | Impoundment stage and stage-dependent area. | impoundment routing routine | hydraulic continuity integration |
| `Qtotal` | `ft^3 s^-1` | Sum outflow from all active outflow structures at current stage. | impoundment outlet routine | downstream hydrograph and sediment routing |
| `M`, `Ci`, `Co`, `Dep` | `lb`, `lb ft^-3`, `lb ft^-3`, `lb` | Impoundment sediment mass, influent concentration, effluent concentration, and deposited mass. | impoundment sedimentation routine | watershed sediment accounting |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SYSTEM-001 | Pass-file completeness invariant: all required hillslope boundary fields listed in Chapter 13 intro (storm duration, overland `tc`, `alpha`, runoff depth/volume/peak, endpoint detachment/deposition, particle-class concentrations, and size fractions) are present with declared units before channel/impoundment consumption. | hard-fail | REF-SYSTEM-CH1-WATERSHED, REF-SYSTEM-CH13-PASSFILE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-002 | Channel runon decomposition invariant: runon identity and depth conversion hold (`rov = rol + roi`, `rod = rov/Ach`) with non-negative volumes/area and explicit contributor accounting. | hard-fail | REF-SYSTEM-CH13-RUNON, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-003 | Channel duration harmonization invariant: event duration uses the explicit max rule `durc = max(durrunon, durchan, durirrig)` and all duration terms share consistent event basis/units. | hard-fail | REF-SYSTEM-CH13-RUNON | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-004 | Channel runoff-case invariant: channel runoff must follow the four-case partition of §13.2, including transmission-loss equations (`tl`) for applicable cases and explicit zero branch for Case IV (`qci = 0`, `rod = 0` implies `qcf = 0`, `rof_f = 0`). | hard-fail | REF-SYSTEM-CH13-TRANSLOSS | `[DIRECT][Static]` |
| INV-SYSTEM-005 | Hydrograph-merge invariant: with one upstream contributor, inlet peak is direct pass-through; with multiple contributors, SCS triangular synthetic hydrographs are computed and superimposed before selecting inlet peak. For `ipeak <= 2`, peak/duration calculations are skipped when `rof_f <= 0.001 m^3`; for `ipeak >= 3`, routed channel flow may still be evaluated from incoming hydrograph when local runoff is absent. | hard-fail | REF-SYSTEM-CH13-PEAKIN, REF-SYSTEM-CH13-PEAKOUT, REF-SYSTEM-WSHDRV-ORDER, REF-SYSTEM-WSHPEK-IPEAK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-006 | Outlet method-branch invariant: channel/watershed outlet routing uses exactly one declared `ipeak` branch (`1` modified Rational, `2` CREAMS, `3` linear kinematic-wave, `>=4` Muskingum-Cunge) with explicit branch identity retained in outputs and no implicit fallback switching. | hard-fail | REF-SYSTEM-CH13-PEAKOUT, REF-SYSTEM-WSHPEK-IPEAK, REF-SYSTEM-WSHCHR-WAVE, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-007 | Impoundment hydraulic continuity invariant: stage update obeys continuity (`dV/dt = Qi - Qo`, `dH/dt = (Qi - fQo(H))/fA(H)`), and adaptive timestep reset to minimum is enforced whenever outflow regime changes or inflow starts/ends. | hard-fail | REF-SYSTEM-CH14-HYDCONT, REF-SYSTEM-CH14-ADAPT | `[DIRECT][Static]` |
| INV-SYSTEM-008 | Impoundment outflow aggregation invariant: total outflow equals sum of active outlet-structure contributions (`Qtotal`) and each inactive structure contributes exactly zero when stage is below its inlet threshold. | hard-fail | REF-SYSTEM-CH14-OUTFLOW | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-009 | Sediment continuity invariant: channel sediment continuity and impoundment sediment continuity equations are enforced with explicit upstream/lateral loads and deposition terms; no untracked mass creation/loss across handoff boundaries is allowed. | hard-fail | REF-SYSTEM-CH13-SEDCONT, REF-SYSTEM-CH14-SEDCONT, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-010 | Breakpoint forcing invariant: when channel runoff is event-driven, rainfall inputs preserve breakpoint sequence semantics required by Chapter-2 conventions (explicit interval times/intensities and end-of-storm zero-intensity termination). | governance-fail | REF-SYSTEM-CH13-RUNON, REF-SYSTEM-CH2-BRKPT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-011 | INT10 cross-lane publication invariant: hillslope boundary publication to watershed/channel integration is allowed only after canonical daily plant/hydrology lane closure (`decomp -> growth -> watbal`) has completed without typed ordering/state-transfer violations. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-CH13-PASSFILE, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-012 | PL14 strict replay provenance invariant: Tier-A replay lane execution must explicitly surface missing required replay artifacts (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`), strict-diff status, and provenance-hash gaps as typed gate failures or `HOLD` signals; no synthetic fallback artifact substitution is allowed. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-013 | PL15 Tier-A closeout governance invariant: unresolved strict Tier-A deltas remain blocking unless explicit risk-acceptance approval reference (owner, rationale, and scope) is recorded; silent tier down-classification and implicit risk acceptance are forbidden. | governance-fail | REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-014 | PL14R strict replay rerun reproducibility invariant: post-closure-wave Tier-A rerun must stage required interchange surfaces (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`), persist strict comparator JSON artifacts, and record command/binary/tool/output hashes; missing required surfaces, comparator artifacts, or provenance hashes must hard-fail replay disposition and keep the lane in `HOLD`. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-015 | PL15R Tier-A recloseout supersession invariant: refreshed hold-lift governance must classify residual Tier-A deltas from the latest PL14R schema-aligned strict replay evidence set (parquet comparator JSON artifacts plus day-by-day parity evidence) and must treat stale pre-supersession strict-failure signatures as historical context, not active blockers. If unresolved Tier-A blockers remain after supersession, explicit risk-acceptance reference is still mandatory. | governance-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-016 | WB20 forward-solver parity publication invariant: Tier-A forward-solver lane publication must include explicit lane-manifest evidence showing `wb20_forward_solver_lane_enabled = 1` and confirming observed closure targets are excluded from acceptance-driving inputs; missing manifest/no-substitution evidence is an unresolved governance blocker. | governance-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-017 | PL14S semantic replay-evidence invariant: Tier-A hillslope replay publication must persist semantic comparator evidence (`h5_wat_semantic_comparator.json`) with row-key presence deltas, per-column tolerance verdicts, and top divergent-key diagnostics for investigation; missing or malformed semantic report evidence, or suppressed strict-comparator skip/execution status in provenance output, is a hard-fail/HOLD condition. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-SYSTEM-001` | runtime | Boundary payload validator at hillslope->channel/impoundment handoff | Typed hard error on missing field, malformed units, or absent class vectors | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-002` | runtime | Channel inlet runon assembler | Typed hard error on decomposition/area/volume domain failure | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-003` | runtime | Channel event-duration assembly | Typed hard error on inconsistent duration basis or max-rule violation | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-004` | runtime | Channel runoff case-switch and transmission-loss calculator | Typed hard error when case outputs violate required equations or zero branch | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-SYSTEM-005` | runtime | Hydrograph merge and threshold/routed-gating logic | Typed hard error on invalid contributor-merge branch or threshold/routed-gating handling | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-006` | runtime | `ipeak` outlet branch selector/executor | Typed hard error on ambiguous, mixed, or undefined outlet branch in single evaluation path | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-007` | runtime | Impoundment continuity integrator and timestep controller | Typed hard error on continuity mismatch or missing minimum-step reset at regime transitions | Tier-B gate | `[DIRECT][Static]` |
| `INV-SYSTEM-008` | runtime | Impoundment outflow aggregator | Typed hard error on structure-sum mismatch or inactive-structure non-zero contribution | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-009` | runtime | Channel and impoundment sediment continuity routines | Typed hard error on continuity residual beyond tolerance or missing load term | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-010` | governance | Review/disposition/verification and promotion checklist | Promotion `HOLD` until breakpoint forcing semantics are contractually closed with companion climate/runoff contracts | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-011` | runtime | Hillslope daily-lane closure gate at system boundary publish handoff | Typed hard error and publish block when coupled plant/hydrology lane ordering or transfer closure fails | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-012` | runtime + governance | Comparator replay harness staging + disposition gate | Typed hard error or explicit `HOLD` when required replay artifacts/provenance hashes are missing, or strict comparator failure is masked | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-013` | governance | PL15 closeout criteria matrix + final decision record + conditional risk-acceptance artifact reference | Promotion `HOLD` when unresolved Tier-A deltas lack explicit approval reference; reject implicit risk-accept posture | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-014` | runtime + governance | PL14R rerun staging + provenance manifest gate | Typed hard error / explicit `HOLD` when required replay interchange surfaces, strict comparator JSON artifacts, or reproducibility hashes are missing | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-015` | governance | PL15R recloseout criteria matrix + refreshed hold-lift decision record + supersession references | Governance `HOLD` when active Tier-A blocker classification is derived from stale pre-supersession deltas, or when post-supersession unresolved blockers lack explicit approval reference | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-016` | governance | WB20 forward-solver lane publication checklist + disposition evidence gate | Governance `HOLD` when forward-lane manifest/no-substitution evidence is missing or does not prove observed-target exclusion from acceptance logic | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-017` | runtime + governance | PL14S semantic comparator publication gate + provenance integrity checklist | Typed hard error / explicit `HOLD` when semantic comparator JSON or required investigation diagnostics are missing, or when provenance omits strict-comparator skipped/executed posture | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols follow chapter authority notation. openWEPP boundary/API field
names for system-integration surfaces are not finalized in this cycle; identity
aliases remain mandatory placeholders until implementation contracts provide
explicit divergent names.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `durstorm`, `tc_h`, `alpha`, `qdepth`, `rof`, `qp` | identity names | hillslope pass-file payload | chapter-declared units preserved | `[DIRECT][Static]` |
| `det_hs`, `dep_hs` | identity names | hillslope sediment endpoint payload | `kg` -> `kg` | `[DIRECT][Static]` |
| `Csed,k`, `Fsize,k` | identity names | particle-class concentration/fraction vectors | `kg m^-3` and `fraction` preserved | `[DIRECT][Static]` |
| `rov`, `rol`, `roi`, `rod`, `Ach` | identity names | channel runon-runoff assembly | `m^3`/`m`/`m^2` preserved | `[DIRECT][Static]` |
| `durc`, `durrunon`, `durchan`, `durirrig` | identity names | channel event-duration harmonization surfaces | `s` -> `s` | `[DIRECT][Static]` |
| `qci`, `qcf`, `tl` | identity names | channel runoff-case and transmission-loss surfaces | `m` and `m^3` preserved | `[DIRECT][Static]` |
| `tb`, `tp`, `Aw`, `qa`, `qpi` | identity names | synthetic hydrograph merge surfaces | `min`, `m^2`, `m`, `m^3 s^-1` preserved | `[DIRECT][Static]` |
| `qpo`, `tc`, `tcc`, `tcs`, `tci` | identity names | channel/watershed outlet peak-flow surface | `m^3 s^-1` and `h` preserved | `[DIRECT][Static]` |
| `Qi`, `Qo`, `H`, `Aimp`, `Qtotal` | identity names | impoundment hydraulic routing surfaces | `ft^3 s^-1`, `ft`, `ft^2` preserved | `[DIRECT][Static]` |
| `M`, `Ci`, `Co`, `Dep` | identity names | impoundment sediment-mass surfaces | `lb` and `lb ft^-3` preserved | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| No-runon/no-runoff channel event | Case IV branch with `qci = 0`, `rod = 0`, `qcf = 0`, and `rof_f = 0`. | Explicitly defined valid zero-flow branch in §13.2 case logic. | `[DIRECT][Static]` |
| Single-contributor merge | Exactly one watershed element contributes runon and inlet peak equals that contributor's peak. | Explicit §13.4.1 merge rule for single source. | `[DIRECT][Static]` |
| Sub-threshold runoff event | `rof_f <= 0.001 m^3` yields zero peak/duration and skip to downstream element. | Explicit §13.4.1 threshold behavior. | `[DIRECT][Static]` |
| Inactive outlet structures | Outflow contribution from a structure remains zero when stage is below structure inlet threshold. | Explicit §14.3.8 summation logic. | `[DIRECT][Static]` |
| Zero sediment inflow interval | `Qi*Ci = 0` with continuity reducing to effluent/deposition evolution only. | Consistent with Eq. [14.6.1] mass-balance form. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invalid States

- Missing any required pass-file field or particle-class vector required for
  downstream handoff. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative or undefined runon/runoff volumes, area, or stage/area terms in
  continuity equations. `[DIRECT][Static] + [INFERENCE][Static]`
- Channel duration computed without max-rule basis, or with mixed units across
  duration contributors. `[DIRECT][Static] + [INFERENCE][Static]`
- Peak-runoff merge branch inconsistent with contributor count (single-source
  vs multi-source synthetic hydrograph behavior). `[DIRECT][Static] + [INFERENCE][Static]`
- Outflow-regime transition processed without minimum timestep reset in
  impoundment integration. `[DIRECT][Static]`
- Sediment continuity evaluated without explicit upstream/lateral load terms or
  with unexplained continuity residual beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-SYSTEM-P-001: Publish complete pass-file payload with declared units and
  particle-class semantics before channel/impoundment consumption.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-002: Enforce runtime guards for all `INV-SYSTEM-*` hard-fail
  invariants prior to publishing downstream boundary outputs.
  `[INFERENCE][Static]`
- OBL-SYSTEM-P-003: Surface typed integration errors (missing payload,
  continuity failure, branch ambiguity) without silent defaulting/clamping.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-004: Persist method-branch identity and key closure terms in
  diagnostic payload for comparator/replay traceability.
  `[INFERENCE][Static]`
- OBL-SYSTEM-P-005: Comparator replay producers must emit command traces,
  strict comparator JSON artifacts, and reproducible tool/binary/output hashes
  for each required interchange replay surface.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-006: PL14S comparison-suite producers must emit semantic
  comparator artifacts with row-key presence deltas, per-column tolerance
  verdicts, top divergent keys, and explicit strict-comparator skip/execution
  provenance status; silent omission of these diagnostics is forbidden.
  `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-SYSTEM-C-001: Channel and impoundment consumers must reject malformed
  handoff payloads explicitly and propagate invariant IDs.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-C-002: Downstream routing/reporting consumers must preserve units
  and branch semantics for peak-flow and sediment payloads.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-C-003: Consumers must not reinterpret absent contributor payloads
  as zero unless the governing branch explicitly defines that behavior.
  `[INFERENCE][Static]`
- OBL-SYSTEM-C-004: Comparator/replay consumers must classify violations at
  tier-appropriate gates and preserve `HOLD` when governance guards remain open.
  `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Pass-file completeness and runon decomposition (`INV-SYSTEM-001/002/003`) | hillslope->channel/impoundment boundary ingest | Hard error with invariant ID and field-level diagnostics; block boundary publish | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Channel runoff/peak-flow branch semantics (`INV-SYSTEM-004/005/006`) | channel assembly and outlet-peak routines | Hard error on case-logic or method-branch mismatch | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Impoundment hydraulic/outflow continuity (`INV-SYSTEM-007/008`) | impoundment routing integration loop | Hard error on continuity/regime-transition/outflow-sum failure | Tier-B gate | `[DIRECT][Static]` |
| Sediment continuity (`INV-SYSTEM-009`) | channel and impoundment sediment routines | Hard error on unresolved continuity residual | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Breakpoint forcing governance (`INV-SYSTEM-010`) | review/verification/promotion cycle | Governance `HOLD` until companion forcing/route contracts close boundary semantics | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Coupled lane publication closure (`INV-SYSTEM-011`) | hillslope->watershed publish boundary | Hard error when publish is attempted after failed/invalid plant-water coupled lane closure | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Strict replay artifact/provenance completeness (`INV-SYSTEM-012`) | comparator staging + replay disposition boundary | Hard error / `HOLD` when required replay artifacts or provenance hashes are missing; no fallback artifact substitution | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL15 residual Tier-A governance closeout (`INV-SYSTEM-013`) | comparator disposition and PL08 hold-lift decision boundary | Governance `HOLD` unless unresolved Tier-A deltas are either closed or explicitly risk-accepted with approval reference; no silent down-classification | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL14R rerun reproducibility completeness (`INV-SYSTEM-014`) | comparator rerun staging + provenance publication boundary | Hard error / `HOLD` when required interchange surfaces, strict comparator JSON outputs, or binary/tool/output hashes are missing from rerun evidence | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL15R recloseout supersession governance (`INV-SYSTEM-015`) | comparator re-disposition and refreshed PL08 hold-lift decision boundary | Governance `HOLD` when active blocker classification ignores superseding schema-aligned strict replay evidence, or when post-supersession unresolved blockers lack explicit risk-acceptance reference | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB20 forward-solver parity lane governance (`INV-SYSTEM-016`) | parity-lane evidence publication and disposition boundary | Governance `HOLD` when WB20 lane-manifest/no-substitution evidence is absent or does not prove observed-target exclusion from acceptance-driving closure logic | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL14S semantic replay diagnostics completeness (`INV-SYSTEM-017`) | semantic comparator artifact publication and provenance boundary | Hard error / `HOLD` when semantic comparator report content or strict-skip provenance status is absent, malformed, or silently suppressed | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not bitwise
parity). Integration-boundary interpretation tolerances for review/comparator
surfaces are:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-SYSTEM-001 | Runon volume closure residual for Eq. [13.2.1] | `<= 1e-9 m^3` | Comparator interpretation bound; runtime still hard-fails on material mismatch. | `[INFERENCE][Static]` |
| TOL-SYSTEM-002 | Runon-depth conversion residual for Eq. [13.2.2] | `<= 1e-12 m` | Applied after unit-normalized conversion. | `[INFERENCE][Static]` |
| TOL-SYSTEM-003 | Outlet-flow continuity residual for Eq. [14.2.1]/[14.2.5] over one integration step | `<= 1e-8 ft^3` | Comparator-only tolerance for numerical integration noise. | `[INFERENCE][Static]` |
| TOL-SYSTEM-004 | Outflow-summation residual for Eq. [14.3.18] | `<= 1e-10 ft^3 s^-1` | Inactive-structure contribution remains exact zero by branch rule. | `[INFERENCE][Static]` |
| TOL-SYSTEM-005 | Sediment continuity residual for Eq. [13.5.17] and Eq. [14.6.1] | `<= 1e-8` in native mass units | Unit-aware residual check by component (`lb` vs `kg` surfaces). | `[INFERENCE][Static]` |
| TOL-SYSTEM-006 | Tier-A strict replay numeric tolerance for PL14 closeout lane | `abs_tol = 0`, `rel_tol = 0` | Comparator-lane authority is strict diff detection; residual disposition belongs to PL15. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-SYSTEM-007 | Tier-A semantic replay tolerance profile for PL14S investigation lane | `default abs=0.1`, `default rel=0.02`, with per-column overrides from `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json` | Investigation-grade semantic comparator tolerances are explicit evidence artifacts and do not replace strict-diff structural checks. | `[DIRECT][Static] + [INFERENCE][Static]` |

## WS11 Channel-Routing Physics-Equivalence Integration Addendum

### WS11 Integration Runtime Aliases

| Surface | Symbols |
|---|---|
| Channel runtime controls | `ws10_channel_{id}_chnn`, `ws10_channel_{id}_ctlslp`, `ws10_channel_{id}_chnk`, `ipeak` |
| Channel runtime outputs | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff` |
| Impoundment runtime controls | `ws10_impoundment_{id}_h`, `ws10_impoundment_{id}_hfull`, `ws10_impoundment_{id}_deltat`, `ws10_impoundment_{id}_qinf` |
| Impoundment runtime outputs | `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout`, `ws10_impoundment_{id}_hnext`, `ws10_impoundment_{id}_outflow_volume` |
| Hillslope contributor payloads | `hs{ID}_peakro`, `hs{ID}_watdur` |

### WS11 Integration Rules

1. Watershed-node execution order must honor topology dependencies and consume
   upstream WS11 payloads through explicit symbol references, not implicit
   fallback defaults.
2. WS11 channel integration authority must preserve explicit `ipeak` branch
   semantics (`1` Rational, `2` CREAMS, `3` kinematic wave, `>=4`
   Muskingum-Cunge) with deterministic branch identity.
3. WS11 routing authority must not collapse channel execution into the
   pre-WS11 gain-factor surrogate `(1 + ctlslp) / (1 + chnn)` or equivalent
   single-gain substitutions.
4. Missing/non-finite/out-of-domain WS11 boundary payloads are hard-fail system
   integration states and must propagate typed guard IDs unchanged.
5. System integration must preserve deterministic publish ordering and make
   WS11 dependency payload availability observable at the node boundary.
6. WS11 integration pathways must not silently clamp or synthesize replacement
   state/flux values to repair invalid boundary inputs.

### WS11 Guard Families

| Kernel lane | Guard IDs |
|---|---|
| Channel lane | `WKERNEL-WS10-CHANNEL-E-001..003` |
| Impoundment lane | `WKERNEL-WS10-IMPOUNDMENT-E-001..003` |

### WS11 Contract-Derived System Vectors

Minimum WS11 integration vectors:
1. Deterministic `channel -> impoundment -> downstream channel` topology run
   emits finite non-negative WS11 channel outputs at each node boundary.
2. `ipeak = 3` and `ipeak >= 4` vectors preserve routed closure semantics
   (`roff = qpo * durrof` within tolerance) on published channel payloads.
3. Missing required WS11 symbol halts node execution with `-E-001`.
4. Non-finite WS11 symbol halts node execution with `-E-002`.
5. Domain/dependency WS11 violation halts node execution with `-E-003`.

## WS12 Impoundment Physics-Equivalence Integration Addendum

### WS12 Integration Runtime Aliases and Coefficient Families

| Surface | Symbols |
|---|---|
| Impoundment runtime controls | `ws10_impoundment_{id}_h`, `ws10_impoundment_{id}_hfull`, `ws10_impoundment_{id}_deltat`, `ws10_impoundment_{id}_qinf` |
| Impoundment required parser-projected coefficient families | canonical impoundment coefficient/threshold surfaces (`a,b,c,d,e,ha,ht,hlm,a0,a1,a2,l0,l1,l2`) from `SC-INFILE-WATERSHED-IMPOUNDMENT-001` |
| Impoundment runtime outputs | `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout`, `ws10_impoundment_{id}_hnext`, `ws10_impoundment_{id}_outflow_volume` |
| Upstream dependency payloads | `hs{ID}_peakro`, `hs{ID}_watdur`, `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout` |

### WS12 Integration Rules

1. Impoundment-node integration authority is continuity + stage-discharge
   routing with adaptive-step retry/regime-transition handling; the WS10
   headroom surrogate is non-authoritative for WS12 parity claims.
2. Regime-boundary crossings must retry with initial timestep reset before
   publish; unresolved crossings are hard-fail integration states.
3. Outflow assembly must preserve structure-control minima and additive
   contribution semantics required by `SC-IMPOUND-001` WS12 authority.
4. Missing/non-finite/out-of-domain WS12 impoundment boundary payloads must
   propagate typed guard IDs unchanged (`WKERNEL-WS10-IMPOUNDMENT-E-001..003`).
5. WS12 integration pathways must not silently clamp/synthesize replacement
   state/flux values to repair invalid impoundment boundary inputs.

### WS12 Contract-Derived System Vectors

Minimum WS12 integration vectors:
1. Deterministic topology run (`channel -> impoundment -> downstream channel`)
   emits finite non-negative WS12 impoundment outputs and preserves dependency
   provenance.
2. Regime-transition crossing vector triggers retry/reset behavior prior to
   publication; invalid unresolved transition fails with `-E-003`.
3. Missing required WS12 impoundment symbol/coefficient payload halts node
   execution with `WKERNEL-WS10-IMPOUNDMENT-E-001`.
4. Non-finite WS12 impoundment symbol/intermediate halts node execution with
   `WKERNEL-WS10-IMPOUNDMENT-E-002`.
5. Domain/continuity violation halts node execution with
   `WKERNEL-WS10-IMPOUNDMENT-E-003`.

## ARCH22 Typed Production-Surface Addendum

### Typed Runtime Surface Authority

1. Covered production system-integration interfaces must resolve boundary-state
   and boundary-flux surfaces via ARCH22 typed symbol families:
   `HillslopeProductionStateSymbol`, `HillslopeProductionFluxSymbol`,
   `WatershedProductionStateSymbol`, and `WatershedProductionFluxSymbol`.
2. Covered production guard/accessor helper signatures must not accept raw
   `&str` symbol identifiers where typed ARCH22 symbols exist.
3. Typed migration must preserve deterministic publication ordering, dependency
   visibility, and existing hard-fail boundary classes/message IDs.

### Contract-Derived Migration Vectors

1. Static migration proof: covered production integration accessors use typed
   symbol families, not stringly `&str` parameters.
2. Nominal migration vector: coupled hillslope/watershed production execution
   preserves deterministic state/flux publication semantics.
3. Failure migration vectors: missing/non-finite/domain/dependency violations
   preserve existing typed hard-fail boundary classes and guard IDs.

## EROD12 Cross-Domain Ownership and Guard Closure Addendum

| Cross-domain lane | Producer ownership | Consumer guard ownership | Closure posture | Evidence |
|---|---|---|---|---|
| Hydrology forcing to erosion/routing (`Q`, `peakro`, `watdur`, `wb16_*`) | `SC-RUNOFFPART-001` + `SC-WATBAL-001` | `SC-SED-001`, `SC-HYDRAULICS-001`, `SC-ROUTE-001` | Required Wave-0 ownership/guard semantics are explicit in canonical companion contracts. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Hydraulics-to-erosion coupling (`fr`, `fi/fe`, `w`, `fs`, `ft`, `τf/τfe`) | `SC-HYDRAULICS-001` | `SC-SED-001` | Producer and consumer guard ownership is canonicalized for required boundaries. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Sediment export to routing (`sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i`) | `SC-SED-001` | `SC-ROUTE-001` | Routing consumer guard ownership for sediment handoff completeness is explicit. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SYSTEM-001 | Companion contracts for channel hydraulics/erosion (`SC-HYDRAULICS-001`, `SC-SED-001`), watershed routing (`SC-ROUTE-001`), and impoundment internals (`SC-IMPOUND-001`) remain in-review with open non-Wave-0 promotability gaps and staged production-kernel completion. | Full system-boundary promotion remains provisional for watershed-wide release, while EROD12 Wave-0 ownership/guard closure for required erosion boundaries is explicit. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SYSTEM-002 | Alias map remains identity-only because concrete openWEPP boundary field names for system payloads are not finalized. | Symbol continuity to implementation surfaces is incomplete. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SYSTEM-003 | Chapter 13 notes that separate climate files for hillslope and channel/impoundment components are possible but "not been tested" in cited text. | Cross-file forcing consistency risk remains for mixed-forcing configurations. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-SYSTEM-004 | CREAMS outlet peak-flow method is statistical and chapter-cited dataset support is for watersheds in the `70 ha` to `6200 ha` range. | Method-selection risk exists when applied outside referenced dataset conditions. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-17 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with integration invariants, guard map, alias map, tolerances, and gap register for SCI-17 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: normalized evidence metadata, added duration-family alias coverage, added evidence labels for degenerate/tolerance rows, and clarified CREAMS dataset applicability range in gap text. |
| `2026-05-23` | `3` | `Codex` | INT10 amendment: added cross-lane publication invariant (`INV-SYSTEM-011`) and guard/disposition authority requiring successful `decomp -> growth -> watbal` closure before system-boundary publication. |
| `2026-05-23` | `4` | `Codex` | PL14 amendment: added strict replay artifact/provenance completeness invariant (`INV-SYSTEM-012`), replay-lane guard/disposition authority, and explicit strict Tier-A tolerance authority (`abs_tol=0`, `rel_tol=0`) for closeout staging. |
| `2026-05-23` | `5` | `Codex` | PL15 amendment: added Tier-A residual closeout governance invariant (`INV-SYSTEM-013`) requiring explicit risk-acceptance references for unresolved blockers and prohibiting silent down-classification/implicit risk acceptance. |
| `2026-05-23` | `6` | `Codex` | WS10 amendment: added system integration authority for WS10 production watershed runtime aliases, deterministic dependency payload publication rules, and WS10 typed guard-family/test-vector requirements. |
| `2026-05-23` | `7` | `Codex` | ARCH22 amendment: added typed production-surface authority requiring covered system integration interfaces to consume boundary symbols via ARCH22 typed symbol families while preserving deterministic publication and existing failure-class/message continuity. |
| `2026-05-23` | `8` | `Codex` | PL14R amendment: added strict replay rerun reproducibility invariant (`INV-SYSTEM-014`) requiring required include-surface staging plus persisted comparator/provenance hash evidence for post-closure-wave Tier-A rerun authority. |
| `2026-05-23` | `9` | `Codex` | PL15R amendment: added refreshed Tier-A recloseout supersession invariant (`INV-SYSTEM-015`) requiring blocker classification from latest schema-aligned strict replay evidence and explicit risk-acceptance reference only when post-supersession blockers remain. |
| `2026-05-23` | `10` | `Codex` | WB20 amendment: added forward-solver parity governance invariant (`INV-SYSTEM-016`) requiring explicit lane-manifest and no-observed-target-substitution evidence before Tier-A parity-lane disposition can close. |
| `2026-05-23` | `11` | `Codex` | EROD12 amendment: added cross-domain ownership/guard closure addendum covering required erosion-lane Wave-0 boundaries and refined `GAP-SYSTEM-001` to distinguish remaining non-Wave-0 system promotability holds from resolved Wave-0 ownership semantics. |
| `2026-05-24` | `12` | `Codex` | CLI02 amendment: replaced required strict-replay include-surface authority from legacy candidate files to simulation-driven partitioned interchange surfaces (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`) and aligned supersession evidence wording to parquet comparator artifacts. |
| `2026-05-24` | `13` | `Codex` | WS12 amendment: added impoundment physics-equivalence integration authority requiring continuity/regime-driven impoundment routing with parser-projected coefficient families and preserved WS10 guard-family continuity for boundary failures. |
| `2026-05-24` | `14` | `Codex` | WS11 amendment: added channel-routing physics-equivalence integration authority requiring explicit `ipeak` branch execution (Rational/CREAMS/KW/MC), prohibited pre-WS11 gain-factor surrogate substitution, and anchored integration semantics to pinned legacy routing provenance while preserving existing WS10 guard-family continuity. |
| `2026-05-24` | `15` | `Codex` | PL14S amendment: added semantic replay diagnostics invariant (`INV-SYSTEM-017`), semantic/provenance publication guard authority, semantic replay producer obligations, and explicit semantic-tolerance profile authority for investigation-grade Tier-A replay evidence. |
