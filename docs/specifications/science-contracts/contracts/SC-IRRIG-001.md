---
contract_id: SC-IRRIG-001
title: Irrigation Event Coupling Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 2
producer_scope:
  - Sprinkler and furrow irrigation event-definition/scheduling surfaces
  - Rainfall-irrigation concurrency handling and furrow hydraulic state surfaces
  - Irrigation-domain outputs required by runoff, erosion, and daily closure consumers
consumer_scope:
  - Runoff partition, hydraulics, and erosion consumers using irrigation-forced runoff surfaces
  - Daily water-balance and soil-water consumers requiring irrigation-addition accounting
  - Comparator/replay surfaces using irrigation event/state confidence signals
evidence_level: Static
last_reviewed: 2026-05-20
supersedes: []
superseded_by: []
---

# SC-IRRIG-001 Irrigation Event Coupling Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for irrigation event coupling, including
sprinkler rainfall-concurrency behavior, furrow hydrology/hydraulics event
phases, and scheduling-state transitions that control when irrigation is
applied and how irrigation additions are handed off to coupled hydrology and
erosion domains. `[INFERENCE][Static]`

## Scientific Scope

In scope:
- Sprinkler-system event semantics, including concurrent rainfall+irrigation
  hydrograph construction. `[DIRECT][Static] + [INFERENCE][Static]`
- Furrow-event hydrology/hydraulics definitions needed to publish irrigation
  runoff/erosion-forcing surfaces. `[DIRECT][Static] + [INFERENCE][Static]`
- Irrigation scheduling alternatives and transition rules (`no-irrigation`,
  depletion-level, fixed-date, combined scheduling). `[DIRECT][Static]`
- Coupling obligations to runoff, water-balance, and erosion consumers.
  `[DIRECT][Static] + [INFERENCE][Static]`

Out of scope:
- Kernel implementation details and Rust API naming. `[INFERENCE][Static]`
- Non-irrigation runoff/infiltration physics owned by `SC-RUNOFFPART-001`.
  `[INFERENCE][Static]`
- Sediment transport physics beyond irrigation-driven hydrologic boundary
  obligations owned by `SC-SED-001`. `[INFERENCE][Static]`
- Watershed channel/impoundment routing internals. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-IRRIG-CH12-INTRO | `references/50201000/chap12.pdf` §12.1 | Declares irrigation component scope and event attribution for runoff/soil-loss accounting. | `[DIRECT][Static]` |
| REF-IRRIG-CH12-SPRINKLER | `references/50201000/chap12.pdf` §12.2 | Stationary sprinkler assumptions and OFE targeting behavior; sprinkler additions share rainfall-addition computational pathway. | `[DIRECT][Static]` |
| REF-IRRIG-CH12-CONCURRENT-SPRINKLER | `references/50201000/chap12.pdf` §12.2.1 | Concurrent rainfall+sprinkler hydrograph merge rules for `D_irr <= D_rain` and `D_irr > D_rain` cases with 10-block output hydrographs. | `[DIRECT][Static]` |
| REF-IRRIG-CH12-FURROW-HYDROLOGY | `references/50201000/chap12.pdf` §12.3.1.1, Eq. [12.3.1]-[12.3.5] | Furrow infiltration formulation, Green-Ampt-derived parameterization, and wetting-front cap semantics. | `[DIRECT][Static]` |
| REF-IRRIG-CH12-FURROW-HYDRAULICS | `references/50201000/chap12.pdf` §12.3.1.2, Eq. [12.3.6]-[12.3.14] | Furrow continuity/kinematic-wave formulation and advance/continuing/depletion/recession phase handling. | `[DIRECT][Static]` |
| REF-IRRIG-CH12-FURROW-EROSION-LINK | `references/50201000/chap12.pdf` §12.3.1.3 | Irrigation erosion path consumes peak runoff and effective runoff duration from furrow hydrology/hydraulics outputs. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-IRRIG-CH12-INFLOW | `references/50201000/chap12.pdf` §12.3.1.4 | Constant, cut-back, and surge inflow-management options and required branch semantics. | `[DIRECT][Static]` |
| REF-IRRIG-CH12-CONCURRENT-FURROW | `references/50201000/chap12.pdf` §12.3.2 | Furrow irrigation rainfall concurrency restriction and low-rainfall exception (`depth < 0.001 m` plus low peak-intensity condition). | `[DIRECT][Static]` |
| REF-IRRIG-CH12-SCHED-DEPLETION | `references/50201000/chap12.pdf` §12.4.2 | Depletion-level trigger semantics, irrigation-period gating, and single-OFE limitation under depletion scheduling. | `[DIRECT][Static]` |
| REF-IRRIG-CH12-SCHED-FIXED | `references/50201000/chap12.pdf` §12.4.3 | Fixed-date scheduling trigger semantics and no-irrigation fallback when fixed-date events are exhausted. | `[DIRECT][Static]` |
| REF-IRRIG-CH12-SCHED-COMBINATION | `references/50201000/chap12.pdf` §12.4.4 | Combined fixed-date/depletion scheduling mode transitions and priority ordering. | `[DIRECT][Static]` |
| REF-IRRIG-CH2-DISAG-LINK | `references/50201000/chap2.pdf` §2.2 | Rainfall block disaggregation conventions used before sprinkler-concurrency intensity augmentation. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-IRRIG-CH11-COUPLING | `references/50201000/chap11.pdf` §11.2.2 Eq. [11.2.5] + `chap12.pdf` §12.3.1.3 | Erosion-consumer coupling requires hydrologic outputs (peak runoff and effective runoff duration). | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-IRRIG-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative irrigation additions and finite hydraulic states are required for valid process coupling. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `D_irr` | `s` | Irrigation event duration. | irrigation scheduler | concurrency and hydraulic branches |
| `D_rain` | `s` | Rainfall-event duration on irrigation day. | climate/rainfall forcing | concurrency branch selector |
| `I_irr` | `m s^-1` | Sprinkler irrigation application rate for concurrency merge. | sprinkler event builder | rainfall-block intensity augmentation |
| `i_k` | `m s^-1` | Rainfall block intensity before/after sprinkler augmentation for block `k`. | disaggregation + irrigation concurrency logic | runoff partition/hydrograph consumers |
| `N_blk` | `count` | Final disaggregated hydrograph block count (10 by rule in §12.2.1). | concurrency assembler | runoff partition consumer |
| `Z` | `m^2` | Cumulative infiltrated volume per unit furrow length. | furrow hydrology component | furrow hydraulics and scheduling |
| `Ix`, `Iy` | `m` | Horizontal and vertical wetting-front advance distances. | furrow hydrology component | infiltration parameterization and bounds checks |
| `(Ix)_max` | `m` | Maximum horizontal wetting-front advance distance `(W-b)/2`. | furrow hydrology component | wetting-front cap branch |
| `Ke` | `m s^-1` | Effective hydraulic conductivity in furrow infiltration formulation. | soil/hydrology parameterization | furrow hydrology component |
| `k`, `a`, `fo` | `m^(2)/s^a`, `fraction`, `m^2 s^-1` | Kostiakov-Lewis infiltration parameters. | furrow hydrology calibration branch | furrow infiltration update |
| `Q` | `m^3 s^-1` | Furrow flow rate. | furrow hydraulics component | erosion-coupling hydrologic output |
| `A` | `m^2` | Furrow cross-sectional flow area. | furrow hydraulics component | continuity equation solver |
| `So`, `Sf` | `m m^-1` | Furrow slope and friction slope in kinematic-wave assumption. | topography + hydraulics component | furrow hydraulics solver |
| `alpha`, `m` | coefficient, exponent | Power-function coefficients/exponent in `Q = alpha * A^m`. | furrow hydraulics component | numerical flow-area solve |
| `dt`, `dx` | `s`, `m` | Time and distance increments for furrow hydraulic computation grid. | furrow hydraulics solver | phase update logic |
| `DL` | `fraction` | Depletion level ratio (current/max available soil water). | depletion scheduler | irrigation-trigger decision |
| `DL_crit` | `fraction` | User-specified critical depletion threshold. | management input surface | depletion scheduler |
| `I_req` | `m` | Irrigation requirement to fill profile to field capacity in active scheduling branch. | depletion scheduler | sprinkler/furrow depth computation |
| `I_min`, `I_max` | `m` | User-specified minimum and maximum sprinkler irrigation depths. | management input surface | sprinkler depth limiter |
| `p_req` | `fraction` | Percent of irrigation requirement applied under sprinkler mode. | management input surface | sprinkler depth computation |
| `qp` | `m^3 s^-1` | Peak runoff rate emitted for furrow irrigation erosion coupling (legacy label `Qp` preserved as alias). | furrow hydrology/hydraulics output | erosion component |
| `De` | `s` | Effective runoff duration (`runoff volume / peak runoff rate`) for erosion coupling. | furrow hydrology/hydraulics output | erosion component |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-IRRIG-001 | Irrigation-addition domain invariant: emitted irrigation depths/rates/durations must be finite and non-negative, and irrigation mode must be explicit (`none`, `sprinkler`, `furrow`) for every simulated day. | hard-fail | REF-IRRIG-CH12-INTRO, REF-IRRIG-CH12-SPRINKLER, REF-IRRIG-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IRRIG-002 | Sprinkler-concurrency merge invariant: when rainfall and sprinkler irrigation are concurrent, merge logic must follow §12.2.1 case rules (`D_irr <= D_rain` or `D_irr > D_rain`), output exactly 10 intensity-duration blocks, and conserve rainfall-plus-irrigation applied depth within declared tolerance. | hard-fail | REF-IRRIG-CH12-CONCURRENT-SPRINKLER, REF-IRRIG-CH2-DISAG-LINK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IRRIG-003 | Furrow-rainfall concurrency invariant: furrow irrigation is disallowed on rainfall days except when rainfall depth is below `0.001 m` and peak intensities are below effective hydraulic conductivity of all layers; exception handling must be explicit. | hard-fail | REF-IRRIG-CH12-CONCURRENT-FURROW | `[DIRECT][Static]` |
| INV-IRRIG-004 | Furrow-infiltration formulation invariant: Eq. [12.3.1]-[12.3.5] branch semantics must be enforced with non-negative `Z`, bounded `Ix` (`0 <= Ix <= (Ix)_max` when fronts meet), and finite infiltration opportunity-time solutions. | hard-fail | REF-IRRIG-CH12-FURROW-HYDROLOGY, REF-IRRIG-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IRRIG-005 | Furrow-hydraulics continuity invariant: continuity/kinematic-wave relations Eq. [12.3.6]-[12.3.14] must be solved with explicit phase-state handling (advance, continuing, depletion, recession) and no implicit phase skipping. | hard-fail | REF-IRRIG-CH12-FURROW-HYDRAULICS | `[DIRECT][Static]` |
| INV-IRRIG-006 | Inflow-management invariant: exactly one declared inflow-management branch (`constant`, `cut-back`, or `surge`) governs a furrow event; branch-specific assumptions (including grid-boundary shifting and omission of overlapping surge waves) must remain explicit. | hard-fail | REF-IRRIG-CH12-INFLOW | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IRRIG-007 | Depletion-level scheduling invariant: irrigation-trigger decisions must enforce irrigation-period boundaries and critical-depletion comparisons exactly as §12.4.2 describes, including the one-OFE-per-day limitation for depletion scheduling. | hard-fail | REF-IRRIG-CH12-SCHED-DEPLETION | `[DIRECT][Static]` |
| INV-IRRIG-008 | Fixed-date and combined scheduling invariant: fixed-date events fire only on matching simulation dates, and mode transitions in §12.4.3-§12.4.4 must be explicit when event lists or periods are exhausted. | hard-fail | REF-IRRIG-CH12-SCHED-FIXED, REF-IRRIG-CH12-SCHED-COMBINATION | `[DIRECT][Static]` |
| INV-IRRIG-009 | Coupling payload completeness invariant: irrigation domain must publish required hydrologic outputs for downstream consumers (`irrigation additions`, `qp`, `De`, and scheduling/mode context), with unit-consistent semantics and explicit provenance of rainfall-concurrency adjustments. | hard-fail | REF-IRRIG-CH12-FURROW-EROSION-LINK, REF-IRRIG-CH11-COUPLING, REF-IRRIG-CH12-CONCURRENT-SPRINKLER | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-IRRIG-010 | Governance limitation invariant: contract interpretation must keep Chapter-12 simplifications explicit (single-OFE depletion scheduling, omitted overlapping surge-wave interactions, and exception-scoped furrow-rainfall handling); missing caveat labeling blocks promotion. | governance-fail | REF-IRRIG-CH12-INFLOW, REF-IRRIG-CH12-SCHED-DEPLETION, REF-IRRIG-CH12-CONCURRENT-FURROW | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-IRRIG-001` | runtime | Irrigation event-domain validator | Typed hard error on negative/non-finite additions, durations, or missing mode declaration | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IRRIG-002` | runtime | Sprinkler concurrency hydrograph assembler | Typed hard error on rule-branch mismatch, non-10-block output, or conservation residual above tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IRRIG-003` | runtime | Furrow-rainfall concurrency gate | Typed hard error when furrow branch executes outside allowed exception | Tier-A gate | `[DIRECT][Static]` |
| `INV-IRRIG-004` | runtime | Furrow infiltration solver and bounds checker | Typed hard error on invalid Eq. [12.3.1]-[12.3.5] state domains or wetting-front cap violations | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IRRIG-005` | runtime | Furrow hydraulics phase-state solver | Typed hard error on continuity equation failure, non-convergent phase-step handling, or implicit phase transitions | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-IRRIG-006` | runtime | Inflow-management branch selector | Typed hard error on mixed/implicit branch activation or missing branch assumptions | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IRRIG-007` | runtime | Depletion-level scheduler | Typed hard error on period-gating violations or illegal multi-OFE depletion branch activation | Tier-A gate | `[DIRECT][Static]` |
| `INV-IRRIG-008` | runtime | Fixed-date/combined scheduling state machine | Typed hard error on date-trigger mismatch or invalid mode-transition sequence | Tier-A gate | `[DIRECT][Static]` |
| `INV-IRRIG-009` | runtime | Irrigation boundary payload validator | Typed hard error on missing required coupling surfaces (`qp`, `De`, irrigation-addition context) or units mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-IRRIG-010` | governance | Contract review/disposition/verification checklist | Promotion `HOLD` when simplification caveats are absent or silently treated as closed science | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-12 notation (with retained
WEPP-style symbols). Concrete openWEPP runtime-field names are not yet fixed,
so identity aliases are required until implementation surfaces diverge.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `D_irr`, `D_rain`, `I_irr` | identity names | sprinkler scheduling and concurrency surfaces | `s`, `s`, `m s^-1` preserved | `[DIRECT][Static]` |
| `i_k`, `N_blk` | identity names | concurrent-event hydrograph surfaces | intensity and block-count semantics preserved | `[DIRECT][Static]` |
| `Z`, `Ix`, `Iy`, `(Ix)_max` | identity names | furrow infiltration state surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Ke`, `k`, `a`, `fo` | identity names | furrow infiltration parameter surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Q`, `A`, `So`, `Sf`, `alpha`, `m`, `dt`, `dx` | identity names | furrow hydraulics solver surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `DL`, `DL_crit`, `I_req`, `I_min`, `I_max`, `p_req` | identity names | scheduling/requirement surfaces | depth/fraction semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `qp` | `qp` (primary), `Qp` (legacy alias) | erosion-coupling peak-runoff surface | `m^3 s^-1` preserved across aliases | `[DIRECT][Static] + [INFERENCE][Static]` |
| `De` | identity name | erosion-coupling duration surface | `s` preserved | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| No-irrigation day | Scheduling mode resolves to no-irrigation (outside periods or exhausted event list), with explicit no-irrigation flag/state. | Explicit §12.4.1 behavior. | `[DIRECT][Static]` |
| Sprinkler-only day | Irrigation occurs without rainfall, so no rainfall-concurrency merge is needed. | Valid non-concurrent branch in §12.2/§12.2.1. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Furrow day with negligible rainfall | Rainfall event exists but meets the low-depth/low-intensity exception; rainfall is not simulated and furrow irrigation proceeds. | Explicit §12.3.2 exception. | `[DIRECT][Static]` |
| Depletion non-trigger day | Day is within irrigation period but depletion ratio does not exceed critical threshold, so no irrigation occurs. | Explicit §12.4.2 trigger logic. | `[DIRECT][Static]` |
| Fixed-date miss day | Simulation date does not match a fixed-date event, so fixed-date branch performs no irrigation. | Explicit §12.4.3 date-equality trigger rule. | `[DIRECT][Static]` |

## Invalid States

- Negative or non-finite irrigation depths/rates/durations (`I_irr`, `I_req`, `D_irr`) beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Concurrent sprinkler event emitted with block count other than 10 or with unbounded merge residual. `[DIRECT][Static] + [INFERENCE][Static]`
- Furrow irrigation executed on rainfall day outside §12.3.2 exception. `[DIRECT][Static]`
- Furrow infiltration/hydraulics states with invalid domains (`Z < 0`, `Ix < 0`, `A < 0`, non-finite solver states). `[DIRECT][Static] + [INFERENCE][Static]`
- Ambiguous inflow-management branch activation (more than one branch active for the same event). `[DIRECT][Static] + [INFERENCE][Static]`
- Depletion-level scheduling assigns more than one OFE to irrigate on the same day. `[DIRECT][Static]`
- Missing required coupling outputs (`qp`, `De`, irrigation-addition context) at boundary publication time. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-IRRIG-P-001: Emit irrigation event/scheduling surfaces with canonical symbols and units declared in this contract. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IRRIG-P-002: Enforce explicit branching for concurrency, inflow-management, and scheduling transitions; no silent fallback branch behavior. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IRRIG-P-003: Surface invariant failures as typed errors; no silent clamping/defaulting on invalid hydraulic or scheduling states. `[INFERENCE][Static]`
- OBL-IRRIG-P-004: Preserve coupling-ready hydrologic outputs (`qp`, `De`, irrigation additions, mode context) for downstream erosion/water-balance consumers. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-IRRIG-C-001: Runoff/hydraulic consumers must preserve irrigation-addition units and concurrency-adjusted hydrograph semantics without reinterpretation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IRRIG-C-002: Erosion consumers must reject malformed `qp`/`De` payloads and preserve Chapter-12 effective-duration semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IRRIG-C-003: Water-balance/soil-water consumers must account for irrigation additions using declared sign/unit conventions and explicit scheduling-mode provenance. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-IRRIG-C-004: All consumers must fail explicitly on invariant-violating irrigation payloads and propagate invariant IDs in error context. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Event domain and concurrency merge (`INV-IRRIG-001/002/003`) | scheduling + concurrency assembly | Hard error; reject irrigation event publication | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Furrow infiltration/hydraulics state validity (`INV-IRRIG-004/005/006`) | furrow hydrology/hydraulics solver | Hard error on domain/branch/phase violation | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Scheduling state-machine integrity (`INV-IRRIG-007/008`) | scheduler transition stage | Hard error on invalid trigger or mode transition | Tier-A gate | `[DIRECT][Static]` |
| Coupling payload completeness (`INV-IRRIG-009`) | irrigation boundary handoff | Hard error on missing malformed required outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Simplification-governance labeling (`INV-IRRIG-010`) | review/verification/promotion | Governance `HOLD` until simplification caveats are explicit and retained | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). Contract-specific tolerances:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-IRRIG-001 | Sprinkler concurrent-event depth-conservation residual | `<= 1e-9 m` | Residual computed on total rainfall+irrigation applied depth versus merged-hydrograph depth. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-IRRIG-002 | Non-negative-domain comparator tolerance for irrigation/hydraulic magnitudes | lower bound `>= -1e-12` in declared units | Comparator-noise allowance only; runtime hard-fails on material negatives. | `[INFERENCE][Static]` |
| TOL-IRRIG-003 | Furrow-rainfall exception threshold | rainfall depth must satisfy `< 0.001 m` exactly, and peak intensity must remain below all-layer effective conductivity | Rule from §12.3.2 is treated as a hard branch threshold, not a smoothing region. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-IRRIG-004 | Scheduling trigger comparator tolerance for depletion ratio | `abs(DL - DL_crit) <= 1e-12` treated as threshold-equality boundary | Explicitly prevents floating-noise oscillation around the trigger boundary. | `[INFERENCE][Static]` |

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-IRRIG-001 | Per-invariant comparator vectors for all `INV-IRRIG-*` families are not yet curated in this package. | Limits immediate automation depth for irrigation-specific invariant gating. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-IRRIG-002 | Concrete openWEPP runtime-field aliases for canonical irrigation symbols are not yet fixed. | Alias map remains identity-only pending boundary finalization. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-IRRIG-003 | Downstream coupled contracts (`SC-HYDRAULICS-001`, `SC-SED-001`, `SC-ROUTE-001`) remain incomplete, so irrigation-output ownership boundaries are provisional. | Promotion-readiness depends on downstream contract completion and consistency. | non-promotable | `[DIRECT][Static]` |
| GAP-IRRIG-004 | Chapter-12 simplifications omit overlapping surge-wave interactions and constrain depletion scheduling to one OFE/day. | Some real-world irrigation regimes may require richer coupling semantics than this contract currently permits. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-14 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-12 authority anchors, irrigation invariants, guard map, alias map, obligations, tolerance notes, and gap register for SCI-14 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: normalized evidence-mode token, added claim-level evidence tags in scope/degenerate/tolerance sections, added explicit `qp`/`Qp` alias continuity row, and replaced broad Chapter-11 coupling citation with precise §11.2.2 Eq. [11.2.5] anchor. |
