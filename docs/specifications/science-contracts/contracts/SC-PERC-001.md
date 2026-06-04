---
contract_id: SC-PERC-001
title: Percolation Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 25
producer_scope:
  - Layer-by-layer percolation flux surfaces from root-zone water storage states
  - Below-root-zone percolation-loss accounting surfaces used by daily closure
  - Percolation coupling surfaces consumed by subsurface/lateral-flow and drainage routines
consumer_scope:
  - Daily water-balance accounting consumers
  - Subsurface/drainage consumers that ingest percolation recharge terms
  - Comparator/replay surfaces using Tier-A daily closure confidence signals
evidence_level: Static
last_reviewed: 2026-06-04
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
| REF-PERC-LEGACY-SOILW | `/workdir/wepp-forest_260430_baseline/src/watbal.for:960-966`, `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1018-1025`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative aggregate soil-water recomputation after percolation/lateral/drainage uses `soilw(i) = st(i) + thetdr(i)*(dg(i)-frozen(i))` and `watcon = Σsoilw(i)`. | `[DIRECT][Static]` |
| REF-PERC-LEGACY-HOURLY-BOTK | `/workdir/wepp-forest_260430_baseline/src/perc.for:163-178,186-214`, `/workdir/wepp-forest_260430_baseline/src/purk.for:167-188`, `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:540-545`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative hourly bottom-layer restrictive seepage lineage: hourly bottom layer sets `meblfc=1` and forces `fx=1`, `perc` assigns bottom restrictive conductivity via `kslast`, computes thickness-weighted `sscz = (dg(i)+ui_bdrkth)/(dg(i)/ssc(i)+ui_bdrkth/ssc(i+1))`, `purk` mutates `st` and remembers bottom seepage as `sep/ui_LFtstp`, and `watbal_hourly` accumulates `deepSeep += sep`. | `[DIRECT][Static]` |
| REF-PERC-LEGACY-LOWER-SAT-CLAMP | `/workdir/wepp-forest_260430_baseline/src/perc.for:143-158,186-188`, `/workdir/wepp-forest_260430_baseline/src/purk.for:167-188`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative lower-layer saturation attenuation: non-bottom `stu = (st(i+1)+frzw(i+1))/ul(i+1)` is capped to `0.95` before `cr = sqrt(1-stu)` and before `sep = min(vv, 86400*fx*sscz*cr)`, so over-UL lower storage throttles percolation by `sqrt(0.05)` instead of causing a hard failure. | `[DIRECT][Static]` |
| REF-PERC-LEGACY-HOURLY-FIN | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:342-345,460-525`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative same-pass liquid input lineage: `fin = rain - interception + wmelt + irrigation` before runoff/runon adjustments, then hourly `xfin = fin/24 + carry` is added to `st(i)` starting at the top layer and distributed through `tillay(2)` before percolation/ET. | `[DIRECT][Static]` |
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
| `Ksbot` | `m s^-1` | Restrictive-layer saturated hydraulic conductivity used for bottom-layer seepage damping when restrictive layer is active (`slflag=1`). | soil restrictive-layer producer contract | bottom-layer effective conductivity branch (`perc.for`) |
| `Bbot` | `m` | Restrictive-layer thickness (`ui_bdrkth`) used by hourly bottom-layer thickness-weighted harmonic conductivity. | soil restrictive-layer producer contract | hourly bottom-layer `sscz` branch (`perc.for`) |
| `Ksai` | `m s^-1` | Adjusted hydraulic conductivity for layer `i`. | percolation routine | Eq. [5.4.2]-[5.4.3] routing |
| `Bi` | `fraction` | Conductivity-shape parameter controlling approach of `Ksai` toward near-zero at field capacity; legacy-authoritative runtime derivation in the active WB18 branch uses `Bi = -2.655/log10(FCi/ULi)` for `FCi/ULi > 0`, with explicit legacy-degenerate fallback `Bi = 0` for non-positive `FCi/ULi`. | percolation routine | Eq. [5.4.3]-[5.4.4] + legacy baseline `/workdir/wepp-forest_260430_baseline/src/watbal.for` |
| `Θi+1`, `ULi+1` | `m`, `m` | Lower-layer water-content state and upper limit used for percolation restriction term. | lower-layer state/parameters | Eq. [5.4.5] reduction factor |
| `D` | `m` | Cumulative percolation loss below root zone in daily water balance. | percolation-water-balance coupling | daily closure Eq. [5.1.1] |
| `Pe` | `m d^-1` | Percolated water into subsurface drainable layer. | percolation routine | subsurface continuity Eq. [6.2.1], [6.2.5] |
| `fin`, `xfin` | `m` | Same-pass liquid input and hourly layer-ingress increment from direct rain, routed snowmelt (`wmelt`), irrigation, and carry/runoff adjustments. | WB12/WB14 infiltration partition + snow/irrigation coupling | WB18 layer storage mutation before percolation |
| `tillay(2)` | `m` | Baseline tilled-layer depth controlling top-down `xfin` distribution into `st(i)`. | management initial-condition projection | infiltration-to-layer storage distribution |
| `soilw(i)` | `m` | Baseline per-layer aggregate unfrozen-water intermediate: `st(i) + thetdr(i)*(dg(i)-frozen(i))`. | WB18/WB11 aggregate recomputation | WB13 `Total-Soil` lineage |
| `watcon` | `m` | Baseline aggregate root-zone unfrozen water, `Σ soilw(i)`. | WB18/WB11 aggregate recomputation | WB13 `Total-Soil` lineage |
| `θ`, `θFC`, `θa` | `m^3 m^-3` | Total moisture, field-capacity moisture, and entrapped air defining drainable-water term in subsurface coupling. | subsurface state routine | drainable-layer storage accounting |

## Algorithm State Surfaces (WB18 Percolation Production Kernel)

### Required Inputs

| Surface | Symbols |
|---|---|
| Scheduler phase metadata | `phase_name`, `phase_class`, `consumer_adapter` |
| Percolation consumer-boundary state family | `wb11_nsl` (legacy `nsl` fallback), `wb19_thetdr`, `wb19_thetfc`, `ssc` |
| WB18 percolation state inputs | `wb11_soil_water`, `wb12_infiltration`/same-pass WB14 snowmelt forcing lineage when active routed melt is present, `management.initial.params.tillay2_m`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`, `wb19_thetdr_####`, `wb19_dg_####` (legacy generic `thetdr_####`/`dg_####` accepted for fixture compatibility), optional `wb18_perc_frozen_depth_####`, optional `wb18_perc_lane_substeps`, optional `slflag`, optional `kslast`, optional `ui_bdrkth` |

### Required Outputs

| Surface | Output |
|---|---|
| Percolation flux outputs | `D`, `Pe` |
| Percolation state updates | `wb11_soil_water`, `wb18_perc_theta_####` |
| Scheduler/kernel failure surface | Typed hard-fail status for missing/non-finite/out-of-range percolation domains |

### Mutated State Surfaces

WB18 mutates percolation boundary surfaces deterministically:
- state update: same-pass routed-snowmelt infiltration liquid (`fin`/`xfin`)
  is added to `wb18_perc_theta_####` before per-layer percolation routing when
  active WB14/WB12 snowmelt forcing is present.
- state update: per-layer moisture updates (`wb18_perc_theta_####`) are
  bottom-up routed by layer percolation flux `pei`.
- state update: `wb11_soil_water = Σ soilw(i)` after per-layer
  routing/writeback, where `soilw(i) = wb18_perc_theta_i +
  thetdr_i*(dg_i - frozen_i)`.
- flux update: `D = pei(bottom layer)` (daily deep-percolation loss exported
  below root zone).
- flux update: `Pe = D` for subsurface recharge coupling.

## Algorithm Specification (WB18 Percolation Production Execution)

1. Require finite per-layer WB18 symbols (`wb18_perc_theta_####`,
   `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`) for
   `1..nsl` and enforce declared domains.
2. When active routed snowmelt exists, apply baseline `fin/xfin` layer ingress
   before percolation routing for the snowmelt component:
   - obtain same-pass WB14/WB12 infiltration from the same routed-snowmelt,
     residual direct-rain, irrigation, interception, and Green-Ampt lineage
     used by runoff partition,
   - require finite non-negative infiltration,
   - distribute infiltrated water top-down into `st(i)`/`wb18_perc_theta_i`
     using `tillay(2)` where present; if `tillay(2) <= 0`, the first layer
     receives the remaining ingress per baseline branch behavior,
   - do not defer this water to WB12 storage reconciliation or WB13
     publication compensation.
3. Execute explicit per-layer field-capacity branch (`pei = 0` when
   `Θi <= FCi`; otherwise `pei > 0`), routing layers from bottom to top.
4. Compute conductivity-domain per-layer flow using Chapter-5 lineage plus
   baseline daily restrictive-layer branch authority:
   - saturation fraction `stz = Θi / ULi`
   - if `stz < 0.95`, derive per-layer conductivity-shape parameter from
     legacy-authoritative WB18 lineage:
     - `ratio_i = FCi / ULi`
     - if `ratio_i <= 0`, apply explicit legacy-degenerate branch `Bi = 0`
       (matching baseline `hk=0` fallback from `watbal.for`)
     - if `ratio_i > 0`, derive `Bi = -2.655 / log10(ratio_i)` and hard-fail
       non-finite/non-physical denominator domains
     - adjusted conductivity factor `fx = max(stz^Bi, 0.002)`
   - if `stz >= 0.95`, saturated branch bypasses dynamic-`Bi` derivation and
     sets `fx = 1`
   - per-layer effective conductivity:
     - default `Ksi_eff = Ksi`,
     - for daily lane (`wb18_perc_lane_substeps = 1`), bottom layer
       (`i = nsl`), and restrictive layer enabled (`slflag=1`), set
       `Ksi_eff = 2*Ksi*Ksbot/(Ksi+Ksbot)` using `Ksbot = kslast`,
     - for hourly lane (`wb18_perc_lane_substeps = 24`), bottom layer
       (`i = nsl`), set `fx = 1` per baseline `meblfc` branch, and when
       restrictive layer is enabled (`slflag=1`), set
       `Ksi_eff = (dg_i+Bbot)/(dg_i/Ksi + Bbot/Ksbot)` using
       `Bbot = ui_bdrkth` and `Ksbot = kslast`.
   - travel-capacity-limited layer flux
     `pei_pre = min(Θi - FCi, Δt * Ksi_eff * fx)`
   - lower-layer restriction for non-bottom layers computes
     `stu = Θi+1 / ULi+1`, caps `stu >= 0.95` to `0.95` per baseline
     `perc.for`, and applies `pei = pei_pre * sqrt(1 - stu_clamped)`;
     bottom layer exports `D` directly.
5. Resolve lane attenuation divisor `wb18_perc_lane_substeps`:
   - when symbol is absent, default to daily divisor `1`,
   - when symbol is present, require finite positive integral domain (`>= 1`).
6. Apply lane semantics using legacy `purk.for` / `watbal_hourly.for`
   authority:
   - daily lane (`wb18_perc_lane_substeps = 1`) executes one percolation pass
     and publishes `pei = pei_unscaled`,
   - hourly lane (`wb18_perc_lane_substeps = 24`) executes an explicit
     substep loop of length `24`; each substep recomputes `pei_unscaled` from
     current layer state and applies `pei_step = pei_unscaled / 24`,
   - hourly `D`/`Pe` publication is the bottom-layer accumulated seepage
     across all substeps in the day.
7. Recompute aggregate WB11 storage from baseline `soilw` semantics:
   - require `wb19_thetdr_####` and `wb19_dg_####` for each layer
     (legacy generic aliases accepted for older fixture vectors),
   - if `wb18_perc_frozen_depth_####` is present, require finite
     `0 <= frozen_i <= dg_i`; if absent, the active branch is the unfrozen-layer
     branch (`frozen_i = 0`) and cannot be used as evidence for frost-active
     parity closure,
   - compute `soilw(i) = wb18_perc_theta_i + thetdr_i*(dg_i - frozen_i)`,
   - publish `wb11_soil_water = Σsoilw(i)`.
8. Emit deterministic `D`/`Pe`, per-layer state updates, and aggregate
   `wb11_soil_water = Σsoilw(i)`.
9. Reject missing, non-finite, or out-of-range percolation domains with typed
   hard-fail status; no silent fallback/clamping paths are permitted.

## Branch and Guard Table (WB18 Percolation Kernel)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-PERC-WB18-EXECUTE` | phase class `hydrology_percolation_deep_seepage` | `wb11_nsl` (legacy `nsl` fallback), `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`, `wb19_thetdr_####`, `wb19_dg_####` (+ legacy generic fallback, optional `wb18_perc_frozen_depth_####`, optional `wb18_perc_lane_substeps`, optional `slflag`, optional `kslast`, optional `ui_bdrkth`) | runtime | deterministic per-layer percolation/writeback execution |
| `BR-PERC-WB11-MISSING` | required percolation symbol absent | percolation required symbols | runtime | typed hard-fail (`HKERNEL-WB11-PERC-E-001`) |
| `BR-PERC-WB11-NONFINITE` | percolation symbol is NaN/Inf | percolation required symbols | runtime | typed hard-fail (`HKERNEL-WB11-PERC-E-002`) |
| `BR-PERC-WB11-DOMAIN` | percolation symbol/derived flux outside domain bounds | percolation required + emitted symbols | runtime | typed hard-fail (`HKERNEL-WB11-PERC-E-003`) |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-PERC-001 | Field-capacity eligibility invariant: per layer, Eq. [5.4.1] branch semantics are explicit (`pei = 0` when `Θi <= FCi`; routing expression only when `Θi > FCi`). | hard-fail | REF-PERC-CH5-PERC | `[DIRECT][Static]` |
| INV-PERC-002 | Per-layer excess-water bound invariant: when `Θi > FCi`, emitted `pei` must be non-negative and not exceed the available excess-water term implied by Eq. [5.4.1]. | hard-fail | REF-PERC-CH5-PERC, REF-PERC-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-003 | Travel-time/conductivity-domain invariant: Eq. [5.4.2] and Eq. [5.4.3] usage requires finite positive routing domains (no undefined `ti`, `Ksai`, or layer-moisture terms in active percolation branch). | hard-fail | REF-PERC-CH5-PERC, REF-PERC-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-004 | Adjusted-conductivity formulation invariant: adjusted conductivity follows Eq. [5.4.3]-[5.4.4] semantics and preserves near-field-capacity damping behavior, including branch-conditioned WB18 authority (`stz >= 0.95 -> fx=1`; `stz < 0.95 -> dynamic Bi` with explicit non-positive-ratio `Bi=0` legacy-degenerate path). | hard-fail | REF-PERC-CH5-PERC, REF-PERC-CH7-PARAM, REF-PERC-CH7-FROZEN + legacy baseline `/workdir/wepp-forest_260430_baseline/src/watbal.for` | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-005 | Lower-layer restriction invariant: when Eq. [5.4.5] is applied, lower-layer saturation reduction must remain in real-number domain and cannot amplify `pei` above its pre-restriction value. Baseline `perc.for` caps non-bottom lower-layer `stu >= 0.95` to `0.95` before `sqrt(1-stu)`; this explicit cap is authoritative routing behavior, not silent defaulting, and non-finite/negative lower-layer ratios remain typed domain failures. | hard-fail | REF-PERC-CH5-PERC, REF-PERC-PHYS-BOUNDS, REF-PERC-LEGACY-LOWER-SAT-CLAMP | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-006 | Below-root loss accounting invariant: percolation routed below the root zone is treated as loss in Chapter-5 daily closure (`D`) and cannot be silently recycled into root-zone storage within this contract boundary. | hard-fail | REF-PERC-CH5-BAL, REF-PERC-CH5-LINK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-007 | Subsurface coupling invariant: daily percolation recharge term `Pe` used by subsurface continuity equations is emitted with unit/sign consistency and complete boundary payload semantics. | hard-fail | REF-PERC-CH6-CONT, REF-PERC-CH6-DRAIN | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-008 | Coupled root-zone update invariant: percolation processing remains explicitly coupled with infiltration/ET daily accounting paths described in §5.5 and does not permit silent omission of percolation updates from layer-water bookkeeping. | hard-fail | REF-PERC-CH5-LINK, REF-PERC-CH5-BAL | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-009 | Governance scope invariant: claims about subsurface lateral-flow/drainage mechanics beyond declared percolation boundary are non-promotable unless backed by `SC-SUBHYD-001` authority. | governance-fail | REF-PERC-CH6-CONT, REF-PERC-CH6-DRAIN | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-010 | WB18 percolation execution invariant: percolation phase computes deterministic per-layer `pei`, preserves lane-specific execution semantics (daily single-pass; hourly 24-substep recompute loop), aggregates `D`/`Pe`, and updates both layer moisture (`wb18_perc_theta_####`) and aggregate soil-water state (`wb11_soil_water`) under explicit field-capacity branching. | hard-fail | REF-PERC-CH5-PERC, REF-PERC-PHYS-BOUNDS + legacy `/workdir/wepp-forest_260430_baseline/src/purk.for` + `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-011 | WB18 percolation guard invariant: missing/non-finite/out-of-range per-layer percolation domains must surface typed hard failures (`HKERNEL-WB11-PERC-E-001..003`) and cannot be silently clamped/defaulted; explicit legacy-degenerate `Bi=0` branch for non-positive `FC/UL` is authoritative behavior, not silent fallback. | hard-fail | REF-PERC-PHYS-BOUNDS + legacy baseline `/workdir/wepp-forest_260430_baseline/src/watbal.for` | `[INFERENCE][Static] + [DIRECT][Static]` |
| INV-PERC-012 | HPHYS0242 hourly percolation cadence invariant: in hourly-lane closure, WB18 must complete the 24-substep accumulated `D`/`Pe` and mutated `wb18_perc_theta_####` lineage before final-hour ET and the WB19 drainage/lateral tail execute; downstream WB12 storage reconciliation must consume the same-pass `D`. Stale, missing, non-finite, or aggregate-only percolation lineage cannot satisfy hourly WB14/WB12 closure. | hard-fail | REF-PERC-CH5-PERC, REF-PERC-CH5-BAL, legacy `/workdir/wepp-forest_260430_baseline/src/purk.for`, legacy `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:541-560`, SC-WATBAL-001#INV-WATBAL-034, SC-EVAP-001#INV-EVAP-014 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-013 | HPHYS0246 WB18 aggregate soil-water invariant: after WB18 percolation mutates `st(i)`/`wb18_perc_theta_####`, aggregate `wb11_soil_water` must be recomputed as baseline `watcon = Σ soilw(i)` rather than `Σst(i)`, preserving required `thetdr_i*dg_i` residual/dead-water storage in unfrozen conditions and subtracting declared frozen depth when explicitly present. | hard-fail | REF-PERC-LEGACY-SOILW, SC-WATBAL-001#INV-WATBAL-029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-014 | HPHYS0248 hourly bottom restrictive-layer invariant: when hourly WB18 executes at the bottom layer, baseline `perc.for` sets `meblfc=1` and forces `fx=1`; when `slflag=1`, effective conductivity must then follow thickness-weighted restrictive-layer lineage `Ksi_eff = (dg_i+ui_bdrkth)/(dg_i/Ksi + ui_bdrkth/kslast)` before `sep = min(vv, 86400*Ksi_eff)` and `purk`'s `sep/ui_LFtstp` mutation/`deepSeep` accumulation. Reusing unrestricted bottom `Ksi`, omitting `ui_bdrkth`, applying unsaturated `fx` damping to hourly bottom seepage, or using daily-only harmonic conductivity for hourly closure is invalid evidence. | hard-fail | REF-PERC-LEGACY-HOURLY-BOTK, REF-PERC-CH5-PERC, SC-WATBAL-001#INV-WATBAL-036 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-015 | HPHYS0260 WB18 trace-localization invariant: residual ownership claims for H1/H7/H39 `Dp`, `Total-Soil`, or `SoilWaterTotal` must consume trace-grade post-WB18 and final-storage evidence for `D`, `Pe`, `wb18_perc_pei_####`, `wb18_perc_theta_####`, `wb19_thetdr_####`, `wb19_dg_####`, optional `wb18_perc_frozen_depth_####`, and aggregate `wb11_soil_water`. Trace classification must preserve the baseline aggregate relation `watcon = Σ(st(i) + thetdr(i)*(dg(i)-frozen(i)))` and the WB18 publication relation `D = Pe` for bottom-layer loss, without collapsing `D` to `Σpei_####`. | hard-fail | REF-PERC-LEGACY-SOILW, REF-PERC-LEGACY-HOURLY-BOTK, REF-PERC-CH5-BAL, INV-PERC-013, INV-PERC-014, SC-WATBAL-001#INV-WATBAL-046 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PERC-016 | HPHYS0283 active-snowmelt infiltration-to-layer invariant: when routed snowmelt (`wmelt`) is present, WB18 percolation must consume the same-pass WB14/WB12 infiltration lineage as top-down `fin/xfin` ingress to `st(i)`/`wb18_perc_theta_i` before percolation routing and aggregate `watcon` recomputation. Evidence that publishes `wb12_infiltration` or reduces `Q` without increasing layer/aggregate storage remains a hard storage-collapse defect for active snowmelt events. Full direct-rain `fin/xfin` ingress remains baseline authority but is outside the HPHYS0283 snowmelt-specific closure claim unless separately promoted by a follow-on package. | hard-fail | REF-PERC-LEGACY-HOURLY-FIN, REF-PERC-LEGACY-SOILW, SC-SNOWFREEZE-001#INV-SNOWFREEZE-018, SC-RUNOFFPART-001#INV-RUNOFFPART-015, SC-WATBAL-001#INV-WATBAL-058 | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-PERC-001` | runtime | Layer percolation branch selector (Eq. [5.4.1]) | Typed hard error on implicit/mismatched branch behavior | Tier-A gate | `[DIRECT][Static]` |
| `INV-PERC-002` | runtime | Excess-water bounds check on per-layer `pei` | Typed hard error on negative or excess-over-bound percolation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-003` | runtime | Active-branch routing-domain validator for `ti`/`Ksai`/state terms | Typed hard error on undefined/non-finite domain terms | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-004` | runtime | Adjusted-conductivity evaluator for Eq. [5.4.3]-[5.4.4] and active Chapter-7 condition modifiers | Typed hard error on conductivity-domain violation or damping-semantics mismatch | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-005` | runtime | Lower-layer restriction evaluator (Eq. [5.4.5]) | Typed hard error on non-finite/negative lower-ratio domain or amplification above pre-restriction flux; baseline-authoritative `stu >= 0.95 -> 0.95` cap is required attenuation behavior | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-006` | runtime | Daily closure assembler for below-root loss term `D` | Typed hard error on inconsistent loss accounting at boundary publish | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-007` | runtime | Percolation-to-subsurface boundary payload validator (`Pe`) | Typed hard error on missing malformed units/sign payload | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-008` | runtime | Layer-water bookkeeping integration checks with infiltration/ET update path | Typed hard error on omitted percolation update in daily coupled accounting | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-009` | governance | Contract review/disposition/promotion checklist | Promotion `HOLD` when subsurface mechanics claims exceed declared contract boundary | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-010` | runtime | WB18 per-layer percolation production kernel execution path | Typed hard error on malformed/non-deterministic per-layer or aggregate percolation writeback outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-011` | runtime | WB18 per-layer guard table (`HKERNEL-WB11-PERC-E-001..003`) | Typed hard error on missing/non-finite/domain-invalid per-layer percolation inputs/outputs | Tier-A gate | `[INFERENCE][Static]` |
| `INV-PERC-012` | runtime + governance | WB18 hourly-lane output lineage validator plus scheduler-order gate into ET/WB19/WB12 | Typed hard error / explicit `HOLD` when hourly `D`/`Pe`/layer-state lineage is stale, missing, malformed, or consumed out of baseline order | HPHYS cadence/order closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-013` | runtime | WB18 aggregate soil-water recomputation from `wb18_perc_theta_####`, `wb19_thetdr_####`, `wb19_dg_####`, and optional `wb18_perc_frozen_depth_####` (legacy generic fallback accepted for fixture compatibility) | Typed hard error on missing/non-finite/domain-invalid residual-storage symbols; aggregate writeback must not collapse to `Σtheta` | HPHYS0246 closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-014` | runtime + governance | WB18 hourly bottom-layer effective-conductivity selector plus H39 `Dp`/`Pe` evidence gate | Typed hard error / explicit `HOLD` when hourly restrictive-layer `D`/`Pe` lineage omits `ui_bdrkth`/`kslast` or bypasses baseline thickness-weighted `sscz` | HPHYS0248 H39 `Dp`/`Pe` closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-015` | runtime + governance | WB18 trace-localization validator spanning `D`/`Pe`, per-layer `pei`, per-layer `st`, residual/depth/frozen aggregate components, and final aggregate `wb11_soil_water` | Typed hard error / explicit `HOLD` when trace evidence omits required WB18/storage maps, when `D`/`Pe` or aggregate storage identities do not reconcile, or when residual ownership is assigned without trace-grade evidence | HPHYS0260 WB18/storage residual-classification gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PERC-016` | runtime + governance | WB18 active-snowmelt same-pass infiltration ingress before percolation, layer-state writeback, and aggregate `wb11_soil_water` recomputation | Typed hard error / explicit `HOLD` when routed melt/infiltration is absent from layer storage or is deferred to publication-only compensation | HPHYS0283 spring storage-collapse gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols follow Chapter-5/Chapter-6 WEPP notation. WB18 establishes
explicit runtime aliases for per-layer percolation state/flux surfaces.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `Θi` | `wb18_perc_theta_####` | per-layer moisture state consumed and mutated by WB18 percolation routing | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `FCi` | `wb18_perc_fc_####` | per-layer field-capacity threshold surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ULi` | `wb18_perc_ul_####` | per-layer upper-limit storage surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ksi` | `wb18_perc_ssc_####` | per-layer saturated hydraulic conductivity surfaces | `m s^-1` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ksbot` | `kslast` | restrictive-layer conductivity used by daily bottom-layer branch when `slflag=1` | `m s^-1` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Bbot` | `ui_bdrkth` | restrictive-layer thickness used by hourly bottom-layer thickness-weighted branch when `slflag=1` | `m` preserved | `[DIRECT][Static]` |
| `pei` | `wb18_perc_pei_####` | per-layer percolation flux outputs | `m` per step preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `st(i)` / `Θi` | `wb18_perc_theta_####` | per-layer liquid storage state after WB18 routing | `m` preserved | `[DIRECT][Static]` |
| `thetdr(i)` | `wb19_thetdr_####` (legacy `thetdr_####` fallback) | residual/dead-water volumetric layer state used for aggregate `soilw(i)` recomputation | `m^3 m^-3` multiplied by `dg_i` | `[DIRECT][Static]` |
| `dg(i)` | `wb19_dg_####` (legacy `dg_####` fallback) | layer thickness for aggregate residual/dead-water storage | `m` preserved | `[DIRECT][Static]` |
| `frozen(i)` | `wb18_perc_frozen_depth_####` | optional frozen-depth decrement in baseline `soilw(i)` recomputation; absent means unfrozen branch only | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `soilw(i)` / `watcon` | `wb11_soil_water` | aggregate WB11 soil-water publication after WB18 | `m` preserved | `[DIRECT][Static]` |
| `ui_LFtstp` | `wb18_perc_lane_substeps` | per-lane seepage attenuation divisor from legacy hourly routing semantics | unitless positive integer | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Pe`, `D` | `Pe`, `D` | aggregate recharge/loss coupling surfaces | `m` preserved | `[DIRECT][Static]` |
| `fin` / `xfin` | `wb12_infiltration` plus same-pass WB14 forcing lineage | same-pass liquid ingress consumed by WB18 before percolation | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `tillay(2)` | `management.initial.params.tillay2_m` | tilled-layer ingress distribution depth for `xfin`; non-positive values route remaining ingress into the first layer per baseline branch behavior | `m` preserved | `[DIRECT][Static]` |
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
| Non-positive `FC/UL` ratio in active branch | `stz < 0.95` and `FCi/ULi <= 0`, with explicit authoritative mapping `Bi = 0` before `fx = max(stz^Bi, 0.002)`. | Baseline WEPP `watbal.for` sets `hk=0` for non-positive ratio and continues percolation execution. `[DIRECT][Static]` |
| Frozen/restrictive attenuation regime | Effective conductivity is strongly reduced by frozen/restrictive conditions but remains in valid domain. | Chapter-5 and Chapter-7 conductivity-adjustment semantics. `[DIRECT][Static] + [INFERENCE][Static]` |
| Hourly bottom restrictive layer | `slflag=1`, `wb18_perc_lane_substeps=24`, bottom-layer `Ksi>0`, `kslast>0`, and `ui_bdrkth>0`, producing `Ksi_eff` near `kslast` when the restrictive layer is much thicker than the soil layer. | Baseline `perc.for` thickness-weighted `sscz` branch. `[DIRECT][Static]` |
| No below-root export day | Per-layer routing occurs within root zone but aggregate below-root `D` is zero for the step. | Valid daily state when no percolation crosses the root-zone boundary. `[INFERENCE][Static]` |

## Invalid States

- Active percolation branch (`Θi > FCi`) with undefined/non-finite travel-time or conductivity terms. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative percolation flux or per-layer percolation exceeding available excess-water bound beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Eq. [5.4.5] restriction computed outside real domain (invalid lower-layer ratio) or increasing `pei`. `[DIRECT][Static] + [INFERENCE][Static]`
- Below-root percolation exported without consistent daily loss accounting term `D`. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing/malformed `Pe` payload for subsurface continuity consumers. `[DIRECT][Static] + [INFERENCE][Static]`
- Silent omission of percolation-layer updates in coupled daily infiltration/ET/water-balance bookkeeping path. `[DIRECT][Static] + [INFERENCE][Static]`
- Hourly bottom-layer restrictive execution with `slflag=1` but missing,
  non-finite, or non-positive `ui_bdrkth`/`kslast`, or with unrestricted
  bottom-layer `Ksi` used as `Ksi_eff`. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-PERC-P-001: Emit per-layer percolation and coupling surfaces (`pei`, `Pe`, `D`) with canonical symbols and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PERC-P-002: Enforce explicit Eq. [5.4.1]-[5.4.5] branch logic and guard domains; no implicit fallback branches. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PERC-P-003: Propagate invariant failures as typed errors; no silent clamping/defaulting of percolation terms. `[INFERENCE][Static]`
- OBL-PERC-P-004: Preserve boundary-ready loss/recharge semantics for daily closure (`D`) and subsurface coupling (`Pe`). `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PERC-P-005: Preserve opt-in trace observability for per-layer WB18
  routing fluxes, mutated layer storage, residual/depth/frozen aggregate
  components, and aggregate `D`/`Pe` so residual classification can distinguish
  WB18 internal identity divergence from baseline-magnitude follow-up work.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PERC-P-006: Apply active-snowmelt same-pass infiltrated liquid to layer
  storage before percolation and aggregate soil-water recomputation; do not
  use WB12 storage reconciliation or WB13 output publication as compensation
  for omitted routed-`wmelt` `fin/xfin` layer ingress.
  `[DIRECT][Static] + [INFERENCE][Static]`

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
| WB18 percolation production execution and guards (`INV-PERC-010/011`) | percolation kernel execution and guard validation | Hard error on malformed per-layer percolation domains or invalid deterministic updates | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Constants and Parameters Table

| Constant/parameter | Units | Domain | Contract use | Authority |
|---|---|---|---|---|
| `WB18_PERC_STATUS_OK` | status message id | `HKERNEL-WB11-PERC-OK-001` | Typed nominal status for successful WB18 per-layer percolation execution | REF-PERC-CH5-BAL |
| `WB18_PERC_GUARD_MISSING` | status message id | `HKERNEL-WB11-PERC-E-001` | Typed missing-input guard code | REF-PERC-PHYS-BOUNDS |
| `WB18_PERC_GUARD_NONFINITE` | status message id | `HKERNEL-WB11-PERC-E-002` | Typed non-finite guard code | REF-PERC-PHYS-BOUNDS |
| `WB18_PERC_GUARD_DOMAIN` | status message id | `HKERNEL-WB11-PERC-E-003` | Typed domain guard code | REF-PERC-PHYS-BOUNDS |
| `WB18_PERC_BI_COEFFICIENT` | coefficient | `2.655` | Legacy-authoritative coefficient in dynamic per-layer conductivity-shape derivation `Bi = -2.655/log10(FC/UL)` | REF-PERC-CH5-PERC + legacy baseline `/workdir/wepp-forest_260430_baseline/src/watbal.for` |
| `WB18_PERC_MIN_FX` | fraction | `0.002` | Minimum conductivity adjustment factor in active branch | legacy baseline `/workdir/wepp-forest_260430_baseline/src/perc.for` |
| `WB18_PERC_LOWER_STU_CAP` | fraction | `0.95` | Baseline lower-layer saturation cap before `sqrt(1-stu)` attenuation for non-bottom percolation | REF-PERC-LEGACY-LOWER-SAT-CLAMP |
| `WB18_PERC_TIMESTEP_S` | `s` | `86400` | Daily percolation timestep used by WB18 layer travel-capacity term | REF-PERC-CH5-PERC + legacy baseline `/workdir/wepp-forest_260430_baseline/src/perc.for` |
| `WB18_PERC_LANE_SUBSTEPS_DAILY` | unitless | `1` | Daily-lane seepage attenuation divisor | legacy baseline daily branch semantics |
| `WB18_PERC_LANE_SUBSTEPS_HOURLY` | unitless | `24` | Hourly-lane seepage attenuation divisor (`ui_LFtstp`) | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` + `/workdir/wepp-forest_260430_baseline/src/purk.for` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). Contract-specific tolerances:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-PERC-001 | Eq. [5.4.1] per-layer branch residual tolerance | `<= 1e-9 m d^-1` | Residual is evaluated as implemented branch output minus declared branch expression. | `[INFERENCE][Static]` |
| TOL-PERC-002 | Non-negative comparator tolerance for percolation/loss terms (`pei`, `Pe`, `D`) | lower bound `>= -1e-12` in declared units | Comparator-noise allowance only; runtime still hard-fails on material negatives. | `[INFERENCE][Static]` |
| TOL-PERC-003 | Lower-layer restriction radicand comparator-classification tolerance for Eq. [5.4.5] | after baseline `stu` cap, `1 - min(Θi+1 / ULi+1, 0.95) >= -1e-12` | Comparator interpretation must apply the same explicit baseline cap as runtime; non-finite/negative lower ratios still hard-fail. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-PERC-004 | Field-capacity threshold proximity for Eq. [5.4.1] branch split | `abs(Θi - FCi) <= 1e-12 m` treated as threshold-adjacent | Prevents jitter-driven branch toggling near field-capacity boundary in comparator interpretation. | `[INFERENCE][Static]` |

## Test-Vector Obligations

Minimum WB18 percolation production-kernel conformance vectors:

1. Percolation phase emits deterministic per-layer `wb18_perc_pei_####`,
   aggregate `D`/`Pe`, and updates `wb18_perc_theta_####` plus
   `wb11_soil_water`.
2. Non-finite per-layer percolation inputs hard-fail with typed status
   `HKERNEL-WB11-PERC-E-002`.
3. Domain-invalid per-layer percolation inputs hard-fail with typed status
   `HKERNEL-WB11-PERC-E-003` and do not mutate orchestrator writeback surfaces.
4. Dynamic-`Bi` vectors must exercise branch-conditioned authority:
   - active-branch positive-ratio derivation (`Bi = -2.655/log10(FC/UL)`),
   - active-branch non-positive-ratio legacy-degenerate derivation (`Bi = 0`),
   - saturated-branch bypass (`stz >= 0.95 -> fx = 1`) without false
     ratio-domain hard-fail.
5. HPHYS0254 lower-saturation clamp vector: a non-bottom layer with lower
   `wb18_perc_theta/ul >= 0.95` must continue and attenuate by
   `sqrt(0.05)` per baseline `perc.for`; it must not hard-fail solely because
   the downstream layer is above upper limit.
6. Lane-semantics vectors must exercise:
   - daily divisor `wb18_perc_lane_substeps = 1`,
   - hourly divisor `wb18_perc_lane_substeps = 24`,
7. HPHYS0283 same-pass ingress vector: a snowmelt-only event with sufficient
   infiltration capacity must produce positive `wb12_infiltration`, zero `Q`,
   increased `wb18_perc_theta_####`, and increased aggregate
   `wb11_soil_water` by the infiltrated melt depth.
   - hourly substep recompute loop behavior (`24` passes/day),
   - typed hard-fail for non-finite or non-positive lane divisors.
7. Daily restrictive-layer vectors must exercise:
   - `slflag=1` and finite positive `kslast` branch reducing bottom-layer
     percolation via harmonic effective conductivity,
   - typed hard-fail on non-finite or non-positive `kslast` when `slflag=1`.
8. HPHYS0246 aggregate storage vector: with valid `wb19_thetdr_####` and
   `wb19_dg_####`
   and no frozen-depth symbols, WB18 must publish `wb11_soil_water =
   Σ(wb18_perc_theta_i + thetdr_i*dg_i)`; a `Σtheta`-only publication fails.
9. HPHYS0246 guard vector: if a required `wb19_thetdr_####` or
   `wb19_dg_####` symbol is
   missing/non-finite/domain-invalid for WB18 aggregate writeback, the phase
   must return typed `HKERNEL-WB11-PERC-E-001..003` failure rather than silently
   defaulting the residual component to zero.
10. HPHYS0260 trace-localization vector proves opt-in trace rows serialize
    `D`, `Pe`, `wb18_perc_pei_####`, `wb18_perc_theta_####`,
    `wb19_thetdr_####`, `wb19_dg_####`, optional
    `wb18_perc_frozen_depth_####`, and aggregate `wb11_soil_water` from
    post-WB18/final-storage writeback surfaces.

## WB13 Daily Output Coupling Addendum

### WB13 Percolation/Profile Output Symbols

| WB13 column | Percolation/profile coupling surface | Units |
|---|---|---|
| `Dp` | Daily deep-percolation loss term exported from percolation closure surface (`D`) | `mm` |
| `ProfileDepth` | Full-profile depth aggregate for output-surface closure diagnostics | `mm` |
| `ProfilePorosityCap` | Full-profile porosity-capacity aggregate | `mm` |
| `ProfileFCStore` | Full-profile field-capacity storage aggregate | `mm` |
| `ProfileWPStore` | Full-profile wilting-point storage aggregate | `mm` |

### WB13 Coupling Requirements

1. WB13 rows must include finite, non-negative `Dp` and profile-storage symbols
   used by daily water-balance output reconciliation.
2. Profile-storage ordering must satisfy
   `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.
3. Missing/non-finite/out-of-domain WB13 percolation/profile symbols are
   invalid runtime states and must hard-fail with WB13 typed guard posture.
4. WB13 column `Dp` in this addendum is the daily deep-percolation export
   derived from percolation closure term `D` and is explicitly distinct from
   climate time-to-peak descriptor `Dp` (`SC-CLIMATE-001`, units `h`).
5. `ProfileFCStore` publication semantics are layer-authoritative WB13
   aggregates plus explicit normalized-tail contribution:
   `Σ(thetfc_i * dg_i) * 1000 + wb13_profile_fc_tail_mm` (`mm`) under
   HPHYS0216D reconciliation authority.
6. `ProfileWPStore` publication semantics remain normalized-profile storage
   projection from baseline-corrected corrected-layer lineage via
   `wb13_profile_wp_store_mm` under HPHYS0209.
7. `wb13_profile_fc_store_mm` remains diagnostic/reconciliation carry lineage
   and is not direct publication authority for `ProfileFCStore`.
8. `wb13_profile_fc_tail_mm` must be finite and non-negative; missing or
   invalid tail symbols are typed WB13 publication failures.
9. Normalized corrected-layer depth-domain closure requirements remain
   authoritative for profile-capacity/projection seed families; no silent
   truncation or parser-domain surrogate override.
10. WB13 `Dp` publication must consume flux-authoritative `D` on the runtime
    writeback surface and must not be shadowed by stale state-surface `D` when
    both are present.

## HPHYS0208 FC-Threshold Consumer-Lineage Closure Addendum

1. WB18 percolation-consumer seed symbols must be initialized from
   baseline-authoritative threshold lineage:
   - `wb18_perc_theta_#### = st(i)` where
     `st(i) = (((sat * por_i) * cpm_i) - thetdr_i) * dg_i`,
   - `wb18_perc_fc_#### = FCi = dg_i * (thetfc_i - thetdr_i)`,
   - `wb18_perc_ul_#### = ULi = (por_i - thetdr_i) * dg_i`.
2. Required threshold-lineage inputs are `sat`, `por_####`, `cpm_####`,
   `thetfc_####`, `thetdr_####`, and `dg_####`.
3. Missing/non-finite/domain-invalid threshold-lineage inputs are typed
   hard-fail percolation-consumer states; FC/WP surrogate seed substitution is
   prohibited.

## HPHYS0230 WB18 Over-Drainage Authority Addendum

1. WB18 adjusted-conductivity damping must use per-layer dynamic shape
   parameter derived from threshold lineage:
   - `Bi = -2.655 / log10(FCi/ULi)`,
   - `FCi = wb18_perc_fc_####`,
   - `ULi = wb18_perc_ul_####`.
2. Domain handling for dynamic `Bi` derivation is branch-conditioned:
   - active branch (`stz < 0.95`) and `FCi/ULi <= 0` maps explicitly to
     authoritative legacy-degenerate `Bi = 0`,
   - active branch (`stz < 0.95`) and `FCi/ULi > 0` uses
     `Bi = -2.655/log10(FCi/ULi)` with typed hard-fail for non-finite or
     non-physical denominator domains,
   - saturated branch (`stz >= 0.95`) bypasses dynamic-`Bi` ratio evaluation
     (`fx = 1`) and must not raise false ratio-domain hard-fail.
3. Constant exponent substitutions (for example `Bi = 1`) are non-authoritative
   and non-promotable for WB18 process-physics closure.

## HPHYS0232 WB18 Hourly-Lane Seepage Attenuation Addendum

1. WB18 per-layer seepage publication must encode lane attenuation lineage from
   legacy `purk.for` updates:
   - compute per-layer routed seepage `pei_unscaled` per Chapter-5 authority,
   - publish `pei = pei_unscaled / wb18_perc_lane_substeps`.
2. `wb18_perc_lane_substeps` domain is strict: finite, positive, integral,
   and `>= 1`.
3. Runtime seed authority for `wb18_perc_lane_substeps`:
   - daily lane: `1`,
   - hourly lane: `24` (legacy `ui_LFtstp` assignment from
     `watbal_hourly.for`).

## HPHYS0233 WB18 Daily Restrictive-Conductivity and WB13 D-Lineage Addendum

1. For daily lane (`wb18_perc_lane_substeps=1`) bottom-layer percolation
   (`i=nsl`) with restrictive layer enabled (`slflag=1`), WB18 must apply
   baseline-authoritative effective conductivity branch:
   - `Ksbot = kslast`,
   - `Ksi_eff = 2*Ksi*Ksbot/(Ksi+Ksbot)`.
2. `kslast` domain is strict when `slflag=1`: finite and `> 0`.
3. WB13 deep-percolation publication (`Dp`) must be sourced from
   flux-authoritative percolation closure surface `D` and must not be shadowed
   by stale state-surface `D`.

## HPHYS0235 WB18 Hourly Iterative-Lane Authority Reanchoring Addendum

1. `ui_run=1` lineage is authoritative to legacy `watbal_hourly.for`
   execution shape, where percolation is invoked across `ui_LFtstp=24`
   substeps/day rather than a single daily pass with only divisor scaling.
2. Hourly-lane percolation must recompute per-layer routing terms from current
   substep-updated layer state before each substep writeback.
3. Divisor-only hourly treatment (`pei_unscaled / 24` applied once per day
   without substep recomputation) is non-authoritative and non-promotable for
   WB18 closure.

## HPHYS0242 WB18 Same-Pass Cadence Addendum

1. Hourly-lane WB18 `D`/`Pe` publication is same-pass authoritative for the
   downstream WB14/WB12 tail only after all 24 substeps complete and the layer
   state has been written back.
2. Final-hour ET must observe this same-day percolation-mutated layer state;
   WB19 drainage/lateral and WB12 storage closure must consume the same-pass
   `D` value.
3. Compatibility aggregate state cannot replace missing or malformed hourly
   percolation lineage in HPHYS0242 closure claims.
4. Contract-derived vectors must assert hourly percolation before ET and
   same-pass `D` consumption at WB12 storage reconciliation.

## HPHYS0246 WB18 Aggregate Soil-Water Writeback Addendum

1. Baseline-authoritative aggregate storage after WB18 follows
   `/workdir/wepp-forest_260430_baseline/src/watbal.for:960-966` and
   `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1018-1025`:
   `soilw(i) = st(i) + thetdr(i)*(dg(i)-frozen(i))` and
   `watcon = Σsoilw(i)`.
2. The openWEPP WB18 alias mapping is:
   - `st(i) -> wb18_perc_theta_####`,
   - `thetdr(i) -> wb19_thetdr_####`,
   - `dg(i) -> wb19_dg_####`,
   - optional `frozen(i) -> wb18_perc_frozen_depth_####`.
3. WB18 must publish `wb11_soil_water = Σsoilw(i)` after per-layer
   percolation routing. Publishing `Σwb18_perc_theta_####` is
   non-authoritative because it drops the residual/dead-water storage component
   required by baseline `soilw` lineage.
4. Missing/non-finite/domain-invalid `wb19_thetdr_####` or `wb19_dg_####` symbols are
   typed hard-fail states for aggregate WB18 writeback; defaulting the
   residual/dead-water component to zero is prohibited.
5. Absence of `wb18_perc_frozen_depth_####` selects the unfrozen-layer branch
   for aggregate recomputation and cannot be cited as frost-active parity
   closure. Frost-active aggregate closure remains governed by
   `SC-SNOWFREEZE-001` and the eventual per-layer frozen-depth exchange seam.

## HPHYS0254 WB18 Lower-Layer Saturation Clamp Addendum

1. Baseline `perc.for` caps lower-layer saturation `stu` to `0.95` when
   `stu >= 0.95` before applying the lower-layer attenuation
   `cr = sqrt(1-stu)`.
2. This cap is authoritative percolation physics from the pinned legacy
   baseline, not a silent default or heuristic clamp. Runtime must still
   reject missing, non-finite, negative, or invalid-`UL` lower-layer domains.
3. A downstream layer above `UL` after bottom-up `purk.for` routing remains a
   valid continuation state for non-bottom percolation and attenuates by
   `sqrt(0.05)` rather than failing the scheduler.

## HPHYS0260 WB18 Trace Localization Addendum

1. H1/H7/H39 `Dp` and final-storage residual classification must consume
   post-WB18 and final trace rows carrying aggregate `D`, aggregate `Pe`,
   per-layer `wb18_perc_pei_####`, post-routing `wb18_perc_theta_####`,
   `wb19_thetdr_####`, `wb19_dg_####`, optional
   `wb18_perc_frozen_depth_####`, and aggregate `wb11_soil_water`.
2. Trace classifiers must verify `D = Pe` for the WB18 bottom-loss publication
   relation and must not classify `D` as `Σwb18_perc_pei_####`; the per-layer
   `pei` map includes within-profile transfers as well as bottom export.
3. Aggregate storage classification must recompute
   `Σ(wb18_perc_theta_i + wb19_thetdr_i*(wb19_dg_i - frozen_i))` with absent
   frozen-depth maps selecting the unfrozen branch, then compare that result to
   traced `wb11_soil_water`.
4. If these identities close while H1/H7/H39 `Dp` or storage residuals persist,
   continuation must target baseline-authoritative magnitude/initialization
   lineage rather than trace publication or aggregate recomputation defects.

## HPHYS0283 Same-Pass Infiltration Ingress Addendum

1. Baseline `watbal_hourly.for` computes daily liquid ingress as `fin` from
   direct rain, routed `wmelt`, irrigation, interception, and carry/runoff
   adjustments, then applies hourly `xfin` to `st(i)` before percolation and
   final-hour ET.
2. For HPHYS0283, WB18 must therefore treat active-snowmelt same-pass
   WB14/WB12 infiltration as a state mutation input to the per-layer storage
   surface, not only as a runoff partition diagnostic.
3. Spring snowmelt evidence that shows reduced `Q` but unchanged
   `wb18_perc_theta_####`/`wb11_soil_water` is incomplete and remains a
   storage-collapse defect.
4. `management.initial.params.tillay2_m` is the openWEPP runtime alias for
   baseline `tillay(2)` in this distribution lineage; non-positive values
   select the baseline branch where the first layer receives the remaining
   ingress.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-PERC-001 | Per-invariant comparator vectors for per-layer percolation and lower-layer restriction behavior are not yet curated in this package. | Limits immediate automation depth for invariant-specific acceptance checks. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-PERC-002 | Extended alias coverage for optional per-layer diagnostics beyond WB18 core symbols (`wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`, `wb18_perc_pei_####`) is not yet finalized. | Core WB18 aliases are fixed; extended diagnostics remain provisional. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-PERC-003 | Companion contract `SC-SUBHYD-001` is not yet fully authored, so cross-domain ownership boundaries for subsurface routing remain provisional. | Promotion-readiness depends on downstream contract completion/consistency. | non-promotable | `[DIRECT][Static]` |
| GAP-PERC-004 | Chapter-5 validation evidence is reported at aggregate water-balance behavior; dedicated per-layer percolation validation vectors are not explicitly separated in cited material. | Per-layer percolation confidence is lower than aggregate daily closure confidence until dedicated evidence is added. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-PERC-005 | HPHYS0260 adds trace-grade WB18 residual classification authority but does not itself change percolation physics. | Closure remains `HOLD` when identities close but comparator residuals persist, because follow-on work must target baseline-authoritative magnitude or initialization lineage. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |

## Promotion Readiness

This revision remains intentionally non-promotable and stays in lifecycle state
`in_review` while `GAP-PERC-003` remains open. Governance guard
`INV-PERC-009` requires explicit `HOLD` until cross-domain `SC-SUBHYD-001`
authority closure is completed.

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-06-04` | `25` | `Codex` | HPHYS0283 amendment: added same-pass `fin/xfin` infiltration ingress authority requiring routed snowmelt/infiltration to mutate WB18 layer storage before percolation and aggregate `watcon` recomputation. |
| `2026-06-03` | `24` | `Codex` | HPHYS0260 amendment: added `INV-PERC-015` requiring trace-grade `D`/`Pe`, per-layer `pei`, layer storage, residual/depth/frozen components, and aggregate `watcon` identity evidence before assigning H1/H7/H39 `Dp`/storage residual ownership. |
| `2026-06-02` | `23` | `Codex` | HPHYS0254 amendment: added baseline `perc.for` lower-layer `stu >= 0.95 -> 0.95` clamp authority so WB18 non-bottom percolation attenuates over-UL downstream layers by `sqrt(0.05)` instead of hard-failing. |
| `2026-06-02` | `22` | `Codex` | HPHYS0248 amendment: added `INV-PERC-014` and baseline `perc.for`/`purk.for` hourly bottom restrictive-layer authority requiring `ui_bdrkth`/`kslast` thickness-weighted effective conductivity for hourly `Dp`/`Pe` lineage. |
| `2026-06-02` | `21` | `Codex` | HPHYS0246 amendment: added `INV-PERC-013` requiring WB18 aggregate `wb11_soil_water` writeback to recompute baseline `watcon = Σsoilw(i)` from layer theta plus residual/dead-water storage (`thetdr_i*dg_i`, minus explicit frozen-depth when present) rather than collapsing to `Σtheta`; added alias, guard, and test-vector obligations. |
| `2026-06-01` | `20` | `Codex` | HPHYS0242 amendment: added `INV-PERC-012` and same-pass hourly `D`/`Pe`/layer-state cadence authority so final-hour ET, WB19, and WB12 storage consume percolation lineage produced by the current hourly pass. |
| `2026-06-01` | `19` | `Codex` | HPHYS0235 amendment: reanchored hourly WB18 authority from divisor-only attenuation to legacy `watbal_hourly`/`purk` iterative 24-substep semantics; added non-promotable prohibition for single-pass divisor-only hourly treatment. |
| `2026-06-01` | `18` | `Codex` | HPHYS0233 amendment: added daily restrictive-layer bottom-conductivity branch authority (`slflag`/`kslast` harmonic `Ksi_eff`) and WB13 deep-percolation publication anti-shadow requirement (flux-authoritative `D`). |
| `2026-06-01` | `17` | `Codex` | HPHYS0232 amendment: added WB18 hourly-lane seepage attenuation authority (`wb18_perc_lane_substeps`) mapped to legacy `ui_LFtstp` lineage (`daily=1`, `hourly=24`), with required test-vector and guard obligations. |
| `2026-06-01` | `16` | `Codex` | HPHYS0231 amendment: reanchored WB18 dynamic-`Bi` guard placement to baseline branch semantics by codifying explicit non-positive `FC/UL` legacy-degenerate `Bi=0` handling in active branch and saturated-branch ratio-bypass behavior (`stz >= 0.95 -> fx=1`). |
| `2026-06-01` | `15` | `Codex` | HPHYS0230 amendment: replaced constant WB18 conductivity-shape exponent authority with legacy-authoritative dynamic per-layer derivation `Bi = -2.655/log10(FC/UL)`, added strict ratio-domain guard obligations, and recorded over-drainage closure addendum for WB18 `Dp` transient behavior. |
| `2026-05-31` | `14` | `Codex` | HPHYS0216D amendment: reconciled WB13 `ProfileFCStore` authority to `Σ(thetfc_i*dg_i)*1000 + wb13_profile_fc_tail_mm`, retained `wb13_profile_fc_store_mm` as diagnostic/reconciliation lineage, and required finite non-negative tail-symbol guard posture. |
| `2026-05-31` | `13` | `Codex` | HPHYS0216 amendment: realigned WB13 `ProfileFCStore` publication coupling to baseline layer aggregation (`Σ(thetfc_i*dg_i)*1000`), retained `wb13_profile_fc_store_mm` as diagnostic carry lineage, and kept normalized corrected-layer seed/projection depth-domain obligations explicit. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-08 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-5/6 authority anchors, invariants, guard map, alias map, obligations, boundary disposition, tolerances, and gap register for SCI-08 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added direct Chapter-7 anchors for conductivity modifiers, normalized evidence-mode tokens, clarified lower-layer restriction tolerance vs runtime hard-fail semantics, added evidence tags to degenerate-state/tolerance rows, and made non-promotable `HOLD` state explicit. |
| `2026-05-23` | `3` | `Codex` | WB10 amendment: added explicit percolation phase-entry routing authority, unsupported-class typed hard-fail posture, and WB10 percolation test-vector obligations. |
| `2026-05-23` | `4` | `Codex` | WB11 amendment: promoted percolation section from routing-only scaffolding to production-kernel authority with deterministic `D`/`Pe` updates, typed percolation guard codes (`HKERNEL-WB11-PERC-E-001..003`), and WB11 contract-derived vectors. |
| `2026-05-23` | `5` | `Codex` | WB13 amendment: added percolation/profile coupling authority for canonical daily output columns (`Dp`, `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`) with explicit malformed-output hard-fail posture. |
| `2026-05-23` | `6` | `Codex` | WB18 amendment: replaced WB11 scalar percolation authority with WB18 per-layer physics authority (`wb18_perc_theta/fc/ul/ssc/pei_####`), bottom-up layer routing semantics, conductivity-domain constants (`Bi=1`, `fx_min=0.002`, `Δt=86400s`), and updated guard/test obligations. |
| `2026-05-29` | `7` | `Codex` | HPARITY01 amendment: added explicit cross-contract `Dp` disambiguation note (WB13 deep-percolation export vs climate time-to-peak symbol) in WB13 coupling requirements. |
| `2026-05-29` | `8` | `Codex` | HPHYS0202 amendment: made WB13 `ProfileFCStore`/`ProfileWPStore` coupling semantics explicit as layer-authoritative runtime aggregates (`thetfc/thetdr` with `dg`) in `mm`. |
| `2026-05-29` | `9` | `Codex` | HPHYS0205 amendment: required authoritative WB13 layer-theta symbols used by profile storage coupling to carry baseline-corrected moisture lineage when available. |
| `2026-05-30` | `10` | `Codex` | HPHYS0206 amendment: required deterministic normalized-layer mapping closure for authoritative FC/WP publication symbols and prohibited raw-theta fallback for that publication path. |
| `2026-05-30` | `11` | `Codex` | HPHYS0207 amendment: ratified normalized-profile storage publication authority for WB13 FC/WP outputs via `wb13_profile_fc_store_mm`/`wb13_profile_wp_store_mm` and added explicit normalized-tail consumption policy authority. |
| `2026-05-30` | `12` | `Codex` | HPHYS0208 amendment: required WB18 percolation seed-consumer initialization from baseline WB11 threshold lineage (`sat`, `por_####`, `cpm_####`, `thetfc_####`, `thetdr_####`, `dg_####`) and prohibited surrogate FC/WP seed substitution. |
