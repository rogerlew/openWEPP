---
contract_id: SC-SUBHYD-001
title: Subsurface Hydrology and Drainage Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 14
producer_scope:
  - Daily subsurface lateral-flow flux surfaces from drainable-layer states
  - Surface depressional-storage and artificial-drainage flux surfaces
  - Subsurface coupling surfaces exported to daily closure and watershed routing
consumer_scope:
  - Daily water-balance accounting consumers
  - Watershed/channel routing consumers using subsurface and drainage contributions
  - Comparator/replay surfaces using daily closure confidence signals
evidence_level: static
last_reviewed: 2026-05-31
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

## Algorithm State Surfaces (WB19 Lateral/Drainage Production Kernels)

### Required Inputs

| Surface | Symbols |
|---|---|
| Scheduler phase metadata | `phase_name`, `phase_class`, `consumer_adapter` |
| Layer hydrology state family | `nsl`, `solthk`, `solwpv`, `dg_####`, `por_####`, `coca_####`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####` |
| Lateral geometry + conductivity family | `avgslp`, `slplen`, `wb19_lateral_anisotropy_ratio` |
| Drainage geometry + capacity family | `wb19_drain_enabled`, `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter`, `wb11_drainage_coefficient` |
| Coupling carry-forward surface | `Pe` |

### Required Outputs

| Surface | Output |
|---|---|
| Lateral flux output | `q` |
| Drainage outputs | `Qdd`, `Qd` |
| Lateral/drainage state updates | `wb11_drainable_storage`, `wb18_perc_theta_####`, `wb19_fcdep`, `wb19_unsdep`, `wb19_watyld` |
| Scheduler/kernel failure surface | Typed hard-fail status for missing/non-finite/out-of-range lateral/drainage domains |

### Mutated State Surfaces

WB19 mutates lateral/drainage boundary surfaces deterministically:
- lateral phase computes Eq. [6.2.4]-derived `q`, withdraws layer water above
  field capacity from top to bottom, updates `wb18_perc_theta_####`, updates
  `wb11_drainable_storage`, and publishes coupled saturated-depth surfaces
  (`wb19_fcdep`, `wb19_unsdep`, `wb19_watyld`).
- drainage phase computes Eq. [6.2.10]-[6.2.11]-derived `Qdd` with explicit
  equivalent-depth branch + capacity cap, performs tile-layer-to-surface
  withdrawal, updates `wb18_perc_theta_####`, updates
  `wb11_drainable_storage`, and emits `Qd = q + Qdd`.

## Algorithm Specification (WB19 Lateral/Drainage Production Execution)

1. Lateral phase loads WB18 per-layer states (`theta`, `fc`, `ul`, `ssc`,
   `dg`, `por`, `coca`) and computes saturated-zone metrics and effective
   conductivity over saturated thickness:
   - `drfc_i = fc_i + (1-coca_i)*dg_i`
   - `solwpv = 2006`: saturated block includes all layers where
     `theta_i >= drfc_i`
   - `solwpv != 2006`: saturated block is contiguous from surface until first
     unsaturated layer
   - `avpora = Σ(por_i * dg_i / fcdep)`,
     `avfca = Σ((fc_i/dg_i) * dg_i / fcdep)`,
     `avcoca = Σ(coca_i * dg_i / fcdep)`
   - `watyld = avpora - (avfca + (1-avcoca))`
   - `Ke = 86400 * (Σ(ssc_i * dg_i) / Σ(dg_i))`
   - `alpha = atan(avgslp)`
   - `q_potential = (Ho * wb19_lateral_anisotropy_ratio * Ke * sin(alpha)) / slplen`
2. Lateral phase withdraws `q` from layer excess water (`theta_i - drfc_i`) in
   top-to-bottom sequence and emits actual `q` after residual withdrawal
   reduction; for `solwpv < 2006`, update `fcdep = max(fcdep - q/watyld, 0)`
   and `unsdep = soldep - fcdep`.
3. Drainage phase (when `wb19_drain_enabled = 1`) computes Eq. [6.2.10]-[6.2.11]
   branch values using `wb19_drain_depth`, `wb19_drain_spacing`,
   `wb19_drain_diameter`, water-table depth from saturated-layer state, and
   effective saturated conductivity. Emitted `Qdd` is capped by
   `wb11_drainage_coefficient`.
4. Drainage phase withdraws `Qdd` from tile-layer-to-surface excess-water
   sequence and emits total subsurface loss `Qd = q + Qdd`.
5. Reject missing, non-finite, or out-of-range lateral/drainage domains with
   typed hard-fail status; no silent fallback/clamping paths are permitted.

## Branch and Guard Table (WB19 Lateral/Drainage Kernels)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-SUBHYD-WB19-LATERAL-EXECUTE` | phase class `hydrology_lateral_transfer` | `nsl`, `solthk`, `solwpv`, `dg_####`, `por_####`, `coca_####`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`, `avgslp`, `slplen`, `wb19_lateral_anisotropy_ratio`, `Pe` | runtime | deterministic layer-aware lateral execution/writeback |
| `BR-SUBHYD-WB19-DRAIN-EXECUTE` | phase class `hydrology_drainage` | WB19 lateral symbols + `wb19_drain_enabled`, `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter`, `wb11_drainage_coefficient`, `q` | runtime | deterministic layer-aware drainage execution/writeback |
| `BR-SUBHYD-WB19-LATERAL-GUARD` | lateral symbol missing/non-finite/out-of-range | WB19 lateral required + emitted symbols | runtime | typed hard-fail (`HKERNEL-WB11-LAT-E-001..003`) |
| `BR-SUBHYD-WB19-DRAIN-GUARD` | drainage symbol missing/non-finite/out-of-range | WB19 drainage required + emitted symbols | runtime | typed hard-fail (`HKERNEL-WB11-DRAIN-E-001..003`) |

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
| INV-SUBHYD-012 | WB19 lateral execution invariant: lateral phase computes Eq. [6.2.4]-style deterministic `q` from layer-aware conductivity/geometry symbols and updates `wb18_perc_theta_####` + `wb11_drainable_storage` through top-to-bottom excess-water withdrawal above `drfc_i = fc_i + (1-coca_i)*dg_i`. | hard-fail | REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-013 | WB19 drainage execution invariant: drainage phase computes Eq. [6.2.10]-[6.2.11] deterministic `Qdd`, applies explicit capacity cap (`wb11_drainage_coefficient`), emits `Qd = q + Qdd`, and updates `wb18_perc_theta_####` + `wb11_drainable_storage` using `drfc_i` threshold lineage without implicit fallback branches. | hard-fail | REF-SUBHYD-CH6-DRAINFLOW, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-014 | WB19 lateral/drainage guard invariant: missing/non-finite/out-of-range WB19 lateral/drainage domains must surface typed hard failures (`HKERNEL-WB11-LAT-E-001..003`, `HKERNEL-WB11-DRAIN-E-001..003`) and cannot be silently clamped/defaulted. | hard-fail | REF-SUBHYD-PHYS-BOUNDS | `[INFERENCE][Static]` |
| INV-SUBHYD-015 | WB19 water-yield/saturated-depth invariant: lateral phase must apply `solwpv` branch semantics and publish finite coupled states (`wb19_watyld`, `wb19_fcdep`, `wb19_unsdep`); for `solwpv < 2006` with active saturated block, `watyld` must be positive and `fcdep/unsdep` update must follow `fcdep = max(fcdep - q/watyld, 0)` and `unsdep = soldep - fcdep`. | hard-fail | REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-016 | WB19 realized-withdrawal soil-water cap invariant: lateral (`q`) and drainage (`Qdd`) realized withdrawals must not exceed pre-phase `wb11_soil_water`; over-withdrawal is a typed hard-fail domain condition and must not be silently clamped by post-subtraction flooring. | hard-fail | REF-SUBHYD-CH6-LATCONT, REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-CH6-DRAINFLOW, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| `INV-SUBHYD-012` | runtime | WB19 lateral production kernel execution path | Typed hard error on malformed/non-deterministic layer-aware lateral writeback outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-013` | runtime | WB19 drainage production kernel execution path | Typed hard error on malformed/non-deterministic layer-aware drainage writeback outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-014` | runtime | WB19 lateral/drainage guard tables | Typed hard error on missing/non-finite/domain-invalid WB19 lateral/drainage inputs/outputs | Tier-A gate | `[INFERENCE][Static]` |
| `INV-SUBHYD-015` | runtime | WB19 lateral water-yield + saturated-depth branch/coupling validator | Typed hard error on malformed `solwpv` branch behavior or invalid `wb19_watyld`/`wb19_fcdep`/`wb19_unsdep` coupling outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-016` | runtime + external-authority | WB19 realized-withdrawal cap validator plus Level-4 constitutive suite checks | Typed hard error on `q`/`Qdd` over-withdrawal relative to pre-phase `wb11_soil_water`; no silent flooring/default behavior allowed | Tier-A gate + required A3 lane | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols follow Chapter-6 WEPP notation with explicit WB18/WB19 runtime
alias continuity for production kernels.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `θ_i` | `wb18_perc_theta_####` | WB19 per-layer moisture state surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `θFC_i` | `wb18_perc_fc_####` | WB19 per-layer field-capacity surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `θUL_i` | `wb18_perc_ul_####` | WB19 per-layer upper-limit surfaces used in branch coupling checks | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `K_i` | `wb18_perc_ssc_####` | WB19 per-layer saturated conductivity surfaces | `m s^-1` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `dg_i` | `dg_####` | WB19 per-layer thickness surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `por_i` | `por_####` | WB19 per-layer porosity surfaces used in water-yield coupling | dimensionless preserved (`0 < por <= 1`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `coca_i` | `coca_####` | WB19 entrapped-air correction surfaces used by drain-threshold lineage | dimensionless preserved (`0 < coca <= 1`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `drfc_i` | `wb18_perc_fc_#### + (1-coca_####)*dg_####` | WB19 drain-threshold lineage used for saturated-zone classification and withdrawals | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `watyld` | `wb19_watyld` | WB19 water-yield coupling state for `solwpv < 2006` `fcdep` updates | dimensionless preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `fcdep`, `unsdep` | `wb19_fcdep`, `wb19_unsdep` | WB19 saturated/unsaturated depth states after lateral update | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `L` | `slplen` | hillslope length for lateral flux denominator | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `α` | `atan(avgslp)` | slope angle reconstructed from runtime slope ratio | `rad` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `anisrt` | `wb19_lateral_anisotropy_ratio` | lateral anisotropy multiplier | dimensionless preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Pe` | `Pe` | percolation carry-forward coupling surface | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `DS`, `PR`, `Vi`, `Qi`, `FL` | identity names | surface storage/fill/release branch surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `D.C.` | `wb11_drainage_coefficient` | WB19 drainage-capacity cap | `m d^-1` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ddrain`, `sdrain`, `drdiam` | `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter` | WB19 drainage-geometry surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Qdd`, `Qd` | `Qdd`, `Qd` | drainage and aggregate subsurface-loss outputs | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| WB19 lateral/drainage production execution and guards (`INV-SUBHYD-012/013/014`) | lateral/drainage kernel execution and guard validation | Hard error on malformed WB19 lateral/drainage domains or invalid deterministic updates | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Constants and Parameters Table

| Constant/parameter | Units | Domain | Contract use | Authority |
|---|---|---|---|---|
| `WB19_LATERAL_STATUS_OK` | status message id (legacy ID retained) | `HKERNEL-WB11-LAT-OK-001` | Typed nominal status for successful WB19 lateral phase execution | REF-SUBHYD-CH6-LATFLUX |
| `WB19_DRAINAGE_STATUS_OK` | status message id (legacy ID retained) | `HKERNEL-WB11-DRAIN-OK-001` | Typed nominal status for successful WB19 drainage phase execution | REF-SUBHYD-CH6-DRAINFLOW |
| `WB19_LATERAL_GUARD_CODES` | status message id range (legacy IDs retained) | `HKERNEL-WB11-LAT-E-001..003` | Typed WB19 lateral guard codes for missing/non-finite/domain failures | REF-SUBHYD-PHYS-BOUNDS |
| `WB19_DRAINAGE_GUARD_CODES` | status message id range (legacy IDs retained) | `HKERNEL-WB11-DRAIN-E-001..003` | Typed WB19 drainage guard codes for missing/non-finite/domain failures | REF-SUBHYD-PHYS-BOUNDS |

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

## Test-Vector Obligations

Minimum WB19 lateral/drainage production-kernel conformance vectors:

1. Lateral phase emits deterministic Eq. [6.2.4]-derived `q`, updates
   `wb18_perc_theta_####`, and updates `wb11_drainable_storage` from valid WB19
   lateral inputs.
2. Drainage phase emits Eq. [6.2.10]-[6.2.11]-derived cap-limited `Qdd`,
   emits `Qd`, and updates `wb18_perc_theta_####` + `wb11_drainable_storage`
   from valid WB19 drainage inputs.
3. Non-finite/domain-invalid WB19 lateral/drainage inputs hard-fail with typed
   guard codes and do not mutate orchestrator writeback surfaces.

## WB12 Reconciliation Coupling Addendum

### WB12 Coupling Surfaces

| Surface | Symbols |
|---|---|
| Subhyd export into storage reconciliation | `Qd` |
| Drainage/lateral diagnostics available to reconciliation checks | `q`, `Qdd`, `Qd` |
| WB12 storage reconciliation outputs | `wb12_storage_reconciled`, `wb12_storage_closure_delta` |

### WB12 Coupling Requirements

1. `Qd` exported from WB19 lateral/drainage phases remains the required subsurface-loss term consumed by WB12 storage reconciliation.
2. WB12 storage reconciliation must treat `Qd` as a non-negative loss magnitude in closure diagnostics.
3. Missing/non-finite `Qd` at storage reconciliation boundaries is an invalid runtime state and must hard-fail with typed WB12 storage guard codes.

### WB12 Coupling Test Vectors

1. Nominal WB12 reconciliation vector consumes finite `Qd` and preserves deterministic storage closure diagnostics.
2. Missing or non-finite `Qd` during WB12 storage reconciliation hard-fails before writeback mutation.

## WB13 Daily Output Coupling Addendum

### WB13 Subsurface/Drainage Output Symbols

| WB13 column | Subsurface/drainage coupling surface | Units |
|---|---|---|
| `latqcc` | Daily lateral subsurface flow contribution exported for WB13 reporting | `mm` |
| `Tile` | Daily tile/ditch drainage contribution exported for WB13 reporting | `mm` |
| `SubRIn` | Daily subsurface runon contribution added to OFE | `mm` |
| `Qd` | Daily aggregate subsurface/drainage loss term used in closure diagnostics | `mm` |

### WB13 Coupling Requirements

1. WB13 rows must include finite, non-negative `latqcc`, `Tile`, and `SubRIn`
   symbols.
2. Where `Qd` is exported concurrently, WB13 coupling remains deterministic
   under `Qd = latqcc + Tile`.
3. Missing/non-finite/out-of-domain subsurface/drainage WB13 symbols are
   invalid runtime states and must hard-fail with WB13 typed guard posture.

## HPHYS0203 Subsurface WB13 Robustness Validation Addendum

1. Contract-derived robustness vectors must include deterministic WB13
   publication checks for `latqcc` and `Dp` domain continuity:
   - finite publication values,
   - non-negative publication magnitudes,
   - no projection-side surrogate substitution on missing or invalid inputs.
2. Robustness vectors must include targeted guard probes for negative/non-finite
   lateral/percolation source symbols consumed by WB13 publication assembly.
3. Deterministic regression fixtures must preserve `latqcc`/`Dp` column
   availability and domain validity under canonical publication authority.

## HPHYS0208 Coupled Subsurface Residual Closure Addendum

1. WB13 `latqcc` and `Dp` closure evidence is coupled to WB11/WB18 threshold
   seed lineage because lateral/drainage and percolation consumers operate on
   seeded layer storage surfaces (`wb18_perc_theta/fc/ul_####`).
2. Coupled threshold-lineage seed requirements are explicit:
   `sat`, `por_####`, `cpm_####`, `thetfc_####`, `thetdr_####`, and `dg_####`
   must be valid for the same layer domain used by WB19/WB18 consumers.
3. Missing/non-finite/domain-invalid coupled threshold-lineage inputs are
   typed hard-fail subsurface/percolation publication states; surrogate
   fallback is prohibited for WB13 `latqcc`/`Dp` closure claims.

## HPHYS0218 WB19 `drfc` Threshold-Lineage Addendum

1. WB19 layer drain threshold authority is:
   `drfc_i = wb18_perc_fc_i + (1-coca_i)*dg_i`.
2. WB19 saturated-zone classification and realized lateral/drainage
   withdrawals (`q`, `Qdd`) must use `drfc_i` as the layer threshold.
3. `coca_####` is required and must satisfy `0 < coca <= 1`; missing/non-finite/
   domain-invalid values are typed hard-fail WB19 execution states.
4. FC-only fallback threshold behavior is prohibited for WB19 closure claims.

## HPHYS0221 WB19 Water-Yield and Saturated-Depth Coupling Addendum

1. WB19 lateral execution must preserve baseline `solwpv` branch semantics:
   - `solwpv = 2006`: include all saturated layers (`theta_i >= drfc_i`).
   - `solwpv != 2006`: include only contiguous near-surface saturated layers
     until first unsaturated layer.
2. WB19 lateral coupling must compute:
   - `avpora = Σ(por_i * dg_i / fcdep)`,
   - `avfca = Σ((fc_i/dg_i) * dg_i / fcdep)`,
   - `avcoca = Σ(coca_i * dg_i / fcdep)`,
   - `watyld = avpora - (avfca + (1-avcoca))`.
3. For `solwpv < 2006` with active saturated block, update saturated-depth
   states using:
   - `fcdep = max(fcdep - q/watyld, 0)`,
   - `unsdep = soldep - fcdep`.
4. Lateral writeback must publish `wb19_watyld`, `wb19_fcdep`, and
   `wb19_unsdep`.
5. Missing/non-finite/domain-invalid branch/coupling symbols (`solwpv`,
   `por_####`, `wb19_watyld` when required) are typed hard-fail states; no
   fallback path is allowed.

## HPHYS0222 WB19 `solwpv` Branch-Authority Correction Addendum

1. WB19 lateral saturated-layer selection remains:
   - `solwpv = 2006`: all saturated layers.
   - `solwpv != 2006`: contiguous near-surface saturated block.
2. WB19 coupled saturated-depth mutation (`fcdep`, `unsdep`) is authorized
   only for `solwpv < 2006`.
3. For `solwpv >= 2006`, WB19 must not apply `fcdep = fcdep - q/watyld`.
4. Disturbed-soil modes (`solwpv >= 9001`) remain valid for conductivity
   selection but are not authorized to trigger WB19 `fcdep` mutation.
5. External-authority legacy-conformance suite:
   `cas_l3_subhyd_solwpv_fcdep_branch_001` (`periodic`,
   `investigation`), linked to `INV-SUBHYD-015` as non-blocking governance
   evidence pending independent constitutive authority.

## HPHYS0224 WB19 Realized-Withdrawal Soil-Water Cap Addendum

1. WB19 realized lateral/drainage withdrawals (`q`, `Qdd`) must satisfy:
   - `0 <= q <= wb11_soil_water_before_phase`,
   - `0 <= Qdd <= wb11_soil_water_before_phase`.
2. Production subtraction for `wb11_soil_water_after` must be explicit and
   non-clamping:
   - lateral phase: `wb11_soil_water_after = wb11_soil_water_before - q`,
   - drainage phase: `wb11_soil_water_after = wb11_soil_water_before - Qdd`.
3. Any attempted over-withdrawal relative to pre-phase
   `wb11_soil_water_before` is a typed WB19 domain violation and must not be
   repaired by silent floor logic.
4. External-authority constitutive suite
   `cas_l4_subhyd_withdrawal_soilwater_cap_001` is required/hard-fail and
   linked to `INV-SUBHYD-016`.

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
| `2026-06-01` | `16` | `Codex` | HPHYS0224 amendment: added `INV-SUBHYD-016` realized-withdrawal soil-water cap authority, explicit non-clamping subtraction requirements for WB19 lateral/drainage phases, and required Level-4 suite linkage (`cas_l4_subhyd_withdrawal_soilwater_cap_001`). |
| `2026-05-31` | `15` | `Codex` | AUTH09 taxonomy normalization: introduced Level-3 legacy/sanity tier usage for WB19 branch governance and renamed suite reference to `cas_l3_subhyd_solwpv_fcdep_branch_001`. |
| `2026-05-31` | `13` | `Codex` | HPHYS0222 amendment: corrected WB19 `fcdep/unsdep` mutation authority to `solwpv < 2006` only; clarified disturbed-soil mode interaction and linked external-authority suite `cas_l4_subhyd_solwpv_fcdep_branch_001`. |
| `2026-05-31` | `14` | `Codex` | AUTH08A governance re-tiering: reclassified `cas_l4_subhyd_solwpv_fcdep_branch_001` as periodic/investigation legacy-conformance evidence (non-blocking) pending independent constitutive authority. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-09 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-6 authority anchors, invariants, guard map, alias map, obligations, boundary disposition, tolerances, and gap register for SCI-09 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added explicit Eq. [6.2.1] closure identity, added drainage-coefficient (`D.C.`) variable and capacity-cap invariant/guard/tolerance, and expanded producer obligations for hydraulic-capacity enforcement. |
| `2026-05-23` | `3` | `Codex` | WB10 amendment: added explicit lateral/drainage phase-entry routing authority, unsupported-class typed hard-fail posture, and WB10 lateral/drainage test-vector obligations. |
| `2026-05-23` | `4` | `Codex` | WB11 amendment: promoted lateral/drainage sections from routing-only scaffolding to production-kernel authority with deterministic `q`/`Qdd`/`Qd` updates, typed WB11 guard codes, and WB11 contract-derived vectors. |
| `2026-05-23` | `5` | `Codex` | WB12 amendment: added explicit storage-reconciliation coupling authority for `Qd` consumption and typed WB12 closure-diagnostic failure posture. |
| `2026-05-23` | `6` | `Codex` | WB13 amendment: added canonical daily output coupling authority for subsurface/drainage symbols (`latqcc`, `Tile`, `SubRIn`) and deterministic `Qd` relation posture with malformed-output hard-fail requirements. |
| `2026-05-23` | `7` | `Codex` | WB19 amendment: replaced WB11 fraction-split lateral/drain surrogate authority with layer-aware Eq. [6.2.4]/[6.2.10]-[6.2.11] production-kernel authority, explicit WB18/WB19 symbol aliases, and legacy-ID typed guard continuity requirements. |
| `2026-05-30` | `8` | `Codex` | HPHYS0203 amendment: added subsurface WB13 robustness validation obligations for `latqcc`/`Dp` domain guards, non-finite protections, and deterministic regression-fixture coverage. |
| `2026-05-30` | `9` | `Codex` | HPHYS0208 amendment: added coupled threshold-lineage closure authority linking WB13 `latqcc`/`Dp` residual adjudication to WB11/WB18 seed symbols (`sat`, `por_####`, `cpm_####`, `thetfc_####`, `thetdr_####`, `dg_####`) with explicit fail-closed/no-fallback posture. |
| `2026-05-31` | `10` | `Codex` | HPHYS0218 amendment: required WB19 `drfc` threshold lineage (`wb18_perc_fc_#### + (1-coca_####)*dg_####`) for saturated-zone checks and withdrawals, with fail-closed `coca_####` guard continuity. |
| `2026-05-31` | `11` | `Codex` | HPHYS0219 amendment: corrected WB19 `drfc` coefficient-family authority from `cpm_####` to baseline-authoritative `coca_####` and retained fail-closed guard posture for `coca` domain violations. |
| `2026-05-31` | `12` | `Codex` | HPHYS0221 amendment: added WB19 `solwpv` branch semantics and water-yield/saturated-depth coupling authority (`avpora`, `avfca`, `avcoca`, `watyld`, `fcdep`, `unsdep`) with required WB19 coupling writebacks and fail-closed domain posture. |
