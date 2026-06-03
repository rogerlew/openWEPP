---
contract_id: SC-SUBHYD-001
title: Subsurface Hydrology and Drainage Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 26
producer_scope:
  - Daily subsurface lateral-flow flux surfaces from drainable-layer states
  - Surface depressional-storage and artificial-drainage flux surfaces
  - Subsurface coupling surfaces exported to daily closure and watershed routing
consumer_scope:
  - Daily water-balance accounting consumers
  - Watershed/channel routing consumers using subsurface and drainage contributions
  - Comparator/replay surfaces using daily closure confidence signals
evidence_level: static
last_reviewed: 2026-06-02
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
| REF-SUBHYD-LEGACY-DAILY-LATERAL | `/workdir/wepp-forest_260430_baseline/src/watbal.for:286-304,573-704` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline daily WB19 lateral flow computes `hk`, `fzdrfc`, `fzul`, daily `solwpv` branch selection, conductivity weighting, and daily `latqcc` publication. | `[DIRECT][Static]` |
| REF-SUBHYD-LEGACY-HOURLY-TAIL | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:592-887` and `/workdir/wepp-forest_260430_baseline/src/drain.for:181-305` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline hourly tail executes drainage, lateral flow, top-layer saturation clipping into `ui_SCrunf(ii)`, and copy-forward before daily runoff/storage publication. | `[DIRECT][Static]` |
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
| Layer hydrology state family | `nsl`, `solthk`, `solwpv`, `dg_####`, `por_####`, `coca_####`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`, optional frozen-water `wb18_perc_frzw_####` |
| Lateral geometry + conductivity family | `avgslp`, `slplen`, `wb19_lateral_anisotropy_ratio` |
| Drainage geometry + capacity family | `wb19_drain_enabled`, `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter`, `wb11_drainage_coefficient` |
| Coupling carry-forward surface | `Pe` |

### Required Outputs

| Surface | Output |
|---|---|
| Lateral flux output | `q` |
| Drainage outputs | `Qdd`, `Qd` |
| MOFE hourly carry outputs | `ui_LfCrf_####`, `ui_SCrunf_####` |
| Lateral/drainage state updates | `wb11_drainable_storage`, `wb18_perc_theta_####`, `wb19_fcdep`, `wb19_unsdep`, `wb19_watyld` |
| Scheduler/kernel failure surface | Typed hard-fail status for missing/non-finite/out-of-range lateral/drainage domains |

### Mutated State Surfaces

WB19 mutates lateral/drainage boundary surfaces deterministically:
- lateral phase computes Eq. [6.2.4]-derived `q`, withdraws layer water above
  field capacity from top to bottom, updates `wb18_perc_theta_####`, updates
  `wb11_drainable_storage`, publishes coupled saturated-depth surfaces
  (`wb19_fcdep`, `wb19_unsdep`, `wb19_watyld`), and in MOFE hourly lanes emits
  `ui_LfCrf(ii)` plus `ui_SCrunf(ii)` top-layer saturation clipping.
- drainage phase computes Eq. [6.2.10]-[6.2.11]-derived `Qdd` with explicit
  equivalent-depth branch + capacity cap, performs tile-layer-to-surface
  withdrawal, updates `wb18_perc_theta_####`, updates
  `wb11_drainable_storage`, and emits `Qd = q + Qdd` when both same-pass
  components are available.

## Algorithm Specification (WB19 Lateral/Drainage Production Execution)

1. Lateral phase loads WB18 per-layer states (`theta`, `fc`, `ul`, `ssc`,
   `dg`, `por`, `coca`) and computes saturated-zone metrics and effective
   conductivity over saturated thickness:
   - `drfc_i = fc_i + (1-coca_i)*dg_i`
   - `fzdrfc_i = max(drfc_i - frzw_i, 0)` where `frzw_i` is the frozen-water
     storage surface for layer `i`; absent `frzw_i` is the explicit no-frozen
     storage state for lanes that have not activated frost carry surfaces.
   - HPHYS0256 supersedes unqualified lane wording: daily lanes
     (`wb19_lateral_drain_lane_substeps = 1`) follow `INV-SUBHYD-026`
     baseline `watbal.for` lateral authority, while hourly lanes
     (`wb19_lateral_drain_lane_substeps = 24`) follow `INV-SUBHYD-024` and
     `INV-SUBHYD-025` baseline `watbal_hourly.for` authority.
   - `avpora = Σ(por_i * dg_i / fcdep)`,
     `avfca = Σ(thetfc_i * dg_i / fcdep)`,
     `avcoca = Σ(coca_i * dg_i / fcdep)`
   - `watyld = avpora - (avfca + (1-avcoca))`
   - `Ke = 86400 * (Σ(ssc_i * fffx_i * dg_i) / Σ(dg_i))`
     for `solwpv >= 2006`; for `solwpv < 2006`, baseline `watbal_hourly`
     applies the legacy post-aggregation multiplier by the final active-layer
     `fffx` retained from the saturated-layer loop.
   - `alpha = atan(avgslp)`
   - `q_potential = (Ho * wb19_lateral_anisotropy_ratio * Ke * sin(alpha)) / slplen`
2. Lateral phase caps and withdraws `q` from layer excess water above
   `fzdrfc_i` in top-to-bottom sequence and emits actual `q` after residual
   withdrawal reduction; hourly conductivity weighting still uses unfrozen
   `drfc_i` in `fffx = (st_i-drfc_i)/(ul_i-drfc_i)` per
   `watbal_hourly.for:695-717`. For `solwpv < 2006`, update
   `fcdep = max(fcdep - q/watyld, 0)` and `unsdep = soldep - fcdep`.
3. Drainage phase (when `wb19_drain_enabled = 1`) computes Eq. [6.2.10]-[6.2.11]
   branch values using `wb19_drain_depth`, `wb19_drain_spacing`,
   `wb19_drain_diameter`, water-table depth from saturated-layer state, and
   effective saturated conductivity. Emitted `Qdd` is capped by
   `wb11_drainage_coefficient`.
4. Drainage phase withdraws `Qdd` from tile-layer-to-surface excess-water
   sequence. In compatibility daily/direct lanes where same-pass `q` already
   exists, drainage may emit total subsurface loss `Qd = q + Qdd`.
5. HPHYS0242 hourly-lane ordering uses the baseline hourly tail: drainage
   emits `Qdd` before lateral; lateral then emits final `Qd = Qdd + q`,
   `ui_LfCrf(ii) = q(ii)`, and `ui_SCrunf(ii)` after clipping top-layer
   saturation excess from `st(1)` against `fzul = ul(1) - frzw(1)`.
6. Reject missing, non-finite, or out-of-range lateral/drainage domains with
   typed hard-fail status; no silent fallback/clamping paths are permitted.

## Branch and Guard Table (WB19 Lateral/Drainage Kernels)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-SUBHYD-WB19-LATERAL-EXECUTE` | phase class `hydrology_lateral_transfer` | `nsl`, `solthk`, `solwpv`, `dg_####`, `por_####`, `coca_####`, `thetfc_####`, `thetdr_####`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`, `avgslp`, `slplen`, `wb19_lateral_anisotropy_ratio`, `Pe` | runtime | deterministic layer-aware lateral execution/writeback |
| `BR-SUBHYD-WB19-DRAIN-EXECUTE` | phase class `hydrology_drainage` | WB19 lateral symbols + `wb19_drain_enabled`, `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter`, `wb11_drainage_coefficient`; same-pass `q` is required only for compatibility lanes where drainage publishes final `Qd` | runtime | deterministic layer-aware drainage execution/writeback |
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
| INV-SUBHYD-017 | WB19 layer-pool available-cap invariant: lateral/drainage available-pool caps are derived from active per-layer drainable storage (`Σ max(theta_i - drfc_i, 0)`) and must not be expanded by legacy compatibility scalar reconciliation (`max(layer_pool, legacy_term)`). | hard-fail | REF-SUBHYD-CH6-LATSTOR, REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-CH6-DRAINFLOW, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-018 | WB19 saturated-thickness response invariant: under fixed conductivity/geometry/forcing domains, increasing saturated thickness (and corresponding layer-derived available pool) must not decrease realized lateral flux (`q_high >= q_low`) and should increase it when neither case is constrained by non-saturated zero-flow branches. | hard-fail | REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-019 | WB19 FC/WP + COCA coupling invariant: water-yield coupling must compute `avfca` from `thetfc_####` theta lineage (not `wb18_perc_fc_####/dg_####` surrogate), enforce per-layer consistency `wb18_perc_fc_#### = (thetfc_####-thetdr_####)*dg_####`, and apply `solwpv < 2006` `fcdep` mutation using this authoritative `watyld` branch. | hard-fail | REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-020 | WB19 hourly iterative lane invariant: when hourly lane is active (`wb19_lateral_drain_lane_substeps = 24`), WB19 lateral/drainage execution must iterate substeps with state recomputation each substep and accumulate realized daily `q` and `Qdd`; divisor-only single-pass substitution is invalid. | hard-fail | REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-CH6-DRAINFLOW, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-021 | HPHYS0239 WB19->WB12/WB13 handoff invariant: WB19 lateral/drainage handoff must remain deterministic and downstream WB12/WB13 consumers must consume post-WB19 same-pass flux symbols with anti-shadow precedence for `q`/`Qdd`/`Qd` under state/flux conflicts. HPHYS0242 `INV-SUBHYD-023` is the controlling authority for hourly-lane drainage/lateral order. | hard-fail | REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-CH6-DRAINFLOW, REF-SUBHYD-CH5-COUPLING, REF-SUBHYD-PHYS-BOUNDS, INV-SUBHYD-023 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-022 | HPHYS0240 runoff-carryover handoff invariant: downstream WB12/WB14 reconciliation in the post-WB19 hydrology tail must consume same-pass `wb12_runoff_carryover` flux for incoming runoff/runon carryover when present, preserving flux-authoritative anti-shadow semantics and finite non-negative boundary validation before storage reconciliation consumes derived `Q`. | hard-fail | REF-SUBHYD-CH5-COUPLING, REF-SUBHYD-CH13-COUPLING, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-023 | HPHYS0242 hourly WB19 tail invariant: hourly-lane WB19 execution must follow baseline tail ordering `Drainage -> LateralTransfer`, accumulate same-pass `Qdd`, `q`, and final `Qd = Qdd + q`, publish 24-slot `ui_LfCrf` from realized lateral flow, clip positive top-layer saturation excess into 24-slot `ui_SCrunf`, and leave no material post-clipping top-layer excess. Missing/malformed carry arrays, stale `Qd`, or omitted positive `ui_SCrunf` hard-fail. | hard-fail | REF-SUBHYD-LEGACY-HOURLY-TAIL, REF-SUBHYD-CH6-DRAINFLOW, REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS, SC-WATBAL-001#INV-WATBAL-034, SC-RUNOFFPART-001#INV-RUNOFFPART-014 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-024 | HPHYS0247 WB19 lateral saturated-zone lineage invariant: WB19 lateral execution must follow baseline `watbal_hourly` unfrozen conductivity-layer selection for hourly closure claims: a layer is conductivity-active only when `st(i) >= drfc(i)` and either it is the bottom layer or the layer below is saturated to `ul(i+1)` (`meblfc`), and conductivity averaging uses `fffx = clamp((st(i)-drfc(i))/(ul(i)-drfc(i)),0,1)` as the per-layer saturation fraction. For `solwpv < 2006`, the legacy branch applies the baseline post-aggregation multiplier by the final conductivity-active-layer `fffx`; for `solwpv >= 2006`, that second multiplier is not applied. Top-contiguous-only selection, FC-only thresholds, omitted `fffx`, or omitted legacy `solwpv < 2006` post multiplier are invalid. HPHYS0252 `INV-SUBHYD-025` supersedes capacity and withdrawal caps when frozen water `frzw(i)` is present. | hard-fail | REF-SUBHYD-LEGACY-HOURLY-TAIL, REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-025 | HPHYS0252 WB19 frozen-adjusted lateral storage invariant: WB19 lateral capacity and top-down withdrawal floors must follow baseline `watbal_hourly` `fzdrfc(i) = max(drfc(i)-frzw(i),0)` lineage. Lateral capacity-active layers use `st(i) >= fzdrfc(i)` plus the same bottom-contiguous `meblfc` condition, `fcdep`/`tdvv` are assembled from those capacity-active layers, `tdvv = Σ active max(st(i)-fzdrfc(i),0)`, and realized lateral withdrawal must not reduce `st(i)` below `fzdrfc(i)`. The hourly conductivity loop remains governed by `INV-SUBHYD-024` unfrozen `drfc(i)` `fffx` weighting. Negative/non-finite `frzw(i)` or hidden replacement of absent frost carry with a non-zero surrogate is invalid. | hard-fail | REF-SUBHYD-LEGACY-HOURLY-TAIL, REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SUBHYD-026 | HPHYS0256 WB19 daily lateral lane invariant: daily lateral execution (`wb19_lateral_drain_lane_substeps = 1`) must follow baseline `watbal.for` daily authority rather than hourly `meblfc` authority. For `solwpv >= 2006`, all layers with `st(i) >= fzdrfc(i)` contribute to `fcdep`, `tdvv`, and conductivity without a `meblfc` gate; conductivity uses `fzul(i)=ul(i)-frzw(i)`, `sstz=st(i)/fzul(i)` when `fzul(i)>0` else `1`, `hk(i)=-2.655/log10(fc(i)/ul(i))` for positive ratios, and `fffx=max(sstz**hk(i),0.002)` when `sstz<0.95` else `1`. For `solwpv < 2006`, the active block is top-contiguous above `fzdrfc(i)`, and the post-aggregation `fffx=max((avstt/avul)**avhk,0.002)` applies when `avul>0.001` and `sstz<0.95`. Hourly lanes remain governed by `INV-SUBHYD-024`/`INV-SUBHYD-025`; applying hourly `meblfc` selection or hourly unfrozen-`drfc` conductivity to daily lanes is invalid. | hard-fail | REF-SUBHYD-LEGACY-DAILY-LATERAL, REF-SUBHYD-LEGACY-HOURLY-TAIL, REF-SUBHYD-CH6-LATFLUX, REF-SUBHYD-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| `INV-SUBHYD-017` | runtime + external-authority | WB19 available-pool authority validator plus Level-4 constitutive suite checks | Hard-fail when available-pool authority is expanded via legacy max-reconciliation instead of layer-derived `Σ max(theta_i-drfc_i,0)` cap | Tier-A gate + required A3 lane | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-018` | runtime + external-authority | WB19 saturated-thickness response behavioral validator plus Level-4 constitutive suite checks | Hard-fail when increased saturated thickness under fixed drivers fails to produce non-decreasing lateral flux response | Tier-A gate + required A3 lane | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-019` | runtime + external-authority | WB19 FC/WP theta-lineage coupling validator plus Level-4 constitutive suite checks | Hard-fail when `avfca`/`watyld` uses FC-store surrogate lineage or FC-store/theta lineage is inconsistent | Tier-A gate + required A3 lane | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-020` | runtime | WB19 hourly lane iterative execution validator across lateral/drainage phases | Hard-fail when hourly lane behavior collapses to single-pass divisor-only execution without per-substep state recomputation and accumulated daily flux publication | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-021` | runtime + governance | WB19-to-WB12/WB13 handoff validator for deterministic `q`/`Qdd`/`Qd` sequencing and anti-shadow consumption | Typed hard error / explicit `HOLD` when downstream reconciliation/publication consumes stale pre-WB19 surfaces or state-shadowed subsurface symbols | HPHYS hourly handoff closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-022` | runtime + governance | WB12/WB14 runoff-carryover handoff validator at post-WB19 tail boundary | Typed hard error / explicit `HOLD` when carryover uses stale `wb12_runon_input` despite present same-pass `wb12_runoff_carryover`, or when malformed carryover reaches storage-derived `Q` | HPHYS hourly carryover closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-023` | runtime + governance | HPHYS0242 scheduler-order gate, drainage/lateral same-pass `Qd` assembler, and MOFE current carry-array producer | Typed hard error / explicit `HOLD` when hourly tail runs lateral before drainage, publishes stale `Qd`, omits `ui_LfCrf`/`ui_SCrunf`, or leaves unclipped material top-layer saturation excess | HPHYS cadence/order closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-024` | runtime + governance | WB19 lateral saturated-zone selector, `tdvv` capacity cap, `fffx` conductivity weighting, and legacy `solwpv < 2006` post multiplier | Typed hard error / explicit `HOLD` when lateral flow is produced from non-`meblfc` layers, omits saturation-fraction weighting or the legacy post multiplier, or withdraws beyond active-layer `tdvv` | HPHYS0247 H39 lateral residual closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-025` | runtime + governance | WB19 frozen-adjusted lateral capacity and withdrawal threshold lineage | Typed hard error / explicit `HOLD` when frozen-water `frzw(i)` does not lower `tdvv` floors through `fzdrfc(i)`, conductivity substitutes `fzdrfc(i)` for `drfc(i)` `fffx`, or withdrawal reduces `st(i)` below `fzdrfc(i)` | HPHYS0252 H39 storage-availability closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SUBHYD-026` | runtime + governance | WB19 daily lateral lane selector, `hk` conductivity weighting, `fzdrfc`/`fzul` availability, and daily `solwpv` branch behavior | Typed hard error / explicit `HOLD` when daily lanes use hourly `meblfc` selection, omit daily `hk`/`fzul` conductivity weighting, or collapse daily and hourly lateral semantics into one branch | HPHYS0256 `latqcc` lane-branch closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols follow Chapter-6 WEPP notation with explicit WB18/WB19 runtime
alias continuity for production kernels.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `θ_i` | `wb18_perc_theta_####` | WB19 per-layer moisture state surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `FCi` | `wb18_perc_fc_####` | WB19 per-layer FC storage-above-residual surfaces used by `drfc_i` threshold lineage | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `θFC_i` | `thetfc_####` | WB19 per-layer field-capacity theta lineage used in `avfca` water-yield coupling | dimensionless preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `θDR_i` | `thetdr_####` | WB19 per-layer residual theta lineage used for FC/WP consistency checks against `wb18_perc_fc_####` | dimensionless preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `θUL_i` | `wb18_perc_ul_####` | WB19 per-layer upper-limit surfaces used in branch coupling checks | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `K_i` | `wb18_perc_ssc_####` | WB19 per-layer saturated conductivity surfaces | `m s^-1` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `dg_i` | `dg_####` | WB19 per-layer thickness surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `por_i` | `por_####` | WB19 per-layer porosity surfaces used in water-yield coupling | dimensionless preserved (`0 < por <= 1`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `coca_i` | `coca_####` | WB19 entrapped-air correction surfaces used by drain-threshold lineage | dimensionless preserved (`0 < coca <= 1`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `drfc_i` | `wb18_perc_fc_#### + (1-coca_####)*dg_####` | WB19 drain-threshold lineage used for saturated-zone classification and withdrawals | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `frzw_i` | `wb18_perc_frzw_####` | WB19 frozen-water storage surface used to lower lateral capacity/withdrawal threshold through `fzdrfc_i` | `m` preserved; absent means explicit zero frozen storage for non-frost lanes | `[DIRECT][Static] + [INFERENCE][Static]` |
| `fzdrfc_i` | `max(drfc_i-frzw_i,0)` | Baseline frozen-adjusted WB19 lateral capacity and top-down withdrawal floor | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| `ui_LfCrf(ii)` | `ui_LfCrf_{hour:04}` | realized current-OFE hourly lateral-flow carry after withdrawal caps | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_SCrunf(ii)` | `ui_SCrunf_{hour:04}` | realized current-OFE hourly top-layer saturation excess after clipping from `st(1)` | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| WB19 baseline saturated-zone capacity (`INV-SUBHYD-024`) | lateral-flow conductivity layer selection and weighting | Hard error on top-contiguous-only saturated selection, omitted `fffx` weighting, or omitted legacy `solwpv < 2006` post multiplier | HPHYS0247 hourly closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB19 frozen-adjusted storage availability (`INV-SUBHYD-025`) | lateral-flow capacity cap and withdrawal writeback | Hard error on omitted `fzdrfc` threshold lineage, non-finite/negative `frzw`, conductivity using `fzdrfc` instead of `drfc`, or withdrawal below `fzdrfc` | HPHYS0252 hourly storage-availability gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB19 daily lateral lane authority (`INV-SUBHYD-026`) | lateral-flow lane selector and daily conductivity weighting | Hard error on applying hourly `meblfc`/unfrozen-`drfc` conductivity to daily lanes or omitting baseline daily `hk`/`fzul` weighting | HPHYS0256 daily `latqcc` closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| Runoff carryover available to runoff/storage tail checks | `wb12_runoff_carryover` |
| WB12 storage reconciliation outputs | `wb12_storage_reconciled`, `wb12_storage_closure_delta` |

### WB12 Coupling Requirements

1. `Qd` exported from WB19 lateral/drainage phases remains the required subsurface-loss term consumed by WB12 storage reconciliation.
2. WB12 storage reconciliation must treat `Qd` as a non-negative loss magnitude in closure diagnostics.
3. Missing/non-finite `Qd` at storage reconciliation boundaries is an invalid runtime state and must hard-fail with typed WB12 storage guard codes.
4. When same-pass `wb12_runoff_carryover` is present, WB12/WB14 runoff
   reconciliation must use it before storage reconciliation consumes derived
   `Q`; stale `wb12_runon_input` state may only serve compatibility mode when
   the carryover flux is absent.

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
3. WB13 subsurface publication must consume flux-authoritative `q`, `Qdd`, and
   `Qd` symbols when both state and flux surfaces publish the same symbol.
4. Missing/non-finite/out-of-domain subsurface/drainage WB13 symbols are
   invalid runtime states and must hard-fail with WB13 typed guard posture.
5. WB13 subsurface publication must execute only after WB19 lateral/drainage
   phases for the same day-pass have finalized `q`/`Qdd`/`Qd` handoff symbols.

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

## HPHYS0234 WB13 Subsurface Flux-Authority Anti-Shadow Addendum

1. WB13 `latqcc`, `Tile`, and `Qd` publication-coupling symbols are
   flux-authoritative under symbol conflicts:
   - `q` over state duplicate,
   - `Qdd` over state duplicate,
   - `Qd` over state duplicate.
2. WB13 `Qd = latqcc + Tile` coupling checks must be evaluated from the same
   flux-authoritative symbol family.
3. State duplicate symbols may remain for seam continuity but are
   non-authoritative for WB13 subsurface publication under conflict.
4. Contract-derived vectors must include stale-state/flux-conflict probes and
   verify flux-preferred publication outcomes.

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

1. WB19 lateral execution originally carried provisional `solwpv` selector
   wording, but HPHYS0247 supersedes lateral-active layer selection with
   baseline `meblfc` bottom-contiguous authority from `INV-SUBHYD-024`.
2. WB19 lateral coupling must compute:
   - `avpora = Σ(por_i * dg_i / fcdep)`,
   - `avfca = Σ(thetfc_i * dg_i / fcdep)`,
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

1. WB19 lateral saturated-layer selection is superseded by HPHYS0247
   `INV-SUBHYD-024` baseline `meblfc` authority.
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

## HPHYS0225 WB19 Layer-Pool Available-Cap Authority Addendum

1. WB19 available-pool caps for both lateral (`q`) and drainage (`Qdd`) are
   derived from active per-layer state only:
   - `layer_pool = Σ max(theta_i - drfc_i, 0)`.
2. Legacy compatibility scalar `wb11_drainable_storage` may remain in phase
   input schema for seam continuity but is non-authoritative for expanding WB19
   available-pool caps.
3. Runtime reconciliation patterns of the form
   `available_pool = max(layer_pool, legacy_term)` are prohibited.
4. External-authority constitutive suite
   `cas_l4_subhyd_layer_pool_withdrawal_cap_001` is required/hard-fail and
   linked to `INV-SUBHYD-017`.

## HPHYS0226 WB19 Lateral Saturated-Thickness Response Addendum

1. For constitutive lanes where slope, anisotropy, conductivity, `solwpv`
   branch, and forcing are held fixed, increasing saturated thickness (and its
   derived available pool) must not decrease realized lateral flux.
2. Behavioral authority is evaluated by paired component fixtures that differ
   only in saturated-thickness-driving state symbols.
3. External-authority constitutive suite
   `cas_l4_subhyd_lateral_saturated_thickness_response_001` is
   required/hard-fail and linked to `INV-SUBHYD-018`.

## HPHYS0227 WB19 FC/WP + COCA Water-Yield Coupling Addendum

1. WB19 coupling must compute:
   - `avpora = Σ(por_i * dg_i / fcdep)`,
   - `avfca = Σ(thetfc_i * dg_i / fcdep)`,
   - `avcoca = Σ(coca_i * dg_i / fcdep)`,
   - `watyld = avpora - (avfca + (1-avcoca))`.
2. WB19 lateral kernels must require and validate per-layer FC/WP consistency:
   - `wb18_perc_fc_i = (thetfc_i - thetdr_i) * dg_i`
   with typed hard-fail posture when violated.
3. This coupling authority binds `coca_####` threshold lineage and FC/WP theta
   lineage into a single constitutive branch for `solwpv < 2006` `fcdep`
   mutation.
4. External-authority constitutive suite
   `cas_l4_subhyd_watyld_fcwp_consistency_001` is required/hard-fail and
   linked to `INV-SUBHYD-019`.

## HPHYS0238 WB19 Hourly Iterative Lateral/Drainage Addendum

1. Hourly-lane WB19 execution must be explicit and state-iterative:
   - iterate `lane_substeps = wb19_lateral_drain_lane_substeps`,
   - recompute saturated-state, conductivity, and branch quantities each
     substep from mutated layer state,
   - accumulate realized daily `q` and `Qdd` across substeps.
2. `wb19_lateral_drain_lane_substeps` is required to be a positive integral
   scalar when published; runtime absence may default to `1` for backward
   compatibility in direct kernel tests, but production seeding must publish the
   symbol explicitly for daily/hourly lanes.
3. For drainage, daily capacity (`wb11_drainage_coefficient`) remains a
   cumulative-day cap across substeps (`ΣQdd <= wb11_drainage_coefficient`).
4. Divider-only single-pass substitutions (for example, daily potential divided
   by `24` without per-substep state recomputation) are non-authoritative.
5. Contract-derived tests must include hourly-vs-daily lane vectors showing
   behavior divergence under identical forcing/state inputs.

## HPHYS0239 WB19->WB12/WB13 Handoff Ordering Addendum

1. WB19 handoff sequencing is explicit and deterministic:
   - same-pass `q`, `Qdd`, and `Qd = q + Qdd` must be published before WB12
     and WB13 consumers read the hydrology tail,
   - HPHYS0242 refines hourly-lane WB19 order to baseline
     `Drainage -> LateralTransfer`; older lateral-before-drainage wording is
     compatibility-only and is not controlling for hourly lanes.
2. WB12 storage reconciliation and WB13 subsurface publication consume the
   post-WB19 same-pass symbols and must not reuse stale pre-WB19 state copies.
3. Under state/flux symbol conflicts, WB12/WB13 coupling consumers are
   flux-authoritative for `q`, `Qdd`, and `Qd`; state duplicates are
   non-authoritative under conflict.
4. Contract-derived vectors must assert both handoff ordering and
   stale-state/flux-conflict anti-shadow behavior for WB19->WB12/WB13
   interfaces.

## HPHYS0240 Hourly Runoff Carryover Handoff Addendum

1. The post-WB19 hydrology tail must preserve same-pass runoff carryover via
   flux `wb12_runoff_carryover` when that boundary is present.
2. WB12/WB14 runoff reconciliation is flux-authoritative for carryover under
   state/flux conflicts; `wb12_runon_input` is a compatibility input only when
   the same-pass carryover flux is absent.
3. Storage reconciliation consumes `Q` derived from the resolved carryover, so
   malformed carryover must hard-fail before storage closure writeback.
4. Contract-derived vectors must include carryover flux-over-state and
   malformed-carryover rejection probes across the runoff/storage tail.

## HPHYS0242 Hourly Drainage/Lateral/Saturation Tail Addendum

1. Baseline hourly tail order is authoritative for hourly lanes:
   `Drainage -> LateralTransfer -> top-layer saturation clipping -> copy-forward`.
2. Drainage publishes same-pass `Qdd`; lateral publishes realized same-pass `q`
   and then final `Qd = Qdd + q` for WB12 storage closure.
3. MOFE hourly lanes publish `ui_LfCrf(ii)` from realized lateral flow and
   `ui_SCrunf(ii)` from top-layer saturation excess after enforcing
   `st(1) <= fzul = ul(1) - frzw(1)` within declared tolerance.
4. Positive `ui_SCrunf(ii)` is a required production output and must not be
   represented only as an aggregate daily carryover or silently left in layer
   storage.
5. Contract-derived vectors must prove hourly order, final `Qd` freshness,
   positive `ui_SCrunf` clipping, malformed carry-array rejection, and stale
   state/flux anti-shadow behavior.

## HPHYS0247 WB19 Baseline Saturated-Zone Capacity Addendum

1. WB19 lateral conductivity-zone selection for hourly closure claims follows
   baseline `watbal_hourly` `meblfc` lineage:
   - `drfc(i) = fc(i) + (1-coca(i))*dg(i)`,
   - `meblfc(i) = 1` only for the bottom layer or when the layer below is
     saturated to `ul(i+1)`,
   - conductivity-active layer `i` requires both `st(i) >= drfc(i)` and
     `meblfc(i) = 1`.
2. HPHYS0252 supersedes the capacity/withdrawal threshold by requiring
   `fzdrfc(i)` where frozen water is present; the broader layer-pool cap
   remains non-authoritative for expanding lateral withdrawal beyond `tdvv`.
3. Lateral effective conductivity averaging uses
   `fffx(i) = clamp((st(i)-drfc(i))/(ul(i)-drfc(i)),0,1)` and accumulates
   `Σ Ksat(i)*fffx(i)*dg(i)` over conductivity-active layers.
4. For `solwpv < 2006`, baseline `watbal_hourly` applies a post-aggregation
   multiplier by the final active-layer `fffx` retained from the active-layer
   loop before computing lateral flux; `solwpv >= 2006` omits this second
   multiplier.
5. `ul(i)-drfc(i) <= 0`, non-finite `fffx`, or any attempted lateral
   withdrawal beyond `tdvv` is a typed WB19 domain violation, not a silently
   repaired state.
6. Contract-derived vectors must prove non-bottom-contiguous top saturation
   does not emit lateral flow and partially saturated active layers are damped
   by `fffx`, including the legacy `solwpv < 2006` post-aggregation multiplier
   branch.

## HPHYS0252 WB19 Frozen-Adjusted Lateral Storage Addendum

1. WB19 lateral storage availability for hourly closure claims follows baseline
   `watbal_hourly` frozen-water threshold lineage:
   - `drfc(i) = fc(i) + (1-coca(i))*dg(i)`,
   - `fzdrfc(i) = max(drfc(i)-frzw(i),0)`,
   - capacity-active layer `i` requires both `st(i) >= fzdrfc(i)` and
     `meblfc(i) = 1`.
2. Active-layer capacity cap is `tdvv = Σ max(st(i)-fzdrfc(i),0)` over
   capacity-active layers only, matching the legacy `tdvv` and drawdown floor
   lineage in `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`.
3. Realized lateral withdrawal must use the same top-to-bottom sequence as the
   baseline tail and must not reduce any layer below `fzdrfc(i)`.
4. Hourly conductivity averaging is intentionally not changed by frozen-water
   storage: `INV-SUBHYD-024` still uses unfrozen `drfc(i)` for the `fffx`
   denominator and conductivity-active layer condition.
5. Absent `wb18_perc_frzw_####` means explicit zero frozen storage only for
   lanes without an active frost carry state. Present `frzw(i)` must be finite
   and non-negative; material domain violations are typed WB19 hard failures.
6. Contract-derived vectors must prove frozen storage expands `tdvv` and the
   withdrawal floor through `fzdrfc(i)` while preserving unfrozen `drfc(i)`
   conductivity weighting.

## HPHYS0256 WB19 Daily Lateral Lane-Branch Addendum

1. Daily and hourly WB19 lateral lanes are distinct baseline branches:
   - daily lanes use `/workdir/wepp-forest_260430_baseline/src/watbal.for`,
   - hourly lanes use `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`.
2. For daily `solwpv >= 2006`, lateral active layers are all layers satisfying
   `st(i) >= fzdrfc(i)`; hourly `meblfc` bottom-contiguous selection is not
   applied. Conductivity weighting uses `fzul(i)=ul(i)-frzw(i)` and the
   baseline `hk(i)` exponent from `fc(i)/ul(i)`.
3. For daily `solwpv < 2006`, lateral active layers are the top-contiguous
   block satisfying `st(i) >= fzdrfc(i)` before the first inactive layer, and
   the final `fffx` multiplier is computed from `avstt`, `avul`, and `avhk`.
4. Daily `latqcc` uses `latk = 86400*(totk/totdg)` for `solwpv >= 2006` and
   `latk = 86400*(totk/totdg)*fffx` for `solwpv < 2006`; hourly lanes retain
   the `3600`-second per-substep authority expressed in openWEPP as
   `86400 / wb19_lateral_drain_lane_substeps`.
5. Contract-derived tests must prove that a daily `solwpv >= 2006` layer above
   `fzdrfc` can emit lateral flow without the hourly `meblfc` gate, while the
   same state under hourly substeps remains governed by hourly `meblfc`.

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
| `2026-06-02` | `27` | `Codex` | HPHYS0256 amendment: added `INV-SUBHYD-026` and baseline daily `watbal.for` WB19 lateral lane authority, distinguishing daily `fzdrfc`/`fzul`/`hk` conductivity weighting from hourly `meblfc` authority. |
| `2026-06-02` | `26` | `Codex` | HPHYS0252 amendment: added `INV-SUBHYD-025` and baseline `watbal_hourly` frozen-adjusted WB19 lateral storage authority (`fzdrfc = max(drfc-frzw,0)`) for capacity caps and top-down withdrawal floors while retaining unfrozen `drfc` conductivity weighting. |
| `2026-06-02` | `25` | `Codex` | HPHYS0247 amendment: added `INV-SUBHYD-024` and baseline `watbal_hourly` WB19 saturated-zone capacity authority (`meblfc`, active-layer `tdvv`, `fffx` conductivity weighting, and legacy `solwpv < 2006` post multiplier) for H39 hourly lateral closure. |
| `2026-06-01` | `24` | `Codex` | HPHYS0242 amendment: added `INV-SUBHYD-023`, baseline hourly `Drainage -> LateralTransfer` tail authority, final same-pass `Qd = Qdd + q` publication, and required `ui_LfCrf`/`ui_SCrunf` MOFE current carry production with top-layer saturation clipping. |
| `2026-06-01` | `23` | `Codex` | HPHYS0240 amendment: added `INV-SUBHYD-022` and carryover handoff addendum requiring post-WB19 WB12/WB14 reconciliation to consume same-pass `wb12_runoff_carryover` before compatibility `wb12_runon_input`, with malformed-carryover hard-fail posture before storage closure. |
| `2026-06-01` | `22` | `Codex` | HPHYS0239 amendment: added `INV-SUBHYD-021` and handoff-ordering addendum codifying deterministic WB19 `q`/`Qdd`/`Qd` sequencing plus WB12/WB13 anti-shadow consumption requirements for same-pass handoff symbols. |
| `2026-06-01` | `21` | `Codex` | HPHYS0238 amendment: added `INV-SUBHYD-020` and hourly iterative WB19 lateral/drainage addendum requiring per-substep state-recompute accumulation (`wb19_lateral_drain_lane_substeps`) and prohibiting divisor-only single-pass substitutions. |
| `2026-06-01` | `20` | `Codex` | HPHYS0234 amendment: added WB13 subsurface anti-shadow authority requiring flux-preferred publication/coupling for `q`, `Qdd`, and `Qd` under state/flux symbol conflicts, with explicit conflict-probe vector obligations. |
| `2026-06-01` | `19` | `Codex` | HPHYS0227 amendment: added `INV-SUBHYD-019` FC/WP + COCA water-yield coupling authority, required FC-store/theta consistency guard, and Level-4 suite linkage `cas_l4_subhyd_watyld_fcwp_consistency_001`. |
| `2026-06-01` | `18` | `Codex` | HPHYS0226 amendment: added `INV-SUBHYD-018` saturated-thickness lateral-response behavioral authority and linked required Level-4 suite `cas_l4_subhyd_lateral_saturated_thickness_response_001`. |
| `2026-06-01` | `17` | `Codex` | HPHYS0225 amendment: added `INV-SUBHYD-017` layer-pool available-cap authority, prohibited WB19 legacy max-reconciliation expansion (`max(layer_pool, legacy_term)`), and linked required Level-4 suite `cas_l4_subhyd_layer_pool_withdrawal_cap_001`. |
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
