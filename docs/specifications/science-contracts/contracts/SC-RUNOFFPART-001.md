---
contract_id: SC-RUNOFFPART-001
title: Surface Runoff Partition Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 2
producer_scope:
  - Event-scale infiltration accounting and rainfall-excess partition surfaces
  - Depression-storage satisfaction/release and runoff onset transition surfaces
  - OFE-to-OFE runoff/runon and outlet runoff-volume/peak-rate aggregation surfaces
consumer_scope:
  - Daily water-balance consumers requiring surface-runoff depth (`Q`)
  - Erosion/hydraulics consumers requiring runoff duration, volume, and peak discharge
  - Comparator/replay surfaces using Tier-A single-OFE runoff acceptance signals
evidence_level: static
last_reviewed: 2026-05-20
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

## Symbol Alias Map

Canonical symbols in this contract use Chapter-4 WEPP notation. Concrete
openWEPP runtime-field names are not fixed yet, so identity aliases are
required until implementation surfaces diverge.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `R`, `Ri`, `r`, `ri` | identity names | climate-to-runoff forcing surfaces | `m` / `m s^-1` preserved | `[DIRECT][Static]` |
| `F`, `Fi`, `fi` | identity names | infiltration and partition surfaces | `m` / `m s^-1` preserved | `[DIRECT][Static]` |
| `V`, `Vi`, `vi` | identity names | rainfall-excess/routing surfaces | `m` / `m s^-1` preserved | `[DIRECT][Static]` |
| `Ke`, `Ψ`, `θd`, `Sp` | identity names | infiltration branch parameter/state surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Sd`, `rr`, `So` | identity names | depression-storage surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Qv`, `qp` | identity names | routed-runoff and peak-runoff outputs | `m` and `m^2 s^-1` preserved | `[DIRECT][Static]` |
| `De` | identity name | effective-runoff-duration coupling surface | `s` preserved | `[DIRECT][Static]` |
| `Qj-1`, `Vj`, `Qj` | identity names | multi-OFE case-classification surfaces | `m` preserved | `[DIRECT][Static]` |
| `Ka`, `(Ψθd)a`, `Sa`, `Fp`, `Fh` | identity names | multi-OFE averaging and runon/runoff branch surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `α`, `αe`, `m` | identity names | routing/equivalent-plane coefficient surfaces | coefficient/exponent semantics preserved | `[DIRECT][Static]` |
| `Q` | identity name | runoff-depth handoff to daily water-balance domain | `m` preserved | `[DIRECT][Static]` |

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

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-RUNOFFPART-001 | Full per-invariant comparator vectors for multi-OFE cases are not yet curated in this package. | Limits immediate automation depth for `INV-RUNOFFPART-007/008` acceptance checks. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-RUNOFFPART-002 | Concrete openWEPP runtime-field aliases are not yet fixed for runoff partition outputs and internal branch states. | Alias map remains identity-only pending boundary finalization. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-RUNOFFPART-003 | Chapter-4 limitations explicitly note Hortonian-flow framing and reduced recession interaction outside partial-equilibrium correction; companion contracts for variable-source-area/return-flow behavior are not authored. | Scope caveat must remain explicit to avoid over-claiming runoff applicability. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-RUNOFFPART-004 | Coupled contracts `SC-EVAP-001`, `SC-PERC-001`, `SC-SUBHYD-001`, and `SC-SED-001` are not fully authored, so cross-domain ownership boundaries remain provisional. | Promotion-readiness depends on downstream contract completion/consistency. | non-promotable | `[DIRECT][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-06 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-4 authority anchors, invariants, guard map, alias map, obligations, boundary dispositions, tolerances, and gap register. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added explicit event-closure term identity, added normative multi-OFE case outcome table, added alias row for `De`, and split rate tolerances for clearer unit-specific governance. |
