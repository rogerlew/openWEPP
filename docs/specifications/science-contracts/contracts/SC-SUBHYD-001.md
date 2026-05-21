---
contract_id: SC-SUBHYD-001
title: Subsurface Hydrology and Drainage Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 2
producer_scope:
  - Daily subsurface lateral-flow flux surfaces from drainable-layer states
  - Surface depressional-storage and artificial-drainage flux surfaces
  - Subsurface coupling surfaces exported to daily closure and watershed routing
consumer_scope:
  - Daily water-balance accounting consumers
  - Watershed/channel routing consumers using subsurface and drainage contributions
  - Comparator/replay surfaces using daily closure confidence signals
evidence_level: static
last_reviewed: 2026-05-20
supersedes: []
superseded_by: []
---

# SC-SUBHYD-001 Subsurface Hydrology and Drainage Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for subsurface lateral flow, surface
storage/drainage transitions, and artificial subsurface drainage behavior in
openWEPP, including coupled daily boundary semantics for water balance and
watershed routing consumers.

## Scientific Scope

In scope:
- Daily subsurface lateral-flow accounting over drainable layers.
- Surface depressional-storage fill/release behavior coupled to runoff onset.
- Subsurface drainage-to-tile/ditch flux and water-table drawdown behavior.
- Coupling boundaries from Chapter-6 subsurface outputs into Chapter-5 daily
  closure and watershed/channel routing consumers.

Out of scope:
- Kernel implementation details and Rust API naming.
- Event-scale infiltration/rainfall-excess internals owned by
  `SC-RUNOFFPART-001`.
- Root-zone percolation constitutive equations owned by `SC-PERC-001`.
- Channel routing and erosion kernels owned by `SC-ROUTE-001` and
  `SC-SED-001`.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-SUBHYD-CH6-INTRO | `references/50201000/chap6.pdf` §6.1 | Declares Chapter-6 objective and coupling motivation for lateral flow, surface/subsurface drainage, and water-table effects on runoff/erosion. | `[DIRECT][Static]` |
| REF-SUBHYD-CH6-LATCONT | `chap6.pdf` §6.2.1 Eq. [6.2.1] | Daily continuity relation for drainable-layer control volume over hillslope length. | `[DIRECT][Static]` |
| REF-SUBHYD-CH6-LATSTOR | `chap6.pdf` §6.2.1 Eq. [6.2.2]-[6.2.3] | Drainable storage and drainable-water definitions (`S`, `Ho`, `θd`, `θ`, `θFC`, `θa`). | `[DIRECT][Static]` |
| REF-SUBHYD-CH6-LATFLUX | `chap6.pdf` §6.2.1 Eq. [6.2.4]-[6.2.5] | Subsurface lateral-flow Darcy-style flux and daily drainable-thickness update semantics. | `[DIRECT][Static]` |
| REF-SUBHYD-CH6-SURFDS | `chap6.pdf` §6.2.2 Eq. [6.2.6]-[6.2.9] | Surface depressional-storage capacity, fill requirement, and rainfall-excess release behavior. | `[DIRECT][Static]` |
| REF-SUBHYD-CH6-DRAINFLOW | `chap6.pdf` §6.2.3 Eq. [6.2.10]-[6.2.11] + text | Tile/ditch drainage-flux relation and equivalent-depth correction near drains. | `[DIRECT][Static]` |
| REF-SUBHYD-CH6-ANISO | `chap6.pdf` §6.2.3 Eq. [6.2.12]-[6.2.13] + text | Effective anisotropic conductivity and flow-angle relations (`Kzy`, `Kz`, `Ky`, `α`), including ditch horizontal-flow assumption. | `[DIRECT][Static]` |
| REF-SUBHYD-CH6-WTDRAW | `chap6.pdf` §6.2.3 Eq. [6.2.14]-[6.2.15] + text | Water-table drawdown by drainage and drainable-porosity definition; sequential layer withdrawal and negligible-flow condition below drain depth. | `[DIRECT][Static]` |
| REF-SUBHYD-CH5-COUPLING | `references/50201000/chap5.pdf` §5.1 Eq. [5.1.1] | Daily closure consumes subsurface/drain loss term (`Qd`) with explicit accounting semantics. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SUBHYD-CH13-COUPLING | `references/50201000/chap13.pdf` §13.1-§13.2 Eq. [13.2.1]-[13.2.2] | Watershed/channel runon boundaries include lateral hillslope contributions and require unit-consistent runon depth/volume semantics. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SUBHYD-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative flux magnitudes, bounded porosity domains, and explicit branch handling for threshold transitions. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `S` | `m` | Drainable depth/storage state used in daily continuity control volume. | subsurface continuity routine | daily continuity update |
| `Ho` | `m` | Drainable-layer thickness normal to slope. | subsurface lateral-flow routine | storage and lateral-flow flux equations |
| `θd` | `m^3 m^-3` | Drainable soil-water fraction from Eq. [6.2.3]. | subsurface state routine | storage and thickness update |
| `θ`, `θFC`, `θa` | `m^3 m^-3` | Total water content, field-capacity water content, and entrapped-air fraction. | soil/subsurface state routine | `θd` and porosity-domain calculations |
| `Pe` | `m d^-1` | Percolated water input to drainable layer. | percolation coupling (`SC-PERC-001`) | continuity/storage update |
| `D` | `m d^-1` | Seepage loss from drainable layer (Chapter-6 continuity term). | subsurface routine | continuity/storage update |
| `ET` | `m d^-1` | Actual evapotranspiration drawn from drainable layer in continuity relation. | ET coupling (`SC-EVAP-001`) | continuity/storage update |
| `L` | `m` | Hillslope segment length for control-volume scaling. | hillslope geometry input | continuity/storage/flux equations |
| `q` | `m d^-1` | Lateral subsurface discharge per unit width from hillslope. | subsurface lateral-flow routine | downslope OFE transfer and routing coupling |
| `Ke` | `m s^-1` | Effective horizontal hydraulic conductivity at moisture state `θ`. | soil hydraulic state routine | lateral-flow flux equation |
| `α` | `rad` | Average slope angle / effective flow-path angle. | topography/drainage geometry input | lateral-flow/drain-conductivity equations |
| `DS` | `cm` | Maximum depressional-storage depth. | surface drainage subroutine | runoff-onset and storage-fill logic |
| `PR` | `cm` | Rainfall excess required to satisfy depressional storage. | surface drainage subroutine | storage-fill branch condition |
| `Vi`, `Qi` | `cm h^-1` | Interval rainfall-excess rate and profile runoff rate while storage fills. | runoff/surface-drainage coupling | storage-fill accumulation and runoff release |
| `FL` | `cm` | Accumulated rainfall excess filling depressional storage. | surface drainage subroutine | storage-branch transition |
| `Qdd` | `cm d^-1` | Subsurface drainage flux to drains per unit width. | tile/ditch drainage routine | water-table drawdown and `Qd` coupling |
| `Kz`, `Ky`, `Kzy` | `cm d^-1` | Horizontal, vertical, and effective anisotropic conductivity for drainage flux. | subsurface/drainage routine | drainage-flux equation |
| `Md` | `cm` | Midpoint water-table height above drain elevation. | saturated-zone state routine | tile/ditch drainage-flux equation |
| `Ld` | `cm` | Distance between drain tiles or ditches. | drainage geometry input | tile/ditch drainage-flux equation |
| `h`, `he`, `r` | `cm` | Restrictive-layer-to-drain distance, equivalent depth correction, and tile radius. | drainage geometry routine | equivalent-depth/drain-flux equations |
| `md` | `cm` | Water-table depth/elevation state updated by drainage drawdown relation. | saturated-zone routine | daily subsurface-state transition |
| `D.C.` | `cm d^-1` | Drainage coefficient (hydraulic capacity cap) for tile/ditch system. | drainage design/parameter input | cap on emitted drainage flux |
| `φ`, `φdi` | `cm^3 cm^-3` | Porosity and drainable porosity in saturated-zone drawdown calculation. | soil-state routine | water-table drawdown update |
| `Qd` | `m` | Daily subsurface/drainage loss term exported to Chapter-5 closure. | subsurface coupling routine | daily water-balance closure and routing handoff |

## Daily Subsurface Closure Term Definition

For `INV-SUBHYD-001`, daily continuity accounting is published at the declared
Chapter-6 boundary as:

`S2 - S1 = [Pe - (D + ET)L - (q1 + q2)/2] (d2 - d1) + εsubhyd`

where:
- `S1`, `S2` are start/end drainable storage states for the day,
- `Pe`, `D`, `ET`, `L`, `q1`, and `q2` follow Eq. [6.2.1] definitions,
- `εsubhyd` is explicit residual constrained by `TOL-SUBHYD-001`.

This identity is an accounting constraint for contract enforcement and does not
replace the governing Chapter-6 process equations.
`[DIRECT][Static] + [INFERENCE][Static]`

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SUBHYD-001 | Daily continuity invariant: drainable-layer update must satisfy Eq. [6.2.1] with explicit residual tracking for all declared terms (`S`, `Pe`, `D`, `ET`, `L`, `q`). | hard-fail | REF-SUBHYD-CH6-LATCONT, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-002 | Drainable-water state invariant: Eq. [6.2.2]-[6.2.3] semantics are explicit, with `θd` derived from declared state terms and bounded to physically valid range before storage/flux emission. | hard-fail | REF-SUBHYD-CH6-LATSTOR, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-003 | Lateral-flux invariant: Eq. [6.2.4] emission of `q` must remain non-negative with finite conductivity/slope domains and explicit no-flow branch when drainable thickness is zero. | hard-fail | REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-004 | Drainable-thickness transition invariant: Eq. [6.2.5] update is explicit, denominator remains positive/finite, and emitted `Ho` remains within physically valid domain for downstream flux computations. | hard-fail | REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-005 | Surface-storage transition invariant: depressional-storage capacity/fill/release behavior follows Eq. [6.2.6]-[6.2.9] with explicit branch semantics (`FL < DS` vs `FL >= DS`) and non-negative storage/fill terms. | hard-fail | REF-SUBHYD-CH6-SURFDS, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-006 | Drainage-flux invariant: tile/ditch drainage flux follows Eq. [6.2.10]-[6.2.11], requires valid geometry/conductivity domains, and applies explicit equivalent-depth branch conditions. | hard-fail | REF-SUBHYD-CH6-DRAINFLOW, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-007 | Anisotropic-conductivity invariant: `Kzy` computation and flow-angle relations follow Eq. [6.2.12]-[6.2.13], including explicit `α = 0` horizontal-flow branch for ditch drainage. | hard-fail | REF-SUBHYD-CH6-ANISO | `[DIRECT][Static]` |
| INV-SUBHYD-008 | Water-table drawdown invariant: Eq. [6.2.14]-[6.2.15] update requires positive drainable porosity domain, explicit layer-withdrawal sequencing, and explicit negligible-flow branch when water table is below drain elevation. | hard-fail | REF-SUBHYD-CH6-WTDRAW, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-011 | Drainage-capacity invariant: emitted tile/ditch drainage flux cannot exceed declared drainage coefficient (`Qdd <= D.C.`); when Eq. [6.2.10] exceeds capacity, output is explicitly capped. | hard-fail | REF-SUBHYD-CH6-DRAINFLOW, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-009 | Cross-domain coupling invariant: daily subsurface/drain loss term exported as `Qd` is unit/sign-consistent with Chapter-5 daily closure and preserves subsurface-contribution semantics for watershed/channel runon accounting. | hard-fail | REF-SUBHYD-CH5-COUPLING, REF-SUBHYD-CH13-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-010 | Governance scope invariant: contract claims remain within Chapter-6 subsurface/drainage scope; unsupported extrapolation to alternate groundwater/baseflow physics without companion authority is non-promotable. | governance-fail | REF-SUBHYD-CH6-INTRO, REF-SUBHYD-CH13-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-SUBHYD-001` | runtime | Daily continuity assembler for Eq. [6.2.1] | Typed hard error on residual above tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-002` | runtime | Drainable-state derivation validator for Eq. [6.2.2]-[6.2.3] | Typed hard error on invalid domain or malformed state derivation | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-003` | runtime | Lateral-flow flux evaluator for Eq. [6.2.4] | Typed hard error on invalid/non-physical flux output | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-004` | runtime | Drainable-thickness update evaluator for Eq. [6.2.5] | Typed hard error on invalid denominator/domain or non-physical `Ho` transition | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-005` | runtime | Surface-storage branch evaluator for Eq. [6.2.6]-[6.2.9] | Typed hard error on branch-condition mismatch or invalid storage/fill domain | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-006` | runtime | Tile/ditch drainage-flux evaluator for Eq. [6.2.10]-[6.2.11] | Typed hard error on invalid geometry/conductivity/equivalent-depth branch | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-007` | runtime | Anisotropic-conductivity and flow-angle validator | Typed hard error on invalid `Kzy`/`α` domain or branch misuse | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-SUBHYD-008` | runtime | Water-table drawdown updater for Eq. [6.2.14]-[6.2.15] | Typed hard error on invalid porosity domain or drawdown branch misuse | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-011` | runtime | Post-computation drainage-capacity validator | Typed hard error on uncapped `Qdd` output exceeding declared `D.C.` | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-009` | runtime | Subsurface boundary payload validator for `Qd` handoff | Typed hard error on missing malformed field or units/sign mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-010` | governance | Contract review/disposition/verification + promotion checklist | Promotion `HOLD` when scope claims exceed declared authority boundary | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols follow Chapter-6 WEPP notation. Concrete openWEPP
runtime-field names are not yet fixed for this domain, so identity aliases are
required until implementation surfaces diverge.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `S`, `Ho`, `θd`, `θ`, `θFC`, `θa` | identity names | drainable storage and state-definition surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Pe`, `D`, `ET`, `L`, `q`, `Ke`, `α` | identity names | continuity and lateral-flow flux surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `DS`, `PR`, `Vi`, `Qi`, `FL` | identity names | surface storage/fill/release branch surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Qdd`, `D.C.`, `Kz`, `Ky`, `Kzy`, `Md`, `Ld`, `h`, `he`, `r`, `md`, `φ`, `φdi` | identity names | tile/ditch drainage and water-table drawdown surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Qd` | identity name | daily subsurface loss handoff surface | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| No-drainable-layer state | `Ho = 0` yielding `q = 0` for the daily step. | Physically consistent no-lateral-flow branch under zero drainable thickness. |
| Field-capacity boundary state | `θd` near zero with continuity terms dominated by `Pe`, `D`, and `ET` updates. | Expected transition regime around drainable-water threshold. |
| Storage-filling interval | `FL < DS` with reduced `Qi` while depressional storage fills. | Explicit Eq. [6.2.8] branch behavior. |
| Storage-satisfied interval | `FL >= DS` and profile runoff equals rainfall-excess interval rate (`Qi = Vi`). | Explicit Eq. [6.2.8] runoff-release branch. |
| No-active-drainflow state | Water table below tile/ditch elevation causing negligible `Qdd`. | Explicit Chapter-6 branch statement for below-drain condition. |

## Invalid States

- Eq. [6.2.1] continuity residual above declared tolerance without typed failure. `[DIRECT][Static] + [INFERENCE][Static]`
- Drainable-water state emitted with invalid `θd` domain or inconsistent Eq. [6.2.2]-[6.2.3] derivation terms. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative/non-finite lateral flux `q` or non-physical `Ho` transition from Eq. [6.2.4]-[6.2.5]. `[DIRECT][Static] + [INFERENCE][Static]`
- Surface-storage branch mismatch (`FL` branch violated) or negative `DS`/`PR`/`FL` domains. `[DIRECT][Static] + [INFERENCE][Static]`
- Tile/ditch drainage computation with invalid geometry/conductivity domains or invalid equivalent-depth branch. `[DIRECT][Static] + [INFERENCE][Static]`
- Water-table drawdown update with invalid/non-positive `φdi` denominator domain. `[DIRECT][Static] + [INFERENCE][Static]`
- Emitted drainage flux exceeding declared drainage coefficient (`Qdd > D.C.`) without explicit cap/failure behavior. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing/malformed `Qd` payload for daily-closure and routing-boundary consumers. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-SUBHYD-P-001: Emit canonical Chapter-6 subsurface/drainage surfaces with declared units and explicit branch semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SUBHYD-P-002: Enforce Eq. [6.2.1]-[6.2.15] domain guards before publishing downstream boundary payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SUBHYD-P-003: Emit daily `Qd` coupling term with Chapter-5 compatible sign and units semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SUBHYD-P-004: Propagate invariant violations as typed errors; no silent clamping/defaulting of subsurface/drainage terms. `[INFERENCE][Static]`
- OBL-SUBHYD-P-005: Enforce the drainage-capacity cap (`D.C.`) on emitted `Qdd` when Eq. [6.2.10] exceeds hydraulic capacity. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-SUBHYD-C-001: Water-balance consumers must ingest `Qd` with Eq. [5.1.1] closure semantics unchanged. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SUBHYD-C-002: Routing consumers must preserve unit-consistent lateral/subsurface contribution semantics from hillslope boundaries. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SUBHYD-C-003: Coupled hydrology consumers must reject malformed subsurface/drainage payloads and fail explicitly. `[INFERENCE][Static]`
- OBL-SUBHYD-C-004: Consumers must preserve declared branch-state meaning for storage and drainage transitions (no implicit reinterpretation). `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Continuity and drainable-state definitions (`INV-SUBHYD-001/002`) | daily subsurface-state assembly | Hard error; reject daily state publish | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Lateral-flow and thickness transition domains (`INV-SUBHYD-003/004`) | lateral-flow evaluation stage | Hard error on invalid flux/domain transitions | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Surface-storage transitions (`INV-SUBHYD-005`) | rainfall-excess/storage branch stage | Hard error on branch mismatch or invalid storage domains | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Tile/ditch drainage, capacity cap, and anisotropic conductivity (`INV-SUBHYD-006/007/011`) | drainage-flux stage | Hard error on invalid geometry/conductivity/equivalent-depth domains or uncapped over-capacity drainage flux | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Water-table drawdown transition (`INV-SUBHYD-008`) | saturated-zone update stage | Hard error on invalid drawdown-domain or branch misuse | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Cross-domain coupling payload (`INV-SUBHYD-009`) | subsurface boundary handoff | Hard error on missing malformed field or unit/sign mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Governance scope boundary (`INV-SUBHYD-010`) | review/verification/promotion | Governance `HOLD` until scope claims match declared authority | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). Contract-specific tolerances:

| Tolerance ID | Definition | Value | Notes |
|---|---|---|---|
| TOL-SUBHYD-001 | Daily continuity residual tolerance for Eq. [6.2.1] | `<= 1e-9 m` | Residual computed at declared daily control-volume boundary. |
| TOL-SUBHYD-002 | Non-negative comparator tolerance for lateral and drainage flux terms (`q`, `Qdd`, `Qd`) | lower bound `>= -1e-12` in declared units | Comparator-noise allowance only; runtime hard-fails on material negatives. |
| TOL-SUBHYD-003 | Drainable-water threshold tolerance around `θd` boundary | lower bound `>= -1e-12 m^3 m^-3` | Used only for threshold-adjacent comparisons; runtime still rejects material domain violations. |
| TOL-SUBHYD-004 | Positive-denominator tolerance for Eq. [6.2.5] and Eq. [6.2.14] updates | denominator `> 1e-12` in declared units | Prevents unstable division in thickness and water-table updates. |
| TOL-SUBHYD-005 | Storage-branch transition tolerance around `FL - DS` | `abs(FL - DS) <= 1e-9 cm` treated as branch boundary | Prevents branch jitter near storage-satisfaction threshold. |
| TOL-SUBHYD-006 | Drainage-capacity cap tolerance | `Qdd - D.C. <= 1e-12 cm d^-1` | Comparator-noise allowance around hard cap boundary; runtime still enforces explicit cap/failure behavior. |

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SUBHYD-001 | Per-invariant comparator vectors for Chapter-6 lateral-flow, storage-branch, and drainage-drawdown families are not yet curated in this package. | Limits immediate automation depth for invariant-specific acceptance checks. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-SUBHYD-002 | Concrete openWEPP runtime-field aliases for subsurface/drainage surfaces are not yet fixed. | Alias map remains identity-only pending boundary finalization. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SUBHYD-003 | Companion routing contract (`SC-ROUTE-001`) is not yet authored, so watershed handoff ownership for subsurface contributions remains provisional. | Promotion-readiness depends on downstream contract completion/consistency. | non-promotable | `[DIRECT][Static]` |
| GAP-SUBHYD-004 | Chapter-6 validation reports strong storm-runoff agreement but explicitly notes less-acceptable peak-runoff agreement due hydraulic-roughness uncertainty in available datasets. | Peak-rate confidence for coupled routing interpretation is lower than aggregate runoff-volume confidence until dedicated evidence is added. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-09 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-6 authority anchors, invariants, guard map, alias map, obligations, boundary disposition, tolerances, and gap register for SCI-09 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added explicit Eq. [6.2.1] closure identity, added drainage-coefficient (`D.C.`) variable and capacity-cap invariant/guard/tolerance, and expanded producer obligations for hydraulic-capacity enforcement. |
