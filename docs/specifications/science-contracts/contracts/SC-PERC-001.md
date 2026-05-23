---
contract_id: SC-PERC-001
title: Percolation Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 3
producer_scope:
  - Layer-by-layer percolation flux surfaces from root-zone water storage states
  - Below-root-zone percolation-loss accounting surfaces used by daily closure
  - Percolation coupling surfaces consumed by subsurface/lateral-flow and drainage routines
consumer_scope:
  - Daily water-balance accounting consumers
  - Subsurface/drainage consumers that ingest percolation recharge terms
  - Comparator/replay surfaces using Tier-A daily closure confidence signals
evidence_level: Static
last_reviewed: 2026-05-23
supersedes: []
superseded_by: []
---

# SC-PERC-001 Percolation Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for root-zone percolation behavior,
per-layer drainage constraints, below-root loss accounting semantics, and
cross-domain coupling boundaries required by openWEPP daily hydrology.

## Scientific Scope

In scope:
- Layer-wise percolation eligibility and flux calculation semantics from
  Chapter 5 percolation equations.
- Adjusted-conductivity and lower-layer restriction behavior that modulates
  percolation routing.
- Below-root percolation-loss semantics in daily water-balance accounting.
- Coupling boundaries between percolation outputs and subsurface/drainage
  consumers.

Out of scope:
- Kernel implementation details and Rust API naming.
- Surface runoff partition and depression-storage internals owned by
  `SC-RUNOFFPART-001`.
- Full subsurface lateral-flow and tile/ditch drainage physics owned by
  `SC-SUBHYD-001`.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-PERC-CH5-BAL | `references/50201000/chap5.pdf` §5.1 Eq. [5.1.1] | Declares daily water-balance closure with cumulative percolation-loss term `D` below root zone. | `[DIRECT][Static]` |
| REF-PERC-CH5-PERC | `chap5.pdf` §5.4 Eq. [5.4.1]-[5.4.5] | Core percolation equations: field-capacity eligibility, travel time, adjusted conductivity, and lower-layer saturation restriction. | `[DIRECT][Static]` |
| REF-PERC-CH5-LINK | `chap5.pdf` §5.5 text + Fig. 5.2.1 | Defines infiltration/water-balance/percolation linkage and states that percolation below root zone is considered lost in WEPP water balance. | `[DIRECT][Static]` |
| REF-PERC-CH6-CONT | `references/50201000/chap6.pdf` §6.2.1-§6.2.2 Eq. [6.2.1]-[6.2.5] | Subsurface continuity uses daily percolated water `Pe` into drainable layer; percolation is the recharge source for subsurface flow routines. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-PERC-CH6-DRAIN | `chap6.pdf` §6.2.3 Eq. [6.2.10]-[6.2.11] | Drainage/tile-flow routines consume subsurface state influenced by percolation recharge; sets downstream coupling context. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-PERC-CH7-PARAM | `references/50201000/chap7.pdf` §7.8 Eq. [7.8.3]-[7.8.5] | Coarse-fragment and entrapped-air adjustments alter effective porosity/soil-water state surfaces that propagate into Chapter-5 routing terms. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-PERC-CH7-FROZEN | `chap7.pdf` §7.9.7 Eq. [7.9.20]-[7.9.22] | Frozen-soil conductivity adjustment modifies conductivity used by infiltration/percolation calculations. | `[DIRECT][Static]` |
| REF-PERC-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative fluxes, finite conductivity/travel-time domains, and bounded storage fractions for physical plausibility. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `Θi` | `m` | Water content in soil layer `i`. | soil/water-balance state | percolation eligibility/routing |
| `FCi` | `m` | Field-capacity water content of layer `i` (33 KPa convention for many soils). | soil parameterization | percolation eligibility Eq. [5.4.1] |
| `ULi` | `m` | Upper-limit water content of layer `i`. | soil parameterization | conductivity adjustment and lower-layer restriction |
| `pei` | `m d^-1` | Percolation rate through layer `i`. | percolation routine | lower-layer routing and `D`/`Pe` assembly |
| `Δt` | `s` | Percolation travel interval. | percolation timestep control | Eq. [5.4.1] exponential response |
| `ti` | `s` | Travel time through layer `i`. | percolation routine | Eq. [5.4.1] and Eq. [5.4.2] linkage |
| `Ksi` | `m s^-1` | Saturated hydraulic conductivity for layer `i`. | soil hydraulic parameterization | adjusted conductivity computation |
| `Ksai` | `m s^-1` | Adjusted hydraulic conductivity for layer `i`. | percolation routine | Eq. [5.4.2]-[5.4.3] routing |
| `Bi` | `fraction` | Conductivity-shape parameter controlling approach of `Ksai` toward near-zero at field capacity. | percolation routine | Eq. [5.4.3]-[5.4.4] |
| `Θi+1`, `ULi+1` | `m`, `m` | Lower-layer water-content state and upper limit used for percolation restriction term. | lower-layer state/parameters | Eq. [5.4.5] reduction factor |
| `D` | `m` | Cumulative percolation loss below root zone in daily water balance. | percolation-water-balance coupling | daily closure Eq. [5.1.1] |
| `Pe` | `m d^-1` | Percolated water into subsurface drainable layer. | percolation routine | subsurface continuity Eq. [6.2.1], [6.2.5] |
| `θ`, `θFC`, `θa` | `m^3 m^-3` | Total moisture, field-capacity moisture, and entrapped air defining drainable-water term in subsurface coupling. | subsurface state routine | drainable-layer storage accounting |

## Algorithm State Surfaces (WB10 Hydrology Phase-Entry Scaffolding)

### Required Inputs

| Surface | Symbols |
|---|---|
| Scheduler phase metadata | `phase_name`, `phase_class`, `consumer_adapter` |
| Percolation consumer-boundary state family | `nsl`, `thetdr`, `thetfc`, `ssc` |

### Required Outputs

| Surface | Output |
|---|---|
| Hydrology phase entry route | Deterministic route label for `percolation_deep_seepage` phase class |
| Scheduler failure surface | Typed hard-fail status for unsupported/mismatched percolation phase-class routing |

### Mutated State Surfaces

WB10 routing scaffolding mutates only typed entry-route metadata and typed
failure reporting; percolation state/flux updates remain delegated to downstream
kernels.

## Algorithm Specification (WB10 Percolation Routing Skeleton)

1. Scheduler maps `percolation_deep_seepage` to WB10 typed percolation phase
   class.
2. Hydrology routing validates phase/class compatibility before kernel
   invocation.
3. Unsupported or mismatched percolation class combinations are invalid runtime
   states and hard-fail with typed status.
4. Valid percolation routing preserves orchestrator-owned writeback surfaces
   and forwards immutable phase metadata to kernel execution.

## Branch and Guard Table (WB10 Percolation Phase Class)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-PERC-WB10-PERC` | phase `percolation_deep_seepage` | scheduler phase + WB10 phase class metadata | runtime | typed hard-fail on unsupported/mismatched routing class |
| `BR-PERC-WB10-UNSUPPORTED` | any unsupported percolation routing class state | scheduler phase + phase class metadata | runtime | typed hard-fail (`HS-HYDRO-E-001`) and scheduler halt |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-PERC-001 | Field-capacity eligibility invariant: per layer, Eq. [5.4.1] branch semantics are explicit (`pei = 0` when `Θi <= FCi`; routing expression only when `Θi > FCi`). | hard-fail | REF-PERC-CH5-PERC | `[DIRECT][Static]` |
| INV-PERC-002 | Per-layer excess-water bound invariant: when `Θi > FCi`, emitted `pei` must be non-negative and not exceed the available excess-water term implied by Eq. [5.4.1]. | hard-fail | REF-PERC-CH5-PERC, REF-PERC-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-003 | Travel-time/conductivity-domain invariant: Eq. [5.4.2] and Eq. [5.4.3] usage requires finite positive routing domains (no undefined `ti`, `Ksai`, or layer-moisture terms in active percolation branch). | hard-fail | REF-PERC-CH5-PERC, REF-PERC-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-004 | Adjusted-conductivity formulation invariant: adjusted conductivity follows Eq. [5.4.3]-[5.4.4] semantics and preserves near-field-capacity damping behavior, with Chapter-7 state/condition adjustments applied explicitly where active. | hard-fail | REF-PERC-CH5-PERC, REF-PERC-CH7-PARAM, REF-PERC-CH7-FROZEN | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-005 | Lower-layer restriction invariant: when Eq. [5.4.5] is applied, lower-layer saturation reduction must remain in real-number domain and cannot amplify `pei` above its pre-restriction value. Comparator interpretation may classify near-zero negative radicand jitter as threshold-adjacent, but runtime behavior remains explicit hard-fail for negative-domain evaluations. | hard-fail | REF-PERC-CH5-PERC, REF-PERC-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-006 | Below-root loss accounting invariant: percolation routed below the root zone is treated as loss in Chapter-5 daily closure (`D`) and cannot be silently recycled into root-zone storage within this contract boundary. | hard-fail | REF-PERC-CH5-BAL, REF-PERC-CH5-LINK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-007 | Subsurface coupling invariant: daily percolation recharge term `Pe` used by subsurface continuity equations is emitted with unit/sign consistency and complete boundary payload semantics. | hard-fail | REF-PERC-CH6-CONT, REF-PERC-CH6-DRAIN | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-008 | Coupled root-zone update invariant: percolation processing remains explicitly coupled with infiltration/ET daily accounting paths described in §5.5 and does not permit silent omission of percolation updates from layer-water bookkeeping. | hard-fail | REF-PERC-CH5-LINK, REF-PERC-CH5-BAL | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-009 | Governance scope invariant: claims about subsurface lateral-flow/drainage mechanics beyond declared percolation boundary are non-promotable unless backed by `SC-SUBHYD-001` authority. | governance-fail | REF-PERC-CH6-CONT, REF-PERC-CH6-DRAIN | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-010 | WB10 percolation routing invariant: scheduler `percolation_deep_seepage` phase must route through explicit WB10 percolation phase class with no silent generic fallback. | hard-fail | REF-PERC-CH5-BAL, REF-PERC-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-011 | Unsupported routing invariant: unsupported or mismatched percolation phase-class routing states must surface typed hard failures and cannot be silently reassigned. | hard-fail | REF-PERC-PHYS-BOUNDS | `[INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-PERC-001` | runtime | Layer percolation branch selector (Eq. [5.4.1]) | Typed hard error on implicit/mismatched branch behavior | Tier-A gate | `[DIRECT][Static]` |
| `INV-PERC-002` | runtime | Excess-water bounds check on per-layer `pei` | Typed hard error on negative or excess-over-bound percolation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-003` | runtime | Active-branch routing-domain validator for `ti`/`Ksai`/state terms | Typed hard error on undefined/non-finite domain terms | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-004` | runtime | Adjusted-conductivity evaluator for Eq. [5.4.3]-[5.4.4] and active Chapter-7 condition modifiers | Typed hard error on conductivity-domain violation or damping-semantics mismatch | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-005` | runtime | Lower-layer restriction evaluator (Eq. [5.4.5]) | Typed hard error on invalid restriction domain or amplification above pre-restriction flux; no runtime clamping/defaulting | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-006` | runtime | Daily closure assembler for below-root loss term `D` | Typed hard error on inconsistent loss accounting at boundary publish | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-007` | runtime | Percolation-to-subsurface boundary payload validator (`Pe`) | Typed hard error on missing malformed units/sign payload | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-008` | runtime | Layer-water bookkeeping integration checks with infiltration/ET update path | Typed hard error on omitted percolation update in daily coupled accounting | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-009` | governance | Contract review/disposition/promotion checklist | Promotion `HOLD` when subsurface mechanics claims exceed declared contract boundary | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-010` | runtime | WB10 percolation phase-class routing table | Typed hard error on unsupported/mismatched percolation routing class | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-011` | runtime | Unsupported percolation routing guard | Typed hard error (`HS-HYDRO-E-001`) on unsupported percolation routing class combinations | Tier-A gate | `[INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols follow Chapter-5/Chapter-6 WEPP notation. Concrete openWEPP
runtime-field names are not fixed for this domain, so identity aliases are
required until implementation surfaces diverge.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `Θi`, `FCi`, `ULi` | identity names | layer-state/percolation eligibility surfaces | `m` preserved | `[DIRECT][Static]` |
| `pei`, `Pe`, `D` | identity names | percolation output and boundary-loss/recharge surfaces | `m d^-1` / `m` preserved | `[DIRECT][Static]` |
| `Δt`, `ti` | identity names | percolation routing-time surfaces | `s` preserved | `[DIRECT][Static]` |
| `Ksi`, `Ksai`, `Bi` | identity names | percolation conductivity parameter/state surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Θi+1`, `ULi+1` | identity names | lower-layer restriction surfaces | `m` preserved | `[DIRECT][Static]` |
| `θ`, `θFC`, `θa` | identity names | subsurface drainable-state coupling surfaces | `m^3 m^-3` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| Field-capacity-or-drier layer | `Θi <= FCi` and per-layer `pei = 0`. | Explicit Eq. [5.4.1] branch behavior. `[DIRECT][Static]` |
| Near-field-capacity slow drainage | `Θi` only slightly above `FCi`, producing very small positive `pei`. | Consistent with storage-routing response as excess approaches zero. `[DIRECT][Static] + [INFERENCE][Static]` |
| Lower-layer near saturation throttling | `Θi+1` near `ULi+1`, causing Eq. [5.4.5] restriction factor to approach zero. | Explicit lower-layer restriction behavior. `[DIRECT][Static]` |
| Frozen/restrictive attenuation regime | Effective conductivity is strongly reduced by frozen/restrictive conditions but remains in valid domain. | Chapter-5 and Chapter-7 conductivity-adjustment semantics. `[DIRECT][Static] + [INFERENCE][Static]` |
| No below-root export day | Per-layer routing occurs within root zone but aggregate below-root `D` is zero for the step. | Valid daily state when no percolation crosses the root-zone boundary. `[INFERENCE][Static]` |

## Invalid States

- Active percolation branch (`Θi > FCi`) with undefined/non-finite travel-time or conductivity terms. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative percolation flux or per-layer percolation exceeding available excess-water bound beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Eq. [5.4.5] restriction computed outside real domain (invalid lower-layer ratio) or increasing `pei`. `[DIRECT][Static] + [INFERENCE][Static]`
- Below-root percolation exported without consistent daily loss accounting term `D`. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing/malformed `Pe` payload for subsurface continuity consumers. `[DIRECT][Static] + [INFERENCE][Static]`
- Silent omission of percolation-layer updates in coupled daily infiltration/ET/water-balance bookkeeping path. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-PERC-P-001: Emit per-layer percolation and coupling surfaces (`pei`, `Pe`, `D`) with canonical symbols and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PERC-P-002: Enforce explicit Eq. [5.4.1]-[5.4.5] branch logic and guard domains; no implicit fallback branches. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PERC-P-003: Propagate invariant failures as typed errors; no silent clamping/defaulting of percolation terms. `[INFERENCE][Static]`
- OBL-PERC-P-004: Preserve boundary-ready loss/recharge semantics for daily closure (`D`) and subsurface coupling (`Pe`). `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-PERC-C-001: Water-balance consumers must ingest below-root percolation-loss term `D` with Chapter-5 sign/units semantics unchanged. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PERC-C-002: Subsurface consumers must ingest `Pe` consistently with Chapter-6 continuity assumptions and reject malformed payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PERC-C-003: Coupled hydrology consumers must preserve layer-state continuity required by Eq. [5.4.*] routing assumptions. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PERC-C-004: All consumers must fail explicitly on invariant-violating payloads and carry invariant IDs in error context. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Eq. [5.4.1] branch and per-layer flux bounds (`INV-PERC-001/002`) | per-layer percolation evaluation | Hard error; reject step output on branch/bounds failure | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Conductivity/travel-time and lower-layer restriction domains (`INV-PERC-003/004/005`) | routing and restriction post-processing | Hard error on invalid domains or non-physical amplification | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Below-root loss closure and daily coupling consistency (`INV-PERC-006/008`) | daily closure assembly | Hard error on inconsistent accounting/bookkeeping | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Subsurface boundary payload completeness (`INV-PERC-007`) | percolation-to-subsurface handoff | Hard error on missing malformed boundary field | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Scope/governance boundary (`INV-PERC-009`) | review/verification/promotion | Governance `HOLD` until subsurface-boundary claims are contract-aligned | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB10 percolation phase-class routing (`INV-PERC-010/011`) | scheduler percolation entry dispatch | Hard error on unsupported/mismatched percolation routing class | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Constants and Parameters Table

| Constant/parameter | Units | Domain | Contract use | Authority |
|---|---|---|---|---|
| `WB10_PHASE_CLASS_PERC` | class label | exact match | Required class label for `percolation_deep_seepage` routing | REF-PERC-CH5-BAL |
| `WB10_UNSUPPORTED_ROUTING_CODE` | status message id | `HS-HYDRO-E-001` | Typed failure code for unsupported WB10 percolation routing states | REF-PERC-PHYS-BOUNDS |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). Contract-specific tolerances:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-PERC-001 | Eq. [5.4.1] per-layer branch residual tolerance | `<= 1e-9 m d^-1` | Residual is evaluated as implemented branch output minus declared branch expression. | `[INFERENCE][Static]` |
| TOL-PERC-002 | Non-negative comparator tolerance for percolation/loss terms (`pei`, `Pe`, `D`) | lower bound `>= -1e-12` in declared units | Comparator-noise allowance only; runtime still hard-fails on material negatives. | `[INFERENCE][Static]` |
| TOL-PERC-003 | Lower-layer restriction radicand comparator-classification tolerance for Eq. [5.4.5] | `1 - (Θi+1 / ULi+1) >= -1e-12` | Comparator interpretation only; runtime guard still hard-fails on negative-domain evaluations and does not clamp/default. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-PERC-004 | Field-capacity threshold proximity for Eq. [5.4.1] branch split | `abs(Θi - FCi) <= 1e-12 m` treated as threshold-adjacent | Prevents jitter-driven branch toggling near field-capacity boundary in comparator interpretation. | `[INFERENCE][Static]` |

## Test-Vector Obligations

Minimum WB10 scheduler percolation phase-entry conformance vectors:

1. `percolation_deep_seepage` routes with explicit WB10 percolation phase class
   and succeeds through kernel invocation when required symbols are present.
2. Mismatched/unsupported percolation phase-class combinations hard-fail with
   typed scheduler status (`HS-HYDRO-E-001`).
3. No silent fallback from typed percolation phase class to generic hydrology
   class is permitted.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-PERC-001 | Per-invariant comparator vectors for per-layer percolation and lower-layer restriction behavior are not yet curated in this package. | Limits immediate automation depth for invariant-specific acceptance checks. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-PERC-002 | Concrete openWEPP runtime-field aliases for percolation outputs and intermediate routing states are not yet fixed. | Alias map remains identity-only pending boundary finalization. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-PERC-003 | Companion contract `SC-SUBHYD-001` is not yet fully authored, so cross-domain ownership boundaries for subsurface routing remain provisional. | Promotion-readiness depends on downstream contract completion/consistency. | non-promotable | `[DIRECT][Static]` |
| GAP-PERC-004 | Chapter-5 validation evidence is reported at aggregate water-balance behavior; dedicated per-layer percolation validation vectors are not explicitly separated in cited material. | Per-layer percolation confidence is lower than aggregate daily closure confidence until dedicated evidence is added. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Promotion Readiness

This revision remains intentionally non-promotable and stays in lifecycle state
`in_review` while `GAP-PERC-002` and `GAP-PERC-003` remain open. Governance
guard `INV-PERC-009` requires explicit `HOLD` until alias finalization and
`SC-SUBHYD-001` authority closure are completed.

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-08 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-5/6 authority anchors, invariants, guard map, alias map, obligations, boundary disposition, tolerances, and gap register for SCI-08 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added direct Chapter-7 anchors for conductivity modifiers, normalized evidence-mode tokens, clarified lower-layer restriction tolerance vs runtime hard-fail semantics, added evidence tags to degenerate-state/tolerance rows, and made non-promotable `HOLD` state explicit. |
| `2026-05-23` | `3` | `Codex` | WB10 amendment: added explicit percolation phase-entry routing authority, unsupported-class typed hard-fail posture, and WB10 percolation test-vector obligations. |
