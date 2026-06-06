---
contract_id: SC-WATBAL-001
title: Water Balance Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 136
producer_scope:
  - Daily root-zone water balance accounting surfaces
  - Daily evapotranspiration distribution and percolation-routing accounting surfaces
  - Daily coupling surfaces linking climate/infiltration/runoff/snow state into water-balance closure
consumer_scope:
  - Plant growth stress and daily growth-regulation consumers
  - Runoff partition and infiltration antecedent-moisture consumers
  - Subsurface/lateral-flow and drainage consumers using daily loss-accounting surfaces
evidence_level: static
last_reviewed: 2026-06-05
supersedes: []
superseded_by: []
---

# SC-WATBAL-001 Water Balance Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for daily root-zone water-balance closure,
flux/state accounting, and required coupling boundaries for openWEPP hydrology.

## Scientific Scope

In scope:
- Daily water-balance closure at root-zone scale.
- Daily accounting for precipitation/snow contributions, interception, runoff,
  evapotranspiration, percolation, and subsurface/drain losses.
- Root-zone evapotranspiration distribution and percolation-routing invariants
  needed to preserve daily closure semantics.
- Producer/consumer boundary obligations between Chapter-5 water-balance
  surfaces and coupled climate, runoff, subsurface, and plant domains.

Out of scope:
- Kernel implementation details and Rust API layout.
- Event-scale infiltration/runoff partition internals owned by `SC-RUNOFFPART-001`.
- Detailed ET-physics option selection beyond the Chapter-5 closure/accounting
  requirements captured here.
- Watershed/channel routing and impoundment accounting surfaces.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-WATBAL-CH5-BAL | `references/50201000/chap5.pdf` §5.1, Eq. [5.1.1] | Daily continuous water-balance equation and required closure terms (`Θ`, `Θin`, `P`, `I`, `S`, `Q`, `ET`, `D`, `Qd`). | `[DIRECT][Static]` |
| REF-WATBAL-CH5-INTERCEPT | `chap5.pdf` §5.1, Eq. [5.1.2] | Biomass-driven interception term semantics and sign/domain expectations. | `[DIRECT][Static]` |
| REF-WATBAL-CH5-SNOW | `chap5.pdf` §5.1 text (rain/snow partition and melt treatment) | Snow accumulation/melt contribution sign convention in Eq. [5.1.1]. | `[DIRECT][Static]` |
| REF-WATBAL-CH5-ETDIST | `chap5.pdf` §5.3, Eq. [5.3.1]-[5.3.4] | Soil-evaporation depth distribution, root-zone uptake distribution, and water-deficit adjustment semantics. | `[DIRECT][Static]` |
| REF-WATBAL-CH5-PERC | `chap5.pdf` §5.4, Eq. [5.4.1]-[5.4.5] | Percolation eligibility at field capacity, conductivity adjustment, and lower-layer restriction semantics. | `[DIRECT][Static]` |
| REF-WATBAL-CH5-LINK | `chap5.pdf` §5.5, Eq. [5.5.1] | Coupling with infiltration and plant growth (`Ws`) and daily closure linkage expectations. | `[DIRECT][Static]` |
| REF-WATBAL-CH3-COUPLING | `references/50201000/chap3.pdf` §3.1-§3.2 | Snowpack accumulation/melt state that contributes signed `S` terms in Chapter-5 closure. | `[DIRECT][Static]` |
| REF-WATBAL-CH4-COUPLING | `references/50201000/chap4.pdf` §4.2 and §4.5 | Infiltration/runoff outputs feeding daily `Q` and antecedent-moisture coupling with Chapter 5. | `[DIRECT][Static]` |
| REF-WATBAL-CH6-COUPLING | `references/50201000/chap6.pdf` §6.2.1, Eq. [6.2.1]-[6.2.5] | Daily subsurface lateral-flow/drainage terms consistent with Chapter-5 `Qd` accounting. | `[DIRECT][Static]` |
| REF-WATBAL-CH8-COUPLING | `references/50201000/chap8.pdf` §8.2.4 and §8.1 coupling text | Plant-water-stress consumption of `Ws` and required plant-surface inputs to water balance. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-ORDER | `/workdir/wepp-forest_260430_baseline/src/watbal.for:486-497,551-552,918-922,958-967` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline WB11 execution ordering authority (`purk -> evap/evappm -> drain/lateral -> swu -> watcon recompute`). | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-HOURLY-CARRY | `/workdir/wepp-forest_260430_baseline/src/wathour.inc:26-44` and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:438-471,776-885` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline hourly MOFE carry-array authority for `ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`, `ui_LfCrf`, `ui_LFtstp=24`, upstream-current OFE copy-forward, and hourly runon/lateral/saturation carry use. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-HOURLY-POSTET-UL | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:557-590,1018-1025`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline hourly post-ET/pre-WB19 layer-cap ordering and aggregate storage authority: `evap/evappm` completes, lower-layer excess above `ul` or `ul-frzw` is moved upward, WB19 drainage/lateral then execute, and final `watcon` is recomputed from layer `st(i)`. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-WINTER-RAINRELEASE | `/workdir/wepp-forest_260430_baseline/src/snowd.for:240-279`, `/workdir/wepp-forest_260430_baseline/src/winter.for:456-459`, `/workdir/wepp-forest_260430_baseline/src/wshirs.for:185-195`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline rain-on-snow partition lineage retains part or all of `hrrain` in snowpack, adds positive residual `hrrain` into `hrmlt`/`wmelt`, and calls runoff/infiltration event assembly when `wmelt > 0`. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-WMELT-INFIL | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` lines 342-345 and `/workdir/wepp-forest_260430_baseline/src/grna.for` lines 267-269, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Daily/hourly water-balance lineage includes `wmelt(iplane)` in infiltration water supply (`fin`) and Green-Ampt snowmelt event forcing (`smrate = wmelt / dur`) before residual runoff/storage closure is computed. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-HOURLY-FIN | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:342-345,471-479,494-516,520-524`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline hourly local `fin/xfin` lineage includes direct rain after interception, routed snowmelt, and irrigation before top-down layer-storage ingress and final `watcon` recomputation; MOFE carry/runon arrays are governed separately by `REF-WATBAL-LEGACY-HOURLY-CARRY`. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-WATCON | `/workdir/wepp-forest_260430_baseline/src/watbal.for:960-967` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline aggregate root-zone water lineage from layer storage (`st`) through `soilw(i)` into `watcon`. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-WB13 | `/workdir/wepp-forest_260430_baseline/src/outfil.for:623-643` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline WB13 publication semantics for `Ep`, `Es`, `Er`, `Total-Soil`, and `SoilWaterTotal`. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-WB13-RM-SNOW | `/workdir/wepp-forest_260430_baseline/src/contin.for:847-880`, `/workdir/wepp-forest_260430_baseline/src/watbalprint.for:84-106`, `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1082-1142`, and `/workdir/wepp-forest_260430_baseline/src/outfil.for:621-630`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline WB13 `RM` and `Snow-Water` publication lineage: winter processing clears `rain(iplane)` except the warm-rain/no-snow restoration branch, `RM = rain(iplane) + wmelt(iplane) + irdept(iplane) + iraplo(iplane)`, and `Snow-Water = snodpy(iplane) * densg(iplane)`. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-HOURLY-ET-WATCON | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:547-560,978-1026` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline hourly final-hour ET execution and immediate post-ET `watcon = Σsoilw(i)` recomputation from layer `st(i)` storage. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-HOURLY-BOTK | `/workdir/wepp-forest_260430_baseline/src/perc.for:163-178,186-214`, `/workdir/wepp-forest_260430_baseline/src/purk.for:167-188`, `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:540-545` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline hourly bottom-layer restrictive conductivity lineage for `Dp`/`Pe`: hourly bottom `meblfc` forces `fx=1`, bottom restrictive `kslast` plus `ui_bdrkth` thickness-weighted `sscz`, `sep/ui_LFtstp` state mutation, and `deepSeep` accumulation. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-DAILY-LATERAL | `/workdir/wepp-forest_260430_baseline/src/watbal.for:286-304,573-704` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline daily WB19 lateral-flow lineage for `latqcc`, including `hk`, `fzdrfc`, `fzul`, daily `solwpv` branch behavior, and conductivity weighting. | `[DIRECT][Static]` |
| REF-WATBAL-LEGACY-HOURLY-SSH | `/workdir/wepp-forest_260430_baseline/src/input.for:753-761,836-844,927-928`, `/workdir/wepp-forest_260430_baseline/src/tilage.for:571-656`, and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:705-715` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline hourly WB19 `latqcc` lineage uses `ui_ssh(i)` horizontal conductivity assembled from `ssc2*ui_anisrt`, preserving vertical `ssc(i)` for percolation/drainage and daily lateral lanes. | `[DIRECT][Static]` |
| REF-WATBAL-INFILE-WEPPUI | `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md` §4, §8, §11 | Cross-contract requested/effective `wepp_ui` mode propagation authority from parser boundary to runtime lane selection. | `[DIRECT][Static]` |
| REF-WATBAL-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative flux magnitudes and bounded stress factors required for physically valid accounting. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `Θ` | `m` | Root-zone soil water content at day end/state point. | water-balance component | infiltration antecedent-state and reporting consumers |
| `Θin` | `m` | Initial root-zone soil water content for the accounting step. | water-balance component input state | daily closure computation |
| `P` | `m`; WB13/WAT publication `mm` | Daily precipitation contribution. | climate/winter coupling | water-balance closure and WB13/WAT publication |
| `RM` | `m`; WB13/WAT publication `mm` | WB13 daily rainfall + irrigation + snowmelt publication term. Baseline publication uses post-winter `rain(iplane) + wmelt(iplane) + irdept(iplane) + iraplo(iplane)`, not raw precipitation plus a SWE-delta proxy. | climate/winter/runoff/irrigation coupling | WB13 daily water-balance output |
| `I` | `m` | Daily interception by vegetation. | water-balance interception routine | water-balance closure |
| `S` | `m` | Snow-water contribution term (`+` melt, `-` accumulation). | winter hydrology coupling | water-balance closure |
| `Q` | `m`; WB13/WAT publication `mm` | Daily surface-runoff contribution. | runoff partition coupling | water-balance closure and WB13/WAT publication |
| `ET` | `m` | Daily evapotranspiration removal term. | ET distribution routine | water-balance closure |
| `D` | `m` | Daily percolation loss below root zone. | percolation routing routine | water-balance closure |
| `Qd` | `m` | Daily subsurface lateral flow or drain-tile loss term. | subsurface/drainage coupling | water-balance closure |
| `VE` | `kg m^-2` | Above-ground biomass used in interception relation. | plant growth coupling | interception calculation |
| `Es`, `Esb`, `Esp` | `m d^-1` | Actual, bare-soil, and potential soil evaporation rates. | ET distribution routine | ET and layer-water updates |
| `Etp` | `m d^-1` | Potential plant transpiration. | ET partition routine | root uptake distribution and stress factor |
| `UPi`, `Ui` | `m d^-1` | Potential and actual layer-wise plant water use for soil layer `i`. | ET/root-uptake routine | stress factor and layer updates |
| `dx`, `ds` | `m` | Max evaporation-influence depth and effective evaporated depth. | ET distribution routine | layer-water updates |
| `Θr` | `m^3 m^-3` | Residual soil moisture threshold for soil evaporation. | soil-state parameterization | ET reduction logic |
| `Θi` | `m` | Soil water content of layer `i`. | soil-layer state | ET and percolation routing |
| `Θc` | `m^3 m^-3` | Critical soil-water fraction for plant-stress response. | crop parameterization | layer uptake adjustment |
| `FCi` | `m` | Field-capacity water content for layer `i` (per Chapter-5 convention). | soil-layer parameterization | percolation eligibility |
| `ULi` | `m` | Upper-limit water content for layer `i`. | soil-layer parameterization | uptake/percolation limits |
| `st(i)` | `m` | Baseline per-layer liquid storage state used by ET extraction, lateral/drain updates, and aggregate root-zone recomputation. | WB11 hydrology kernel | ET/root uptake + `watcon` lineage |
| `soilw(i)` | `m` | Baseline per-layer unfrozen-water intermediate: `st(i) + thetdr(i)*(dg(i)-frozen(i))`. | WB11 aggregate recomputation path | `watcon`/WB13 publication |
| `watcon` | `m` | Baseline aggregate root-zone unfrozen water (`Σ soilw(i)`) used for closure/publication lineage. | WB11 aggregate recomputation path | WB13 `Total-Soil` lineage |
| `UpStrmQ` | `mm` | WB13/hillslope WAT upstream runoff publication term. | watershed/hillslope carry publication | hillslope WAT output |
| `Total-Soil` | `mm` | WB13/hydout aggregate soil-water publication term from `watcon` lineage. | WB11/WB13 publication | hillslope WAT output |
| `frozwt` | `mm` | WB13/hillslope WAT frozen-water publication term. | frost/water-balance publication | hillslope WAT output |
| `QOFE` | `mm` | WB13/hillslope WAT OFE runoff publication term. | runoff publication | hillslope WAT output |
| `Irr` | `mm` | WB13/hillslope WAT irrigation publication term. | irrigation publication | hillslope WAT output |
| `Area` | `m^2` | WB13/hillslope WAT contributing area publication term. | hillslope geometry publication | hillslope WAT output |
| `SoilWaterTotal` | `mm` | Hydout-equivalent aggregate soil-water publication alias from `watcon` lineage. | WB11/WB13 publication | hillslope WAT output |
| `ProfilePorosityCap` | `mm` | WB13 profile porosity-capacity publication term. | soil/profile publication | hillslope WAT output |
| `ProfileFCStore` | `mm` | WB13 profile field-capacity storage publication term. | soil/profile publication | hillslope WAT output |
| `ProfileWPStore` | `mm` | WB13 profile wilting-point storage publication term. | soil/profile publication | hillslope WAT output |
| `wb13_profile_fc_tail_mm` | `mm` | WB13 profile field-capacity tail diagnostic. | runner/WB13 publication | profile-storage diagnostics |
| `InterceptionStorage` | `mm` | WB13/hillslope WAT interception-storage publication term. | interception publication | hillslope WAT output |
| `ui_SUrunf(1:24)`, `ui_SCrunf(1:24)` | `m` per hourly substep | Saturation-runoff carry arrays: upstream OFE input (`ui_SUrunf`) and current OFE output (`ui_SCrunf`) in baseline hourly water-balance lanes. | previous/current hourly OFE water-balance carry state | downstream OFE hourly `xfin`, WB12/WB13 carry publication |
| `ui_LfUrf(1:24)`, `ui_LfCrf(1:24)` | `m` per hourly substep | Subsurface lateral-flow carry arrays: upstream OFE input (`ui_LfUrf`) and current OFE output (`ui_LfCrf`) in baseline hourly water-balance lanes. | WB19/lateral hourly substep runtime | downstream OFE hourly `xfin`, WB12/WB13 carry publication |
| `pei` | `m d^-1` | Percolation rate through layer `i`. | percolation routine | lower-layer routing and `D` term assembly |
| `ti`, `Δt` | `s` | Travel time through layer `i` and travel interval. | percolation routine | percolation step update |
| `Ksi`, `Ksai`, `Bi` | `m s^-1`, `m s^-1`, `fraction` | Saturated and adjusted hydraulic conductivity with conductivity-shape parameter. | soil/percolation routine | percolation routing |
| `Ksbot`, `Bbot` | `m s^-1`, `m` | Restrictive-layer conductivity (`kslast`) and thickness (`ui_bdrkth`) used by hourly bottom-layer percolation seepage lineage. | soil/percolation routine | WB18 `D`/`Pe` assembly |
| `Ws` | `fraction` | Plant growth water-stress factor (`0..1`) from supply/demand ratio. | water-balance/ET coupling | plant growth regulation |

## Algorithm State Surfaces (WB18/WB17 Hydrology Production Kernels)

### Required Inputs

| Surface | Symbols |
|---|---|
| Scheduler phase metadata | `phase_name`, `phase_class`, `consumer_adapter` |
| Coupled PL ordering preconditions | `pl_order_growth_after_decomp`, `pl_order_watbal_after_growth` (validated at growth dispatch before hydrology lane entry) |
| Runoff reconciliation state family | `nslpts`, `slplen`, `avgslp`, `xinput_0001`, `slpinp_0001`, `nsl`, `solthk`, `thetdr`, `thetfc`, `ssc` |
| Storage reconciliation state family | `nsl`, `solthk`, `thetdr`, `thetfc`, `ssc` |
| WB17 ET + WB18 perc + WB19 lateral/drain state inputs | `wb11_soil_water`, `wb11_et_demand`, `lai`, `wb17_residue_interception`, `solwpv`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`, hourly `wb19_lateral_ssh_####` when modern soil input provides `ui_anisrt`, `dg_####`, `por_####`, `coca_####`, `thetfc_####`, `thetdr_####`, `avgslp`, `slplen`, `wb19_lateral_anisotropy_ratio`, `wb19_lateral_drain_lane_substeps`, `wb19_drain_enabled`, `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter`, `wb11_drainage_coefficient`, `slflag`, `kslast`, `ui_bdrkth` |

### Required Outputs

| Surface | Output |
|---|---|
| WB17 ET + WB18 perc flux outputs | `ET`, `Ws`, `Ep`, `Es`, `Er`, `wb18_perc_pei_####`, `D`, `Pe` |
| WB19 lateral/drainage outputs | `q`, `Qdd`, `Qd` |
| MOFE hourly carry-array outputs | `ui_SCrunf_0001..0024`, `ui_LfCrf_0001..0024`, copy-forward `ui_SUrunf_0001..0024`, `ui_LfUrf_0001..0024` when MOFE hourly carry is enabled |
| WB19 state updates | `wb11_soil_water`, `wb18_perc_theta_####`, `wb11_drainable_storage`, `wb19_fcdep`, `wb19_unsdep`, `wb19_watyld` |
| Scheduler/kernel failure surface | Typed hard-fail status for missing/non-finite/out-of-range WB17/WB18/WB19 hydrology domains |

### Mutated State Surfaces

WB18/WB17 mutate water-balance hydrology surfaces deterministically through
phase-specific ET/percolation/lateral/drainage kernels while preserving
orchestrator-owned writeback commit authority. ET execution is WB17
equation-driven (`Esp`, `Etp`, `Er`, `Es`, `Ep`) while percolation/lateral/
drainage remain deterministic kernels (WB18 percolation + WB19
lateral/drainage).

## Algorithm Specification (WB18/WB17 Scheduler Hydrology Production Execution)

1. Map canonical scheduler phase to typed hillslope kernel phase class.
2. Execute WB17 ET phase (`hydrology_evapotranspiration`) using equation-driven
   partition and stress updates, then execute WB18 percolation and WB19
   lateral/drainage phases with deterministic state/flux updates and typed
   invariant guards.
3. Enforce finite and domain bounds for all required WB18/WB19 inputs and
   emitted outputs prior to writeback acceptance.
4. Preserve explicit routing hard-fail posture for unsupported/mismatched
   hydrology phase-class combinations.
5. Apply only accepted writeback payloads via orchestrator-owned state/flux
   maps; reject malformed payloads with typed status signals.

## Branch and Guard Table (WB18/WB17 Hydrology Kernel Set)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-WATBAL-WB17-ET` | phase class `hydrology_evapotranspiration` | `wb11_soil_water`, `wb11_et_demand`, `lai`, `wb17_residue_interception` | runtime | deterministic WB17 ET partition/writeback execution with typed guards (`HKERNEL-WB11-ET-E-001..003`) |
| `BR-WATBAL-WB18-PERC` | phase class `hydrology_percolation_deep_seepage` | `nsl`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####` (+ `slflag`, `kslast`, `ui_bdrkth` for restrictive bottom-layer closure) | runtime | deterministic WB18 per-layer percolation/writeback execution with typed guards (`HKERNEL-WB11-PERC-E-001..003`) |
| `BR-WATBAL-WB19-LAT` | phase class `hydrology_lateral_transfer` | `nsl`, `solthk`, `solwpv`, `dg_####`, `por_####`, `coca_####`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`, hourly `wb19_lateral_ssh_####` when modern soil input provides `ui_anisrt`, `avgslp`, `slplen`, `wb19_lateral_anisotropy_ratio`, `Pe` | runtime | deterministic WB19 layer-aware lateral execution with typed guards (`HKERNEL-WB11-LAT-E-001..003`) |
| `BR-WATBAL-WB19-DRAIN` | phase class `hydrology_drainage` | WB19 lateral symbols + `wb19_drain_enabled`, `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter`, `wb11_drainage_coefficient`, `q` | runtime | deterministic WB19 layer-aware drainage execution with typed guards (`HKERNEL-WB11-DRAIN-E-001..003`) |
| `BR-WATBAL-WB11-UNSUPPORTED` | unsupported hydrology phase-class state | scheduler phase + phase class metadata | runtime | typed hard-fail (`HS-HYDRO-E-001`) and scheduler halt |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-WATBAL-001 | Daily closure invariant: for each daily step, root-zone accounting must satisfy Eq. [5.1.1] with explicit step residual calculation for `Θ = Θin + (P-I) ± S - Q - ET - D - Qd`; residual beyond tolerance is invalid. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-002 | Flux/storage domain invariant: `Θ`, `Θin`, `P`, `I`, `Q`, `ET`, `D`, and `Qd` are non-negative magnitudes; `S` sign is restricted to Chapter-5 convention (`+` melt, `-` accumulation). | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-SNOW, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-003 | Interception invariant: interception is computed from Eq. [5.1.2] with `VE >= 0`, and `0 <= I <= P` for each daily step. | hard-fail | REF-WATBAL-CH5-INTERCEPT, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-004 | Plant-stress coupling invariant: `Ws = (Σ Ui)/Etp` per Eq. [5.5.1], with `0 <= Ws <= 1`, and layer-use adjustment obeys Eq. [5.3.4] (`0 <= Ui <= UPi`); for `Etp = 0` days, the zero-demand branch is explicit (`Σ Ui = 0` and `Ws = 1`). | hard-fail | REF-WATBAL-CH5-ETDIST, REF-WATBAL-CH5-LINK, REF-WATBAL-CH8-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-005 | Soil-evaporation-depth invariant: Eq. [5.3.1]-[5.3.2] constraints hold, including `0 <= ds <= dx` and no extraction below residual-moisture limits. | hard-fail | REF-WATBAL-CH5-ETDIST, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-006 | Percolation eligibility invariant: per-layer percolation obeys Eq. [5.4.1] (`pei = 0` when `Θi <= FCi`; otherwise `pei >= 0`), and adjusted conductivity/percolation semantics follow Eq. [5.4.2]-[5.4.5]. | hard-fail | REF-WATBAL-CH5-PERC | `[DIRECT][Static]` |
| INV-WATBAL-007 | Coupling invariant: daily runoff/infiltration (`Q`, antecedent near-surface moisture), snow contribution (`S`), subsurface/drain loss (`Qd`), and plant forcing inputs (`LAI`, root depth, biomass/residue context for ET partition) must be present with declared units before closure is accepted. | hard-fail | REF-WATBAL-CH3-COUPLING, REF-WATBAL-CH4-COUPLING, REF-WATBAL-CH5-LINK, REF-WATBAL-CH6-COUPLING, REF-WATBAL-CH8-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-008 | Governance invariant: water moving below root zone (`D`) is treated as loss from this component and cannot be silently reintroduced into root-zone closure without explicit cross-contract authority. | governance-fail | REF-WATBAL-CH5-LINK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-009 | WB17/WB18/WB19 production execution invariant: ET/percolation/lateral/drainage kernels must emit deterministic state/flux updates (`ET`, `Ws`, `Ep`, `Es`, `Er`, `wb18_perc_pei_####`, `D`, `Pe`, `q`, `Qdd`, `Qd`) and update owned state surfaces (`wb11_soil_water`, `wb18_perc_theta_####`, `wb11_drainable_storage`, `wb19_fcdep`, `wb19_unsdep`, `wb19_watyld`) with explicit WB19 branch semantics by `solwpv`. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-ETDIST, REF-WATBAL-CH6-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-010 | WB19 guard + routing invariant: unsupported hydrology phase classes and missing/non-finite/out-of-range WB19 lateral/drainage domains must surface typed hard failures (`HS-HYDRO-E-001`, `HKERNEL-WB11-*-E-*`) without silent reassignment/clamping/defaulting. | hard-fail | REF-WATBAL-PHYS-BOUNDS | `[INFERENCE][Static]` |
| INV-WATBAL-011 | INT10 coupled lane-entry invariant: watbal/hydrology phases execute only after successful plant-lane decomposition/growth transition completion with valid ordering preconditions (`pl_order_growth_after_decomp = 1`, `pl_order_watbal_after_growth = 1`); ordering-symbol violations must hard-fail before watbal-lane completion. | hard-fail | REF-WATBAL-CH5-LINK, REF-WATBAL-CH8-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-012 | PL14 replay-candidate emission invariant: WB13 candidate rows staged for strict Tier-A replay must preserve canonical 25-column schema and deterministic `(Y, J, OFE)` ordering; missing required symbols/artifacts or schema/arity violations must hard-fail replay staging without truncation, padding, or legacy-surface substitution. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-013 | CLIM05 snow-coupled closure invariant: when active snow coupling publishes signed `S`, WB12 storage reconciliation must use `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - Q - ET - D - Qd` and hard-fail on missing/non-finite/domain-invalid `S`. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-SNOW, REF-WATBAL-CH3-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-014 | PL14R replay rerun candidate-surface invariant: strict Tier-A rerun candidate staging must explicitly publish required interchange surfaces (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`) from direct openWEPP candidate outputs; missing required surface coverage or synthetic/bootstrap fallback substitution must hard-fail rerun staging and keep disposition in `HOLD`. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-015 | PL15R schema-aligned replay supersession invariant: Tier-A `H.wat.parquet` residual classification must use canonical 25-column schema-aligned strict replay evidence and day-by-day keyed parity (`OFE,J,Y`) before declaring residual blockers. Schema-only pre-alignment failures are historical context once superseding strict-pass evidence is present. | governance-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-016 | WB20 forward-solver lane invariant: when `wb20_forward_solver_lane_enabled = 1`, runoff/storage closure acceptance must be solver-output-derived (`wb12_*_closure_delta` from solver residual identities) and must not consume `wb12_runoff_observed` or `wb12_storage_observed` as acceptance-driving terms. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-CH4-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-017 | PL14S semantic replay diagnostics invariant: Tier-A hillslope replay evidence for WB13 surfaces must include semantic comparator diagnostics keyed by `(OFE,J,Y)` with row-presence deltas, per-column tolerance verdicts, baseline-only column disclosure, and top divergent rows over required investigation columns (`P`, `Q`, `Ep`, `Es`, `Er`, `Dp`, `Total-Soil`, `frozwt`, `Snow-Water`, `SoilWaterTotal`); missing/malformed semantic report content is a hard-fail evidence defect. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-018 | SIMPIPE production execution ownership invariant: publication of water-balance boundary/output surfaces (`Q`, `ET`, `D`, `Qd`, `interchange/H.wat.parquet`) is valid only when derived from an executed scheduler/kernel lane; projection-only synthesis paths without lane execution provenance are invalid. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-019 | SIMMODE runtime lane-selection invariant: effective `wepp_ui` mode (`ui_run`) must deterministically select water-balance execution lane (`daily` when `ui_run=0`, `hourly` when `ui_run=1`) with requested/effective mode observability preserved; missing mode surfaces or lane/mode mismatch must hard-fail without silent fallback. | hard-fail | REF-WATBAL-INFILE-WEPPUI, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-020 | SIMOUT simulation-owned WB13 provenance invariant: WB13/H.wat publication authority is simulation-owned and must originate from executed hydrology lane state/flux writeback surfaces; synthetic/bootstrap substitution or projection-only reconstruction for required candidate surfaces is invalid. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-021 | SIMCONS selective consolidated-intake invariant: consolidated watbal kernel intake from `/workdir/wepp-forest/fpm-src` must remain selective and provenance-triaged (`adopt`, `defer`, `reject`) with explicit typed guard posture; wholesale import or untriaged policy-surface adoption (including qcap-style clamp overlays) is forbidden. | governance-fail | REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-022 | SIMIMPL14 continuous WB13 span/key invariant: continuous hillslope execution must iterate ordered climate-day forcing across the full available run span, execute scheduler/kernel lifecycle once per day with carried runtime state, and publish one WB13/H.wat row per executed day. Published keys must remain monotonic (`sim_day_index = 1..N`) and use simulation-year key semantics (`Y = calendar_year - start_year + 1`) for replay overlap; span collapse, non-monotonic keys, or calendar-year key substitution are invalid. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-INFILE-WEPPUI, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-023 | SIMIMPL15 strict-lane policy and source-provenance invariant: Tier-A replay tooling must publish explicit lane-policy classification by candidate surface format (`.dat` strict-required, `.parquet` strict-equivalent-required via semantic lane) and explicit candidate surface source classification (`native-runtime-dat`, `conversion-derived-dat`, `native-runtime-parquet`). Missing/ambiguous classification or implicit lane-policy fallback is invalid; conversion-derived dat strict evidence is non-promotable for final Tier-A closure claims. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-024 | SIMIMPL15 parquet semantic-alias/diagnostic continuity invariant: semantic comparator lane must canonicalize required investigation columns across accepted alias forms (`Total-Soil` and legacy `Total-Soil Water`) and publish format-consistent row-width diagnostics from observed row field counts (no sentinel placeholder widths). Missing alias continuity or placeholder-only width diagnostics is invalid evidence. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-025 | SIMIMPL16 replay contract-derived test-coverage invariant: Tier-A replay promotability claims must be backed by contract-derived tests that enforce span overlap closure (`common_row_count > 0` with no unresolved baseline-only/candidate-only key residuals for promotable lanes), simulation-year key-domain alignment, `Total-Soil` alias continuity, strict-lane skip compensation for parquet lanes, and conversion-derived dat provenance row-consistency guards. Missing/failed closure tests are non-authoritative evidence. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-026 | SIMIMPL18 day-key rain/snow partition and publication-source invariant: WB13 day-key `RM` must be derived from runtime liquid input (`rain + melt`) rather than direct precipitation passthrough, and published `Snow-Water`/hydout-equivalent snow storage values must derive from runtime snow state (`snow.runtime_swe`) instead of static sidecar controls (`snow.options.ssd`). | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-SNOW, REF-WATBAL-CH3-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-027 | SIMIMPL18 storage-state mutation invariant: published WB13 storage terms (`Total-Soil`, `frozwt`, `Snow-Water`, `SoilWaterTotal`) must be runtime-state-derived and mutable across multi-day forcing; invariant publication of the full storage tuple under non-zero forcing/thermal variation is invalid and indicates static-parameter publication leakage. | hard-fail | REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-SNOW, REF-WATBAL-CH3-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-028 | SIMIMPL21 baseline execution-order invariant: canonical WB11 authority preserves baseline ordering `purk -> evap/evappm -> drain/lateral -> swu -> watcon recompute`; ET transpiration uptake (`swu`) is not authoritative when executed ahead of drainage/lateral mutation. | hard-fail | REF-WATBAL-LEGACY-ORDER, REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-ETDIST | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-029 | SIMIMPL21 aggregate-lineage invariant: root-zone aggregate publication lineage must remain layer-authoritative such that `watcon = Σ soilw(i)` with `soilw(i)` derived from layer storage state and unfrozen-depth adjustment; WB13 `Total-Soil`/`SoilWaterTotal` values must trace to this lineage plus declared frozen/snow components. | hard-fail | REF-WATBAL-LEGACY-WATCON, REF-WATBAL-LEGACY-WB13, REF-WATBAL-CH5-BAL | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-030 | HPHYS0238 WB19 hourly iterative execution invariant: hourly lane execution must run WB19 lateral/drainage with explicit iterative substeps (`wb19_lateral_drain_lane_substeps=24`) and accumulated daily `q`/`Qdd`; divisor-only single-pass substitutions are non-authoritative for hourly closure claims. | hard-fail | REF-WATBAL-CH6-COUPLING, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-031 | HPHYS0239 WB19->WB12->WB13 handoff ordering invariant: promoted hydrology-tail execution must preserve deterministic same-pass ordering through `PercolationDeepSeepage`, `Evapotranspiration`, WB19 subsurface handoff, `RunoffReconciliation`, and `StorageReconciliation`; WB13 `Q`/`Ep`/`Es`/`Er` publication must consume flux-authoritative symbols under state/flux conflicts. HPHYS0242 `INV-WATBAL-034` is the controlling authority for hourly-lane WB19 drainage/lateral ordering. | hard-fail | REF-WATBAL-LEGACY-ORDER, REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS, INV-WATBAL-034 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-032 | HPHYS0240 hourly runoff-carryover invariant: WB12/WB14 runoff reconciliation must resolve incoming runoff carryover from same-pass `wb12_runoff_carryover` flux when present, publish the resolved carryover as a flux, and use `wb12_runon_input` only as a finite non-negative compatibility surface when the same-pass flux is absent. Malformed carryover fluxes are typed hard failures and cannot be silently replaced by stale state. | hard-fail | REF-WATBAL-LEGACY-ORDER, REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-033 | HPHYS0241 MOFE hourly carry-array invariant: multi-OFE hourly lanes must expose all 24 entries of `ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`, and `ui_LfCrf`, consume upstream `ui_SUrunf + ui_LfUrf` arrays as the hourly runon carry source, publish current `ui_SCrunf`/`ui_LfCrf` arrays, and copy current arrays to upstream arrays for the next OFE/day boundary. Missing, non-finite, negative, wrong-cardinality, or aggregate-only carry payloads hard-fail; daily `wb12_runoff_carryover` may only summarize the explicit arrays. | hard-fail | REF-WATBAL-LEGACY-HOURLY-CARRY, REF-WATBAL-LEGACY-ORDER, REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-034 | HPHYS0242 hourly cadence/ordering invariant: hourly-lane water-balance closure must preserve baseline `watbal_hourly` ordering for the WB14/WB12 tail: percolation precedes final-hour ET, drainage precedes lateral flow in the hourly tail, surface saturation excess (`ui_SCrunf(ii)`) is clipped from top-layer storage before runoff publication, `Q` includes `Σui_SCrunf(ii)` plus partition runoff, and storage reconciliation consumes same-pass `Q`, `ET`, `D`, and `Qd` rather than stale compatibility state. | hard-fail | REF-WATBAL-LEGACY-HOURLY-CARRY, REF-WATBAL-LEGACY-ORDER, REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS, SC-RUNOFFPART-001#INV-RUNOFFPART-014, SC-EVAP-001#INV-EVAP-014, SC-PERC-001#INV-PERC-012, SC-SUBHYD-001#INV-SUBHYD-023 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-035 | HPHYS0247 H39 hourly water-balance lateral/snow gate invariant: closure claims for H39 single-OFE hourly water balance must use runtime winter activation triggers from `SC-SNOWFREEZE-001#INV-SNOWFREEZE-009` and WB19 lateral capacity lineage from `SC-SUBHYD-001#INV-SUBHYD-024`; sidecar-presence-only winter bypasses and lateral withdrawals from non-`meblfc` layers are invalid evidence. | hard-fail | REF-WATBAL-LEGACY-HOURLY-CARRY, REF-WATBAL-LEGACY-ORDER, REF-WATBAL-CH5-SNOW, REF-WATBAL-CH6-COUPLING, SC-SNOWFREEZE-001#INV-SNOWFREEZE-009, SC-SUBHYD-001#INV-SUBHYD-024 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-036 | HPHYS0248 H39 hourly `Dp`/`Pe` restrictive-bottom invariant: promoted H39 hourly water-balance evidence must derive WB18 bottom-layer `D`/`Pe` from `SC-PERC-001#INV-PERC-014` baseline hourly restrictive conductivity lineage (`fx=1`, `kslast`, `ui_bdrkth`, thickness-weighted `sscz`, `sep/ui_LFtstp`, accumulated `deepSeep`). H39 closure claims that use unrestricted bottom `Ksi`, daily-only harmonic conductivity, unsaturated `fx` damping, or omit the restrictive-layer thickness branch must remain in `HOLD`. | hard-fail | REF-WATBAL-LEGACY-HOURLY-BOTK, REF-WATBAL-CH5-BAL, SC-PERC-001#INV-PERC-014 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-037 | HPHYS0249 WB17 aggregate-storage invariant: WB17 `Ep`/`Es` closure claims must derive `Total-Soil` and `SoilWaterTotal` from layer-first ET mutation (`SC-EVAP-001#INV-EVAP-015`) followed by baseline `watcon = Σ(st(i) + thetdr(i)*(dg(i)-frozen(i)))` recomputation after soil evaporation and again after post-WB19 `swu` root uptake. Evidence that subtracts `Ep`/`Es` only from scalar `wb11_soil_water`, executes root uptake before WB19 drainage/lateral mutation, or publishes aggregate storage before final WB17 layer mutation is invalid. | hard-fail | REF-WATBAL-LEGACY-WATCON, REF-WATBAL-LEGACY-HOURLY-ET-WATCON, REF-WATBAL-LEGACY-WB13, SC-EVAP-001#INV-EVAP-015 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-038 | HPHYS0250 WB13 final-`Ep` publication invariant: WB13 daily water-balance rows must consume `SC-EVAP-001#INV-EVAP-016` final post-WB19 root-uptake flux `Ep = ΣUi` and must preserve scheduler/growth activation required to produce active `rtd` before water-balance execution. Evidence that strips PL activation sentinels, suppresses growth-derived root depth under management-present runs, or lets stale state-surface `Ep` shadow final flux `Ep` is invalid WB13 closure evidence. | hard-fail | REF-WATBAL-LEGACY-ORDER, REF-WATBAL-LEGACY-WB13, REF-WATBAL-CH5-ETDIST, SC-EVAP-001#INV-EVAP-016 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-039 | HPHYS0251 WB17/WB13 `swu.for` uptake-magnitude invariant: WB17 aggregate-storage and WB13 `Ep` publication claims must consume `SC-EVAP-001#INV-EVAP-017` root-uptake lineage, including crop-specific effective `pltol`, layer `UPi_####`/`Ui_####`, final `Ep=ΣUi`, and post-uptake `wb11_soil_water` recomputed from mutated layer storage before `Total-Soil`/`SoilWaterTotal` publication. A fixed `pltol=0.25` despite crop data, missing layer uptake traces, or aggregate storage derived from pre-uptake state is invalid closure evidence. | hard-fail | REF-WATBAL-LEGACY-ORDER, REF-WATBAL-LEGACY-WATCON, REF-WATBAL-LEGACY-WB13, SC-EVAP-001#INV-EVAP-017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-041 | HPHYS0254 WB11 initial-storage projection invariant: WB11 `st(i)`/`soilw(i)` seeding must use the same baseline-normalized hydrology seed grid as profile-depth/capacity authority, so `wb11_nsl`, `wb19_dg_####`, `wb19_solthk_####`, `wb19_thetfc_####`, `wb19_thetdr_####`, `wb19_por_####`, `cpm_####`, `wb19_coca_####`, `ssc_####`, and WB18 threshold/store aliases span `wb13_profile_depth_mm` without parser-depth tail truncation before `wb11_soil_water = Σsoilw(i)`. Generic `nsl` and constitutive `thetfc_####`/`thetdr_####` remain AUTH03/AUTH05-owned corrected-parser-layer symbols. | hard-fail | REF-WATBAL-LEGACY-WATCON, REF-WATBAL-LEGACY-WB13, SC-SOIL-001#INV-SOIL-015 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-042 | HPHYS0255 MOFE storage-lineage invariant: MOFE WB13/H.wat single-row publication may aggregate `Area` under MOFE04, but storage fields (`Total-Soil`, `SoilWaterTotal`, `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`) must remain traceable to simulation-owned WB11/WB13 runtime storage lineage. Static area-weighted storage synthesis from per-OFE soil rows is non-authoritative unless a future contract migrates per-OFE dynamic hydrology state and explicitly defines the aggregation operator. | hard-fail | REF-WATBAL-LEGACY-WATCON, REF-WATBAL-LEGACY-WB13, SC-SOIL-001#INV-SOIL-016, SC-SYSTEM-001#INV-SYSTEM-029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-043 | HPHYS0256 WB19 `latqcc` lane-branch invariant: daily WB13 `latqcc` closure evidence must consume WB19 lateral flow produced under `SC-SUBHYD-001#INV-SUBHYD-026` daily `watbal.for` authority when `wb19_lateral_drain_lane_substeps=1`; hourly closure evidence must continue to use `SC-SUBHYD-001#INV-SUBHYD-024`/`INV-SUBHYD-025`. Evidence that applies hourly `meblfc` lateral selection to daily lanes, collapses daily/hourly lateral branches, or treats `latqcc` residuals without lane provenance is invalid. | hard-fail | REF-WATBAL-LEGACY-DAILY-LATERAL, REF-WATBAL-LEGACY-HOURLY-CARRY, REF-WATBAL-CH6-COUPLING, SC-SUBHYD-001#INV-SUBHYD-026 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-044 | HPHYS0258 WB19 realized lateral publication invariant: hourly WB13 `latqcc`/`Qd` closure claims must consume realized WB19 lateral withdrawal from `SC-SUBHYD-001#INV-SUBHYD-028`, with diagnostics distinguishing potential, capped target, `tdvv`, and realized per-layer withdrawal. WB13 publication of uncapped potential, capped-but-unwithdrawn target, stale `Qd`, or aggregate storage that cannot reconcile to realized `q` is invalid. | hard-fail | REF-WATBAL-LEGACY-HOURLY-CARRY, REF-WATBAL-LEGACY-WB13, REF-WATBAL-CH5-BAL, SC-SUBHYD-001#INV-SUBHYD-028 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-045 | HPHYS0259 WB19 trace-localization invariant: residual ownership claims for H1/H7/H39 `latqcc`, `Ep`, `Dp`, `Total-Soil`, or `SoilWaterTotal` must consume trace-grade WB19 evidence from `SC-SUBHYD-001#INV-SUBHYD-029` before reopening WB19 cap/publication logic. When trace identities prove realized `q`, `Qd`, and per-layer withdrawal reconcile internally, continuation must assign the dominant residual focus to downstream WB17 `Ep`, WB18 `Dp`, and final aggregate storage reconciliation unless new baseline-authoritative WB19 divergence evidence is produced. | hard-fail | REF-WATBAL-LEGACY-WB13, REF-WATBAL-CH5-BAL, SC-SUBHYD-001#INV-SUBHYD-029, INV-WATBAL-044 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-046 | HPHYS0260 WB17/WB18/final-storage trace-localization invariant: after HPHYS0259 closes WB19 realized-flow identities, residual ownership claims for H1/H7/H39 `Ep`, `Dp`, `Total-Soil`, or `SoilWaterTotal` must consume trace-grade WB17 `UPi_####`/`Ui_####`, WB18 `D`/`Pe`/`pei`/layer-storage, residual/depth/frozen aggregate components, and final WB13 storage publication evidence. When WB17 uptake identities, WB18 `D=Pe`, and aggregate `watcon = Σ(st(i)+thetdr(i)*(dg(i)-frozen(i)))` reconcile internally, continuation must target baseline-authoritative magnitude/initialization lineage rather than trace publication or WB13 shadowing. | hard-fail | REF-WATBAL-LEGACY-WB13, REF-WATBAL-LEGACY-WATCON, REF-WATBAL-CH5-BAL, SC-EVAP-001#INV-EVAP-018, SC-PERC-001#INV-PERC-015, INV-WATBAL-045 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-047 | HPHYS0261 WB17 `Ep` magnitude/initialization invariant: H1/H7/H39 WB13 `Ep` and storage residual closure claims must consume `SC-EVAP-001#INV-EVAP-019` evidence before assigning the stable day-1 `Ep +0.235294 mm` split to a physics defect. Valid evidence must join candidate WAT `Ep`, baseline WAT `Ep`, trace `Etp`, final `Ep`, `ΣUi_####`, `lai`, `rtd`, raw/effective `pltol`, WB18 `ul(i)`, and stress-threshold ratios with static legacy call-order citations. WB13 `Ep` compensation without this lineage is invalid. | hard-fail | REF-WATBAL-LEGACY-WB13, REF-WATBAL-LEGACY-ORDER, REF-WATBAL-CH5-BAL, SC-EVAP-001#INV-EVAP-019, INV-WATBAL-046 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-048 | HPHYS0262 WB17 PMET demand-seeding invariant: H1/H7/H39 WB13 `Ep`, `Total-Soil`, and `SoilWaterTotal` closure claims must consume `SC-EVAP-001#INV-EVAP-020` evidence before assigning the day-1 `Ep +0.235294 mm` split to plant state, root uptake, or WB13 compensation. Valid evidence must join candidate/baseline WAT `Ep`, trace `wb11_et_demand`, actual ET seed branch, `pmetpara.mode.iflget`, selected `kcb`/`rawp`, fallback status, final `Ep`, `ΣUi_####`, and legacy `evap`/`evappm` call-order citations. PMET-mode closure with Priestley-Taylor or proxy demand is invalid. | hard-fail | REF-WATBAL-LEGACY-WB13, REF-WATBAL-LEGACY-ORDER, REF-WATBAL-CH5-BAL, SC-EVAP-001#INV-EVAP-020, INV-WATBAL-047 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-049 | HPHYS0263 WB11/WB17 EVAPPM migration invariant: when `pmetpara.mode.iflget != 1`, WB13 `Ep` and storage closure evidence must show WB11 demand was seeded from `SC-EVAP-001#INV-EVAP-021` migrated PMET intermediates and actual seed branch `evappm_pmet` before assigning remaining residuals to SWU, WB18/WB19, or WB13 publication. A PMET-mode run whose `wb11_et_demand` is still Priestley-Taylor, coefficient-only tuned, or missing migrated `evappm` intermediate evidence is invalid closure evidence. | hard-fail | REF-WATBAL-LEGACY-WB13, REF-WATBAL-LEGACY-ORDER, SC-EVAP-001#INV-EVAP-021, INV-WATBAL-048 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-050 | HPHYS0264 WB11/WB17 PMET seam invariant: PMET-mode water-balance closure evidence must show the WB17 ET phase consumed `SC-EVAP-001#INV-EVAP-022` component lineage, with `Etp = pmet.ep_m` entering post-WB19 `swu`, non-negative `Es`/`Er` derived from `pmet.es_m`, only within-tolerance negative `Es` roundoff canonicalized to zero, and no Priestley-Taylor/LAI repartition applied to `pmet.ep_m`. WB13 `Ep` remains final only after `swu` uptake; closure evidence that treats pre-SWU PMET `ep` as final `Ep`, omits PMET `Es`, accepts material negative `Es`, or double partitions PMET `ep` is invalid. | hard-fail | REF-WATBAL-LEGACY-WB13, REF-WATBAL-LEGACY-ORDER, SC-EVAP-001#INV-EVAP-022, INV-WATBAL-049 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-051 | HPHYS0265 first-large longer-season `Ep` divergence invariant: after PMET seam correction, full-suite `Ep`/storage residual closure must identify the first H1/H7/H39 candidate/baseline WAT row where `|candidate Ep - baseline Ep| > 0.05 mm` and classify same-day context before assigning ownership. Required context includes candidate/baseline `Ep`, `Total-Soil`, `SoilWaterTotal`, `Dp`, `latqcc`, `Q`, `RM`, and `Snow-Water`, plus trace `SC-EVAP-001#INV-EVAP-023` WB17/SWU identity surfaces. If `Ep = ΣUi` and `Ws = Ep/Etp` close at that day while storage/snow/runoff/lateral residuals are already material, closure must remain `HOLD` on coupled upstream/storage ownership instead of claiming WB13 or WB17 publication closure. | governance-hold | REF-WATBAL-LEGACY-WB13, REF-WATBAL-LEGACY-ORDER, SC-EVAP-001#INV-EVAP-023, INV-WATBAL-046, INV-WATBAL-047, INV-WATBAL-050 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-052 | HPHYS0266 layer/lateral/snow first-divergence invariant: continuation evidence after `INV-WATBAL-051` must classify the same first H1/H7/H39 residual days with layer-distribution context before assigning seasonal `Ep` ownership. Required context includes WB11 aggregate storage, WB18 `theta` layer sum and recomputed-minus-WB11 residual, WB17 stress-layer storage/threshold ratios, WB19 potential/target/realized `q`, `Qdd`, `Qd`, active lateral layers, per-layer withdrawals, and same-day WAT `RM`, `Snow-Water`, `Q`, `Dp`, `latqcc`, `Total-Soil`, and `SoilWaterTotal`. If WB17 and WB19 identities close but SWU-stressed layers and WB19 active/withdrawal layers are materially separated, closure must remain `HOLD` on layer distribution plus snow/runoff/lateral magnitude context instead of patching WB17/SWU or WB13 publication. | governance-hold | INV-WATBAL-051, SC-SUBHYD-001#INV-SUBHYD-030, SC-EVAP-001#INV-EVAP-023, REF-WATBAL-LEGACY-WATCON, REF-WATBAL-LEGACY-WB13, REF-WATBAL-LEGACY-ORDER | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-053 | HPHYS0267 post-lateral/pre-SWU threshold-lineage invariant: after `INV-WATBAL-052`, residual ownership claims for first H1/H7/H39 seasonal `Ep` divergences must expose same-day pre-lateral, post-lateral, and post-SWU layer storage plus threshold lineage before changing production physics. Required evidence includes `wb18_perc_fc_i`, `coca_i`, `drfc_i`, `frzw_i`, `fzdrfc_i`, pre/post-lateral `theta_i`, realized lateral withdrawal by layer, WB17 `ul_i`, `pltol*ul_i`, storage-to-stress-threshold ratios, and same-day WAT storage/snow/runoff/lateral context. If these surfaces reconcile internally while semantic parity remains open, disposition must stay `HOLD` with narrowed ownership rather than patching WB17, WB19, or WB13 by compensation. | governance-hold | INV-WATBAL-052, SC-SUBHYD-001#INV-SUBHYD-031, SC-EVAP-001#INV-EVAP-024, REF-WATBAL-LEGACY-WATCON, REF-WATBAL-LEGACY-ORDER | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-054 | HPHYS0268 spring snowpack/SWE/`RM` lineage invariant: after HPHYS0267 rules out local post-lateral/pre-SWU threshold defects, first material H1/H7/H39 `Ep` divergence claims must expose WB13 `RM` and `Snow-Water` lineage, runtime SWE/depth/density/settle state, hourly rain/snow/melt sums, and signed `S` before reopening WB17 `Ep`. WB13 `Snow-Water` must be runtime SWE publication and `RM` must reconcile to the then-current declared snow/runoff lineage; HPHYS0289 supersedes the earlier precipitation-plus-SWE-delta diagnostic shortcut with baseline `rain + wmelt + irrigation` publication authority. Evidence that hides snowpack inputs or compensates through `Ep`, storage, or publication edits is invalid. | governance-hold | INV-WATBAL-053, SC-SNOWFREEZE-001#INV-SNOWFREEZE-014, SC-EVAP-001#INV-EVAP-024, REF-WATBAL-LEGACY-WB13, REF-WATBAL-LEGACY-WB13-RM-SNOW, REF-WATBAL-LEGACY-ORDER | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-055 | HPHYS0269/HPHYS0303 winter melt/snowpack baselining invariant: water-balance closure evidence must consume `SC-SNOWFREEZE-001#INV-SNOWFREEZE-015` before returning to WB17 `Ep` or storage residual tuning. WB13 `RM` must represent residual direct rain plus redistributed melt, and signed `S` must equal redistributed melt minus snowfall water equivalent minus rain retained in snowpack. Closure evidence that treats all rain-on-snow as immediate liquid forcing, clamps negative hourly melt before corrected daily redistribution, reproduces the archived original baseline negative-melt sign/branch bug, or hides retained-rain lineage is invalid. | hard-fail | INV-WATBAL-054, SC-SNOWFREEZE-001#INV-SNOWFREEZE-015, REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-SNOW, REF-WATBAL-LEGACY-WB13 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-056 | HPHYS0270 daily snowpack state closure invariant: WB13 `RM`/`Snow-Water` and WB17 `Ep` residual classifications must consume `SC-SNOWFREEZE-001#INV-SNOWFREEZE-016` daily carry-state evidence. The required evidence includes day-begin and post-day SWE/depth/density/settle-count state plus deltas so the classifier can distinguish day-begin publication lineage, same-day snowpack mutation, and downstream ET/storage response without compensating through WB13 publication, WB17 `Ep`, or aggregate storage. | governance-hold | INV-WATBAL-054, INV-WATBAL-055, SC-SNOWFREEZE-001#INV-SNOWFREEZE-016, REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-SNOW, REF-WATBAL-LEGACY-WB13 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-057 | HPHYS0271/HPHYS0272 day-36 melt-forcing closure invariant: WB13 `RM`/`Snow-Water`, WB17 `Ep`, and aggregate-storage residual classifications for the H1 sim-day 36 snowpack break must consume `SC-SNOWFREEZE-001#INV-SNOWFREEZE-017` term-level melt/hourly-forcing evidence and `SC-CLIMATE-001#INV-CLIMATE-013` radiation-unit evidence. Evidence that assigns day-36 residual ownership to publication, ET, storage, negative-melt redistribution, or heuristic radiation clipping without proving the `melt.for` trigger/magnitude and climate-radiation unit lineage is invalid. | governance-hold | INV-WATBAL-054, INV-WATBAL-055, INV-WATBAL-056, SC-SNOWFREEZE-001#INV-SNOWFREEZE-017, SC-CLIMATE-001#INV-CLIMATE-013, REF-WATBAL-CH5-BAL, REF-WATBAL-CH5-SNOW, REF-WATBAL-LEGACY-WB13 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-058 | HPHYS0283 spring meltwater partition invariant: daily closure must consume a WB12 partition in which routed snowmelt has been offered to infiltration before residual runoff is assigned, and WB18 must mutate same-pass layer/aggregate storage from that infiltrated melt before percolation and final `watcon` recomputation. `S` remains the signed snow-storage term (`melt - accumulation - retained rain`), but the corresponding positive meltwater cannot bypass infiltration/layer ingress and be subtracted only as `Q`. Spring storage-collapse residual ownership is invalid without this partition and storage-ingress evidence. | hard-fail | REF-WATBAL-LEGACY-WMELT-INFIL, REF-WATBAL-CH5-SNOW, SC-SNOWFREEZE-001#INV-SNOWFREEZE-018, SC-RUNOFFPART-001#INV-RUNOFFPART-015, SC-PERC-001#INV-PERC-016 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-059 | HPHYS0284 spring snow-retention invariant: WB13 `RM` and signed `S` consume the corrected redistributed routed melt from `SC-SNOWFREEZE-001#INV-SNOWFREEZE-019`, while WB13 `Snow-Water` and runtime snow carry-state consume the corrected depth/SWE state lineage. Closure evidence must not force `Snow-Water` to equal `previous SWE - routed net melt` on days with mixed positive/negative hourly melt; such a mass-closure shortcut hides the corrected legacy depth-state adjustment and can delay spring meltout. | hard-fail | INV-WATBAL-055, INV-WATBAL-056, INV-WATBAL-058, SC-SNOWFREEZE-001#INV-SNOWFREEZE-019, REF-WATBAL-LEGACY-WB13, REF-WATBAL-CH5-SNOW | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-060 | HPHYS0285 spring soil-retention invariant: daily closure and WB13 storage publication must consume WB18 layer/aggregate storage after positive same-pass WB12/WB14 local infiltration has entered `st(i)`/`wb18_perc_theta_i` through baseline `fin/xfin` lineage for direct rain, routed snowmelt, or irrigation. Active-snow state may affect snowmelt supply and snow-storage terms, but it cannot gate whether non-snow local infiltration mutates profile storage. Hourly lanes must preserve `xfin = fin/ui_LFtstpF` substep cadence before each percolation substep. Soil-storage residual ownership is invalid while positive local `wb12_infiltration` is publication-only and absent from `Total-Soil`/`SoilWaterTotal` lineage. MOFE carry/runon storage-ingress promotion remains follow-up scope under `INV-WATBAL-033`/`INV-WATBAL-034`. | hard-fail | REF-WATBAL-LEGACY-HOURLY-FIN, REF-WATBAL-LEGACY-WATCON, SC-RUNOFFPART-001#INV-RUNOFFPART-016, SC-PERC-001#INV-PERC-017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-061 | HPHYS0286 post-ingress retention/WB18-WB17 coupling invariant: daily/hourly closure and WB13 storage publication must consume post-ET layer storage after baseline lower-layer upper-limit redistribution has run. After WB17 soil evaporation and before WB19 drainage/lateral, layers `i=nsl..2` with `st(i)` above active cap must move excess upward to `st(i-1)`; positive same-pass outside water (`fin > 1.0e-6`) uses `max(ul(i)-frzw(i),0)`, otherwise `ul(i)`. Closure evidence that leaves lower layers above active cap, discards excess, mutates only scalar aggregate storage, or compensates `Total-Soil`/`SoilWaterTotal` at WB13 publication is invalid. | hard-fail | REF-WATBAL-LEGACY-HOURLY-POSTET-UL, REF-WATBAL-LEGACY-WATCON, SC-EVAP-001#INV-EVAP-026, SC-PERC-001#INV-PERC-018 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-062 | HPHYS0287 snow liquid partition/storage publication invariant: daily closure, WB13 `RM`/`Snow-Water`, and WB13 `Total-Soil`/`SoilWaterTotal` publication must consume a WB12/WB14 snow-liquid partition that first validates runtime snow-state domains. When any snow option/control/runtime state is projected, missing runtime snow vector members cannot be zero-defaulted to keep publication running. Material negative or non-finite snow runtime state cannot be treated as inactive snow and zero-published to keep direct rain/melt infiltration running. Valid non-snow infiltration remains available to WB18/WB11 storage, but invalid snow state is a typed upstream domain failure rather than a closure residual or publication-compensation path. | hard-fail | SC-SNOWFREEZE-001#INV-SNOWFREEZE-020, SC-RUNOFFPART-001#INV-RUNOFFPART-017, INV-WATBAL-058, INV-WATBAL-060, REF-WATBAL-CH5-SNOW, REF-WATBAL-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-063 | HPHYS0288 rain-on-snow `RM`/storage-forcing invariant: daily WB13 `RM`, WB12 runoff, and WB18 same-pass storage-ingress closure must consume residual rain-on-snow released by `snowd.for` holding-capacity accounting through the `winter.for` `hrmlt`/`wmelt` lineage. Direct rain available to the hyetograph/storage path must exclude retained rain and residual rain-on-snow that has been promoted into `wmelt`; retained rain remains snow-storage gain in signed `S`. Closure evidence that treats released rain-on-snow as raw direct precipitation only, omits it from `wmelt`, or double counts it in both direct-rain and routed-melt forcing is invalid. | hard-fail | REF-WATBAL-LEGACY-WINTER-RAINRELEASE, REF-WATBAL-LEGACY-WMELT-INFIL, REF-WATBAL-LEGACY-HOURLY-FIN, SC-SNOWFREEZE-001#INV-SNOWFREEZE-021, SC-RUNOFFPART-001#INV-RUNOFFPART-018, INV-WATBAL-058, INV-WATBAL-060 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-064 | HPHYS0289 WB13 `RM`/`Snow-Water` publication invariant: daily WB13 publication must consume the baseline output lineage `RM = rain(iplane) + wmelt(iplane) + irdept(iplane) + iraplo(iplane)` and `Snow-Water = snodpy(iplane) * densg(iplane)`. For winter-active snow-coupled days, post-winter `rain(iplane)` is zero unless the warm-rain/no-snow restoration branch applies; routed daily `wmelt` is the authoritative snowmelt liquid term. A WB13 `RM` calculation based on raw `prcp + previous_SWE - current_SWE + Irr` is a diagnostic proxy, not publication authority, and is invalid when it bypasses routed `wmelt` or reintroduces precipitation that baseline winter processing cleared. | hard-fail | REF-WATBAL-LEGACY-WB13-RM-SNOW, REF-WATBAL-LEGACY-WINTER-RAINRELEASE, REF-WATBAL-LEGACY-WMELT-INFIL, SC-SNOWFREEZE-001#INV-SNOWFREEZE-022, SC-RUNOFFPART-001#INV-RUNOFFPART-019, INV-WATBAL-063 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-065 | HPHYS0290 post-winter rain publication invariant: WB13 `RM` must consume explicit finite non-negative runtime surfaces for both post-winter `rain(iplane)` and routed `wmelt(iplane)`, plus irrigation. The openWEPP post-winter rain alias is `snow.post_winter_rain_m`; it is the WB12/WB14 liquid term left on the direct-rain path after snow retention, rain-on-snow release, and winter clearing/restoration have executed. WB13 may not infer post-winter rain from raw `prcp`, runtime SWE, snow-active state, or `snow.routed_melt_m`; missing, negative, non-finite, or stale-state-shadowed `snow.post_winter_rain_m` is a typed publication failure. | hard-fail | REF-WATBAL-LEGACY-WB13-RM-SNOW, REF-WATBAL-LEGACY-WINTER-RAINRELEASE, SC-SNOWFREEZE-001#INV-SNOWFREEZE-023, SC-RUNOFFPART-001#INV-RUNOFFPART-020, INV-WATBAL-064 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-066 | HPHYS0291 snow publication lifecycle invariant: WB13 `RM` publication is downstream of a same-day producer flux lifecycle for `snow.post_winter_rain_m` and `snow.routed_melt_m`. WB13 must consume producer-owned fluxes published before the row is built, must reject absent/non-finite/negative post-winter rain, and must not accept state/default substitutions or raw-precipitation reconstruction to keep daily rows running. Trace and full-suite evidence must preserve this lifecycle before assigning remaining snowpack, runoff, storage, or `Ep` residual ownership. | hard-fail | SC-SNOWFREEZE-001#INV-SNOWFREEZE-024, SC-RUNOFFPART-001#INV-RUNOFFPART-021, INV-WATBAL-064, INV-WATBAL-065 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-067 | HPHYS0292 spring snowmelt/infiltration capacity lineage invariant: after WB13 consumes same-day snow publication fluxes, spring H1/H7/H39 `Total-Soil`/`SoilWaterTotal` collapse residual ownership must consume producer-side partition evidence from `winter/snowd`, `wmelt -> fin/smrate`, WB12 cumulative infiltration, residual `Q`, and WB18 same-pass storage ingress. Evidence must distinguish excessive/early routed melt from insufficient infiltration capacity; it must not assign the residual to WB17 `Ep`, aggregate storage, or WB13 publication while capacity inputs and producer melt timing remain unresolved. | governance-hold | REF-WATBAL-LEGACY-WINTER-RAINRELEASE, REF-WATBAL-LEGACY-WMELT-INFIL, INV-WATBAL-058, INV-WATBAL-063, INV-WATBAL-066, SC-SNOWFREEZE-001#INV-SNOWFREEZE-025, SC-RUNOFFPART-001#INV-RUNOFFPART-022 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-068 | HPHYS0293/HPHYS0303 snow producer versus post-ingress storage attribution invariant: after HPHYS0292 demonstrates `Q` parity and WB12 capacity closure, daily `Total-Soil`/`SoilWaterTotal` attribution must first classify `Snow-Water`/`RM` timing from snow producer evidence (`hrmelt_raw`, redistributed melt, retained/released rain, runtime SWE/depth/density before/after, WB13 publication). A storage/percolation/lateral package may proceed only after the snow producer is either corrected or explicitly excluded by trace evidence. Fixed-comparator negative-melt state authority may explain archived-original comparator residuals without authorizing empirical compensation in WB18/WB19/WB17. | governance-hold | INV-WATBAL-067, SC-SNOWFREEZE-001#INV-SNOWFREEZE-026, SC-RUNOFFPART-001#INV-RUNOFFPART-023, REF-WATBAL-LEGACY-WMELT-INFIL, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-069 | HPHYS0294 post-ingress storage/percolation/lateral attribution invariant: with HPHYS0293 snow producer residuals carried as excluded comparator differences and HPHYS0292 `Q` parity closed, daily `Total-Soil`/`SoilWaterTotal` residual ownership must consume WB18/WB19 trace evidence before production edits: aggregate `watcon` identity, same-pass infiltration ingress, `D=Pe`, per-layer `pei`, WB19 lateral potential/target/realized/unrealized terms, and cumulative snow/`RM` residual masks. Mixed residual direction across H1/H7/H39 is not sufficient proof of a WB18/WB19 defect without row-level magnitude accounting. | governance-hold | INV-WATBAL-068, INV-WATBAL-061, SC-PERC-001#INV-PERC-019, SC-SNOWFREEZE-001#INV-SNOWFREEZE-026, SC-RUNOFFPART-001#INV-RUNOFFPART-023 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-070 | HPHYS0295 cumulative storage-budget ownership invariant: after HPHYS0294 closes local WB18 aggregate identity, `D=Pe`, and WB19 target/unrealized lineage, H1/H7/H39 `Total-Soil`/`SoilWaterTotal` residual ownership must be assigned from cumulative row-to-row accounting across candidate/baseline storage deltas, `Ep`, `Es`, `Er`, `D`, `latqcc`, `Q`, `RM`, `Snow-Water`, WB18/WB19 trace identities, and HPHYS0293 excluded snow-producer masks. Production edits to WB17, WB18, WB19, or WB13 are invalid until the cumulative budget proves a process owner that survives per-day timing and excluded snow/`RM` residual separation. | governance-hold | INV-WATBAL-069, INV-WATBAL-068, SC-EVAP-001#INV-EVAP-027, SC-PERC-001#INV-PERC-019, SC-SUBHYD-001#INV-SUBHYD-031 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-071 | HPHYS0296/HPHYS0303 snow/`RM` acceptance invariant: after cumulative accounting assigns dominant residual ownership to snow/`RM`, fixed-comparator negative-melt evidence, candidate WB13 publication identity, and snow-state closure are diagnostic evidence only and may not by themselves accept a residual as semantic-not-bit divergence. Before any window leaves the failing set, water-balance closure must reference the `SC-SNOWFREEZE-001#INV-SNOWFREEZE-027` per-window defective-model verdict: mechanistic `file:line` root cause in both openWEPP and the active fixed comparator or archived original comparator as applicable, reconstruction controlled experiment to named tolerance, independent correctness adjudication, and explicit `LEGACY-DEFECTIVE`/`OPENWEPP-DEFECTIVE`/`UNRESOLVED` disposition. `LEGACY-DEFECTIVE` windows may be auditable documented-legacy-defective re-tierings; `OPENWEPP-DEFECTIVE` and `UNRESOLVED` windows remain failing/HOLD. No WB17/WB18/WB19/WB13 downstream compensation is allowed. | governance-hold | INV-WATBAL-070, INV-WATBAL-068, SC-SNOWFREEZE-001#INV-SNOWFREEZE-027, SC-RUNOFFPART-001#INV-RUNOFFPART-024 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-072 | HPHYS0297 snow/`RM` defect-ledger water-balance invariant: water-balance residual accounting must consume the HPHYS0297 ledger before re-tiering any snow/`RM` window or returning focus to WB17/WB18/WB19. A valid ledger row must include observed candidate/baseline `RM`, reconstructed `/workdir/wepp-forest_260430_baseline` branch `RM`, reconstruction residual to named tolerance, closed `Q` and producer-consumer identity evidence, independent correctness rationale, and a final `LEGACY-DEFECTIVE`/`OPENWEPP-DEFECTIVE`/`UNRESOLVED` verdict. Rows with reconstruction residual outside tolerance remain failing/HOLD; they cannot be excluded from semantic accounting. | governance-hold | INV-WATBAL-071, INV-WATBAL-070, SC-SNOWFREEZE-001#INV-SNOWFREEZE-028, SC-RUNOFFPART-001#INV-RUNOFFPART-025 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-073 | HPHYS0298 paired snow/`RM` source-partition invariant: water-balance residual accounting must consume the HPHYS0298 partition ledger before re-tiering any of the nine H1/H7/H39 target windows or returning focus to WB17/WB18/WB19/WB13. A valid ledger row must include baseline observe identity status, full H1..H39 same-HEAD metrics, candidate/baseline `RM` and residual, ordered first-divergent cut-point, canonical symbol values and units, baseline and openWEPP source-line provenance, closed `Q` and WB13 `RM` identity status, prohibited compensation paths, independent correctness rationale, and a final `LEGACY-DEFECTIVE`/`OPENWEPP-DEFECTIVE`/`UNRESOLVED` verdict. Observe-identity failure, missing first-divergence evidence, or missing verdict keeps the window failing/HOLD and prevents downstream WB17/WB18/WB19/WB13 compensation. | governance-hold | INV-WATBAL-072, INV-WATBAL-071, SC-SNOWFREEZE-001#INV-SNOWFREEZE-029, SC-RUNOFFPART-001#INV-RUNOFFPART-026 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-074 | HPHYS0299 hourly snow partition unit/provenance invariant: water-balance residual accounting must consume the corrected HPHYS0299 unit/provenance ledger before treating HPHYS0298 `hourly-forcing` verdicts as production migration authority or returning focus to WB17/WB18/WB19/WB13. The corrected ledger must compare pinned-baseline `stmtim.for` `hrsnow` snow-depth to openWEPP `snow_hourly_snowfall_depth_sum_m`; the derived `snow_hourly_snowfall_water_equiv_sum_m` surface may support SWE diagnostics but is invalid as canonical `hrsnow` parity evidence. If corrected depth-vs-depth evidence closes the apparent `hrsnow` residual, the old HPHYS0298 all-window `OPENWEPP-DEFECTIVE` verdict is non-authoritative for production migration and continuation routing must be recalculated from full H1..H39 same-HEAD metrics. | governance-hold | INV-WATBAL-073, SC-SNOWFREEZE-001#INV-SNOWFREEZE-030, SC-CLIMATE-001#INV-CLIMATE-014 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-075 | HPHYS0300 raw hourly melt/post-raw routing water-balance invariant: water-balance continuation after HPHYS0299 must classify the nine H1/H7/H39 target windows from corrected forcing status, raw `hrmlt`, post-raw `wmelt`/routed melt, WB13 `RM`, `Q`, `Snow-Water`, and full H1..H39 same-HEAD metrics before re-tiering or returning to WB17/WB18/WB19. Production water-balance edits are invalid while the first divergent source is raw/post-raw snow producer lineage or corrected-depth hourly forcing. A valid HPHYS0300 ledger must preserve the H7 first-2013 `baseline_negative_raw_melt_sum_mm = 0.0` finding, keep H39 first-2013 separate as hourly forcing, require term/state lineage evidence before snow producer edits, and explicitly prohibit compensation through `Ep`, aggregate storage, percolation, lateral flow, or WB13 publication. | governance-hold | INV-WATBAL-074, INV-WATBAL-073, SC-SNOWFREEZE-001#INV-SNOWFREEZE-031, SC-RUNOFFPART-001#INV-RUNOFFPART-026 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-076 | HPHYS0301 H39 rain-release water-balance invariant: water-balance continuation must consume the HPHYS0301 H39 first-2013 residual-rain/release ledger before authorizing any hourly forcing, raw-melt, routed-melt, or downstream consumer edit. The HPHYS0300 raw-rain aggregate delta is invalid as production forcing authority when baseline evidence is residual `hrrain` after rain-on-snow retention/release and openWEPP evidence is raw `snow_hourly_rain_sum_m`; valid comparison uses openWEPP released plus post-winter rain. If source-line forcing root cause is not proven, the row remains `HOLD` under rain-retention/post-raw melt lineage and still requires paired `melt.for`/`snowd.for` term/state evidence. WB17/WB18/WB19/WB13 compensation remains prohibited. | governance-hold | INV-WATBAL-075, INV-WATBAL-074, SC-SNOWFREEZE-001#INV-SNOWFREEZE-032 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-077 | HPHYS0302 water-balance comparator-surface invariant: before any WB17/WB18/WB19/WB13 or snow-producer edit is authorized from H1/H7/H39 target-window residuals, the evidence must prove baseline and openWEPP comparator surfaces represent the same physical quantity in the same units. Daily WAT/WB13 `RM` and `Snow-Water` comparisons are publication/output-surface evidence only; raw `hrmlt` and post-raw `wmelt` aggregate comparisons may localize upstream cut-points but do not prove `amelt`/`bmelt`/`cmelt`/`dmelt` producer defects. Aggregate deltas without paired term/state surfaces remain `HOLD`; downstream water-balance compensation remains prohibited. | governance-hold | INV-WATBAL-076, INV-WATBAL-075, SC-SNOWFREEZE-001#INV-SNOWFREEZE-033 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-078 | HPHYS0305 paired melt-term/state instrumentation invariant: H1/H7/H39 snow/`RM` continuation may not authorize snow producer, forcing, WB13, WB17, WB18, WB19, or WB12 production edits until fixed-comparator baseline observe evidence and openWEPP trace evidence expose paired same-unit term/state surfaces for `amelt`, `bmelt`, `cmelt`, `dmelt`, `hrrain`, `hrtemp`, `tdpt`, `hrad`, `cloudC`, `vwind`, `snodpt`, and `densgt` over all nine target windows. The HPHYS0305 ledger must classify a first divergent source or record a concrete blocker per window; missing paired surfaces keep the row in `HOLD`. | governance-hold | INV-WATBAL-077, INV-WATBAL-075, SC-SNOWFREEZE-001#INV-SNOWFREEZE-033 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-079 | HPHYS0306 branch-active melt-term observe invariant: fixed-baseline `amelt`/`bmelt`/`cmelt`/`dmelt` term values are authoritative only for hours where `melt.for` is actually called and the terms are computed before `wmelt = 0.0254 * (amelt + bmelt + cmelt + dmelt)`. Inactive hours with no baseline melt call are not implicit zero-valued term observations. Paired ledgers must first compare the baseline melt-call key set against openWEPP `snow_hourly_melt_branch_active`; any key-set mismatch is a `branch-active-mask-hold`. Only after the active masks match may forcing, snow-state, or melt-term magnitudes be compared on that branch-active domain. Inactive-hour zero-imputation, treating openWEPP inactive trace publication as baseline term authority, or downstream compensation remains invalid. | governance-hold | INV-WATBAL-078, INV-WATBAL-077, SC-SNOWFREEZE-001#INV-SNOWFREEZE-033 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-080 | HPHYS0307 melt-call branch activation lineage invariant: branch-active mask gaps must be classified from baseline control-flow provenance before any snow-producer or downstream water-balance edit. The fixed-comparator baseline calls `snowd.for` once for every winter hour from `winter.for`, and `snowd.for` calls `melt.for` only when an existing snowpack has entered the non-freezing daily-mean branch and `snodep > 0.0` after adding new snow/drift inputs. openWEPP `snow_hourly_melt_branch_active` is an observation of the corresponding `compute_simimpl29_melt_hour` branch, not a melt-magnitude or publication-success proxy. Evidence must distinguish `baseline-extra-melt-call`, `openwepp-extra-melt-call`, matched active-domain term/state divergence, and trace-parser conflict lanes. Production code edits are authorized only for a source-line-owned openWEPP branch-predicate defect; otherwise rows remain `HOLD`, and WB13/WB17/WB18/WB19/WB12 compensation remains invalid. | governance-hold | INV-WATBAL-079, INV-WATBAL-078, SC-SNOWFREEZE-001#INV-SNOWFREEZE-033 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-081 | HPHYS0308 branch-extra state-ordering invariant: after a branch-active mask gap is classified by `INV-WATBAL-080`, closure evidence must inspect the branch-extra timestamps rather than infer a branch-predicate defect from aggregate counts. Required evidence includes the fixed-baseline melt-call predicate outcome, baseline `snodpt`/`densgt` and melt-term observations, openWEPP `snow_hourly_melt_branch_active`, `snow_hourly_depth_before_m`, `snow_hourly_snowfall_depth_m`, `snow_hourly_depth_available_m`, `snow_hourly_depth_after_m`, and corresponding density/forcing surfaces at each extra key. Rows where openWEPP is inactive because snow depth is already zero while baseline still calls `melt.for` are snow-state carry/depletion holds, not branch-predicate edit authority. Rows where openWEPP is active and baseline has no melt observation require baseline branch-condition/state-ordering instrumentation before any production edit. Downstream WB13/WB17/WB18/WB19/WB12 compensation remains invalid. | governance-hold | INV-WATBAL-080, INV-WATBAL-079, SC-SNOWFREEZE-001#INV-SNOWFREEZE-033 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-082 | HPHYS0309 snow carry/depletion water-balance invariant: after HPHYS0308 routes baseline-extra melt-call keys to snow-state carry/depletion, water-balance continuation must consume a carry-lineage ledger before assigning ownership to melt terms, branch predicates, WB13 publication, WB17 ET, WB18 storage, WB19 lateral/percolation, or WB12 runoff. The ledger must compare fixed-comparator prior-day hour-24 and same-day after-hour `snodpt`/`densgt` against openWEPP `snow_runtime_depth_before_m`, `snow_runtime_swe_before_m`, hourly before/after depth, same-day zero-depth hour, and depletion lead hours. Rows explained by pre-day carry deficit or prior-day openWEPP meltout remain snow carry-state `HOLD`; they are not valid authority for downstream compensation or same-hour term tuning. | governance-hold | INV-WATBAL-081, INV-WATBAL-080, SC-SNOWFREEZE-001#INV-SNOWFREEZE-034, INV-WATBAL-068 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-083 | HPHYS0310 prior-day snow carry water-balance invariant: water-balance continuation after HPHYS0309 must consume an episode-level snow carry divergence ledger before assigning ownership to melt terms, branch predicates, WB13 publication, WB17 ET, WB18 storage, WB19 lateral/percolation, or WB12 runoff. The ledger must cover every affected HPHYS0309 hillslope/window/year group, identify the first material paired fixed-comparator/openWEPP snowpack divergence before the key day, publish candidate source-lane aggregates for initial carry-state projection, snowfall, raw melt, routed melt, retained/released rain proxies, density/depth settling, and corrected negative-melt state loss, and keep all downstream compensation invalid unless a source-line-owned openWEPP carry-state defect is proven. | governance-hold | INV-WATBAL-082, INV-WATBAL-081, SC-SNOWFREEZE-001#INV-SNOWFREEZE-035, SC-SNOWFREEZE-001#INV-SNOWFREEZE-034 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-084 | HPHYS0311 snow carry source-line water-balance invariant: before assigning ownership for HPHYS0310 carry-state rows to melt terms, branch predicates, WB13 publication, WB17 ET, WB18 storage, WB19 lateral/percolation, or WB12 runoff, water-balance continuation must consume a source-line parity ledger that distinguishes inherited prior-year terminal snowpack deltas from projection/update defects. The ledger must cover all seven HPHYS0310 groups, compare fixed-comparator prior-year terminal `snodpt`/`densgt`, day-1 hour-1 carried state, previous/current density-settling state, and openWEPP runtime depth/density/SWE aliases, and explicitly mark downstream compensation invalid unless source-line evidence proves an openWEPP-owned carry-state defect. | governance-hold | INV-WATBAL-083, INV-WATBAL-082, SC-SNOWFREEZE-001#INV-SNOWFREEZE-036 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-085 | HPHYS0312 prior-year terminal snowpack water-balance invariant: before routing inherited prior-year terminal snowpack deltas to melt terms, branch predicates, WB13 `RM`/`Snow-Water`, WB17 ET, WB18 storage, WB19 lateral/percolation, or WB12 runoff, water-balance continuation must consume a prior-calendar-year lineage ledger for each HPHYS0311 `prior-year-terminal-state-hold` group. The ledger must identify the first material paired snowpack divergence within the scanned year or explicitly classify the divergence as already present at year-start, preserve material depth/density tolerances, cite baseline/openWEPP source-line lanes for the classified process, and keep downstream compensation invalid unless a source-line-owned openWEPP snow carry-state defect is proven. | governance-hold | INV-WATBAL-084, INV-WATBAL-083, SC-SNOWFREEZE-001#INV-SNOWFREEZE-037 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-WATBAL-086 | HPHYS0313 split-route snowpack water-balance invariant: water-balance continuation after HPHYS0312 must separately consume evidence for the settling/depth-update route and the earlier carry-state route before assigning ownership to WB13 `RM`/`Snow-Water`, WB17 ET, WB18 storage, WB19 lateral/percolation, WB12 runoff, melt terms, or branch predicates. Settling-route evidence must include full-precision pinned-baseline `wdayct`, `densgy`, `setf`, `densgt`, post-settling depth, actual M3 branch selection from `hrsnow`, branch final depth, and branch input terms at the first material 2013 day 11 hour 11 divergence. If `hrsnow > 0`, evidence must cite the snowing branch at `snowd.for:166-172` and compare pinned-baseline `hrsnow` to homologous openWEPP hourly snowfall before assigning water-balance ownership; if `hrsnow <= 0`, evidence may cite the no-snow `driftg` branch at `snowd.for:145-146` only with branch-gated proof. Year-start-route evidence must recurse into the 2014 terminal carry-state chain feeding 2015 day 1 hour 1 and classify the first material paired divergence. Aggregate water-balance improvement, rounded observe output, or compensation through downstream stores is invalid while either route remains source-line `HOLD`. | governance-hold | INV-WATBAL-085, INV-WATBAL-084, SC-SNOWFREEZE-001#INV-SNOWFREEZE-038 | `[DIRECT][Static] + [INFERENCE][Static]` |

### HPHYS0298 Water-Balance Disposition Addendum

For HPHYS0298, water-balance accounting treats the all-window
`OPENWEPP-DEFECTIVE` verdict as a porting-fidelity defect in openWEPP's
producer-side hourly precipitation-phase partition, not as downstream storage,
percolation, ET, runoff, or WB13 publication ownership. The active authority is
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-029` plus the paired ledger showing first
divergence at `hourly-forcing` (`hrsnow`, and `hrrain`/`hrsnow` for H39
first-2013) against the then-declared pinned-baseline hourly partition
behavior. HPHYS0299 amends that authority: production migration remains
`HOLD` until corrected depth-vs-depth evidence compares baseline `stmtim.for`
`hrsnow` with openWEPP `snow_hourly_snowfall_depth_sum_m`. WB17, WB18, WB19,
and WB13 changes remain prohibited as compensation.

### HPHYS0299 Water-Balance Unit/Provenance Addendum

Water-balance continuation routing must not treat a depth-vs-water-equivalent
diagnostic mismatch as producer physics evidence. The corrected HPHYS0299
ledger is now required before reusing HPHYS0298 `hourly-forcing` verdicts:
canonical `hrsnow` is snowfall depth, the openWEPP parity summary is
`snow_hourly_snowfall_depth_sum_m`, and `snow_hourly_snowfall_water_equiv_sum_m`
is a derived SWE-related summary for different diagnostics.

### HPHYS0300 Water-Balance Raw/Post-Raw Addendum

HPHYS0300 water-balance closure is a producer-lineage classifier. It may use
full-suite H1..H39 metrics to quantify continuation impact, but those metrics
cannot authorize WB17, WB18, WB19, or WB13 compensation while raw `hrmlt`,
post-raw `wmelt`, or corrected-depth hourly forcing remains the first
divergent source. Rows lacking term/state evidence remain `HOLD`, not
semantic closure.

The HPHYS0300 evidence gate is bounded. Once paired baseline/openWEPP
term/state evidence isolates a raw-melt or post-raw source to a named
producer-side term/state input with units and source-line provenance, the next
package must either implement the baseline-authoritative producer correction or
record the blocking invariant that prevents it. It must not route the same
isolated source into another diagnostic-only package, and it must keep H39
first-2013 corrected-depth hourly forcing on a separate actionable correction
lane instead of waiting for raw-melt term instrumentation.

### HPHYS0301 H39 Rain-Release Water-Balance Addendum

HPHYS0301 water-balance continuation supersedes any production edit claim that
compares H39 first-2013 baseline residual rain-on-snow evidence to openWEPP raw
rain. The valid water-balance comparison for that evidence class is baseline
residual `hrrain` against openWEPP released rain plus `snow.post_winter_rain_m`.
When that reconciliation removes the material raw-rain aggregate delta and no
source-line raw forcing defect is proven, H39 first-2013 remains a snow
producer `HOLD` for paired rain-retention/raw-melt/post-raw evidence. It does
not authorize WB17, WB18, WB19, WB13, or forcing-code compensation.

### HPHYS0302 Comparator-Surface Water-Balance Addendum

HPHYS0302 requires water-balance diagnostics to distinguish publication
surfaces from producer surfaces before assigning residual ownership. `RM` and
`Snow-Water` daily WAT/WB13 comparisons may prove output-surface parity or
residual magnitude, while raw `hrmlt` and post-raw `wmelt` comparisons may
bound aggregate cut-points. They do not authorize WB17/WB18/WB19/WB13
compensation or snow-producer edits. A production correction requires paired
baseline/openWEPP term-state evidence for the melt terms and forcing/state
inputs named in `SC-SNOWFREEZE-001#INV-SNOWFREEZE-033`.

### HPHYS0305 Paired Melt-Term/State Addendum

HPHYS0305 is an evidence gate, not a production-correction package. It may add
diagnostic observe/trace surfaces so fixed-baseline `melt.for`/`snowd.for`
symbols can be compared against openWEPP aliases, but it must keep production
physics unchanged unless paired evidence isolates a named source and a
follow-on package implements the source-owned correction under canonical
contract authority. Required baseline/openWEPP pairings are:

- `amelt`/`bmelt`/`cmelt`/`dmelt` -> `snow_hourly_melt_*_in`
- `hrrain` -> `snow_hourly_rain_m`
- `hrtemp`/`tdpt`/`hrad`/`cloudC`/`vwind` -> `winter_hourly_*`
- `snodpt`/`densgt` -> `snow_hourly_depth_after_m` and
  `snow_hourly_density_after_kg_m3`

### HPHYS0306 Branch-Active Melt-Term Observe Addendum

HPHYS0306 closes the HPHYS0305 missing-`amelt` blocker by separating missing
baseline term observations from inactive fixed-baseline melt hours. The
baseline branch-active domain is the set of fixed-comparator observe keys where
`melt.for` reached the paired `amelt`/`bmelt`/`cmelt`/`dmelt` observation after
term computation. The openWEPP branch-active domain is the set of
`snow_hourly_melt_branch_active` keys with `true` values. These domains are
compared before numeric term-state comparisons:

- baseline inactive + openWEPP inactive: skip term/forcing/state comparison;
- baseline active + openWEPP inactive or baseline inactive + openWEPP active:
  route `branch-active-mask-hold`;
- baseline active + openWEPP active: compare paired same-unit
  forcing/state/term surfaces and classify the first source.

No package may convert inactive fixed-baseline hours into zero-valued
`amelt`/`bmelt`/`cmelt`/`dmelt` observations unless a later canonical contract
amendment cites baseline code that explicitly stores such inactive values.

### HPHYS0307 Melt-Call Branch Activation Lineage Addendum

HPHYS0307 routes branch-active mask gaps by control-flow source, not by
downstream `RM`, `Snow-Water`, `Total-Soil`, or melt-magnitude residuals.
Baseline branch activation uses source-line provenance:

- `winter.for` calls `snowd.for` for every winter hour before writing
  `hrmlt(hour,iplane) = wmelt(iplane)`;
- `snowd.for` initializes no-snow/no-snowfall and freezing daily-mean lanes
  without calling `melt.for`;
- `snowd.for` calls `melt.for` only in the existing-snowpack,
  non-freezing-daily-mean lane when post-input `snodep > 0.0`;
- openWEPP `snow_hourly_melt_branch_active` must represent the analogous
  branch predicate where `compute_simimpl29_melt_hour` is invoked.

The HPHYS0307 ledger must classify each row as `baseline-extra-melt-call`,
`openwepp-extra-melt-call`, matched active-domain source divergence, or
trace-parser conflict. A row with only classification evidence and no
source-line-owned openWEPP defect remains `HOLD`.

### HPHYS0308 Branch-Extra State-Ordering Addendum

HPHYS0308 treats branch-extra keys as timestamp-level state-ordering evidence.
The branch predicate may be edited only after the key-level state proves that
openWEPP evaluates the same baseline state on the wrong condition. Otherwise:

- baseline-extra keys where openWEPP snow depth is already zero are
  `snow-state-carry-depletion-hold`;
- openWEPP-extra keys where fixed-baseline has no paired `melt.for`
  observation are `baseline-branch-instrumentation-hold`;
- matched branch-active keys with same-hour `cmelt`/`snodpt` divergence remain
  source-ordering holds;
- aggregate `RM`, `Snow-Water`, `Total-Soil`, or `SoilWaterTotal` residuals
  cannot authorize branch-predicate edits.

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-WATBAL-001` | runtime | Daily closure assembler for Eq. [5.1.1] | Typed hard error on residual beyond tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-002` | runtime | Daily state/flux domain validator | Typed hard error on negative magnitudes or invalid `S` sign convention | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-003` | runtime | Interception calculation + post-calc bounds check | Typed hard error for invalid `VE` domain or `I` out of `[0,P]` | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-004` | runtime | ET distribution and stress-factor computation path | Typed hard error on invalid `Ws`, `Ui`, or `UPi` domain relation, or undefined denominator handling when `Etp = 0` | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-005` | runtime | Soil-evaporation depth updater | Typed hard error on invalid `ds`/`dx` relation or residual-moisture violation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-006` | runtime | Percolation routing by layer | Typed hard error on invalid percolation eligibility/conductivity domains | Tier-A gate | `[DIRECT][Static]` |
| `INV-WATBAL-007` | runtime | Component-boundary payload validator | Typed hard error on missing/invalid required coupling surfaces | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-008` | governance | Contract review/disposition/verification and promotion checklist | Promotion `HOLD` until cross-contract authority for reuse of `D` is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-009` | runtime | WB17 ET + WB18 perc + WB19 lateral/drain production kernel execution paths | Typed hard error on non-deterministic/malformed hydrology writeback outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-010` | runtime | WB19 routing + guard tables | Typed hard error on unsupported phase classes or WB19 domain-invalid lateral/drainage inputs/outputs | Tier-A gate | `[INFERENCE][Static]` |
| `INV-WATBAL-011` | runtime | Scheduler phase closure and coupled lane-entry guard between growth dispatch and hydrology execution | Typed hard error on ordering-precondition violation and halt before watbal completion | Tier-A gate for INT10 coupled replay | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-012` | runtime | WB13 replay-candidate staging gate before strict comparator execution | Typed hard error on missing/invalid WB13 replay rows or missing replay artifacts; no schema rewrite/fallback padding | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-013` | runtime | WB12 storage reconciliation with active CLIM05 snow-coupled `S` term | Typed hard error on missing/non-finite/domain-invalid `S` or violated CLIM05 storage closure equation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-014` | runtime + governance | PL14R strict replay interchange-surface staging gate | Typed hard error / explicit `HOLD` when candidate lane lacks required interchange surfaces (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`) or uses fallback substitution | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-015` | governance | PL15R schema-aligned WB13 replay reclassification gate | Governance `HOLD` when active Tier-A WB13 blocker classification ignores superseding schema-aligned strict-pass/day-parity evidence | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-016` | runtime | WB12 runoff/storage reconciliation lane selector and closure-delta assembler | Typed hard error when forward lane consumes excluded observed targets or emits non-residual closure deltas | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-017` | runtime + governance | PL14S semantic comparator report schema/content gate | Typed hard error / explicit `HOLD` when semantic report omits row-presence deltas, per-column tolerance verdicts, required investigation diagnostics, or baseline-only column disclosure | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-018` | runtime | Runner-to-orchestrator execution provenance gate for watbal publication surfaces | Typed hard error (`HS-SIMPIPE-E-001`) when required surfaces are published without executed lane provenance | SIMIMPL execution gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-019` | runtime | `wepp_ui` effective-mode to watbal-lane selector closure guard | Typed hard error (`HS-SIMMODE-E-001`) on missing mode surfaces or lane/mode mismatch | SIMIMPL execution gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-020` | runtime + governance | WB13 simulation-owned publication provenance gate | Typed hard error / explicit `HOLD` (`HS-SIMOUT-E-001`) on projection-only/synthetic WB13 publication for required candidate surfaces | Tier-A replay integrity gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-021` | governance | Consolidated-kernel intake triage authority gate | Governance `HOLD` (`HS-SIMCONS-E-001`) when consolidated kernels/policies are adopted without explicit provenance triage and guard disposition | Consolidated-intake gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-022` | runtime | Continuous run-span and WB13 row-key closure validator at runner publication boundary | Typed hard error (`HS-SIMOUT-E-001`) on climate-span under-run, non-monotonic `sim_day_index`, row-count mismatch, or non-simulation-year key mapping | Tier-A replay span/key gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-023` | runtime + governance | Strict-lane policy classifier and candidate-source provenance gate at replay staging boundary | Typed hard error / explicit `HOLD` (`HS-SIMOUT-E-001`) when strict/parquet lane policy or candidate source class is missing/ambiguous; conversion-derived dat evidence remains non-promotable for final Tier-A closure | Tier-A replay tooling alignment gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-024` | runtime + governance | Parquet semantic alias and width-diagnostic validator at semantic report publication boundary | Typed hard error / explicit `HOLD` (`HS-SIMOUT-E-001`) when required alias continuity (`Total-Soil`/`Total-Soil Water`) is unresolved or width diagnostics use placeholder sentinel classes | Tier-A replay tooling alignment gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-025` | governance | Contract-derived replay closure-test validator at parity evidence gate | Typed hard error / explicit `HOLD` (`HS-SIMOUT-E-001`) when required SIMIMPL13 blind-spot closure tests are missing/failing, including conversion-derived dat row-consistency and strict-lane compensation coverage | Tier-A replay contract-test closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-026` | runtime | WB13 row assembler + hydout-equivalent publication mapper | Typed hard error on precipitation passthrough partition errors (`RM` sourced from raw `P` under snow-active cold branch) or when `Snow-Water` publication aliases static control `snow.options.ssd` rather than runtime SWE | Tier-A hydrology publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-027` | runtime + governance | Multi-day WB13 storage tuple continuity checker | Typed hard error / explicit `HOLD` when all published storage terms remain invariant across non-zero forcing and thermal transitions, indicating static publication leakage | Tier-A hydrology mutation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-028` | runtime + governance | Baseline WB11 sequencing validator for ET/perc/lateral/drain/root-uptake ordering | Typed hard error / explicit `HOLD` when execution order deviates from baseline-authoritative ordering in promoted WB11 closure claims | SIMIMPL ET/soil-water migration gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-029` | runtime + governance | Layer-to-aggregate water-lineage validator for WB13 publication surfaces | Typed hard error / explicit `HOLD` when aggregate/publication values are not traceable to declared `st(i)` -> `soilw(i)` -> `watcon` lineage | SIMIMPL hydrology publication-lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-030` | runtime | WB19 lateral/drainage hourly lane validator | Typed hard error / explicit `HOLD` when hourly lane claims do not execute WB19 iterative substeps and accumulated daily flux publication semantics | HPHYS hourly migration gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-031` | runtime + governance | Hydrology-tail order validator plus WB13 flux-authority anti-shadow checks for `Q`/`Ep`/`Es`/`Er` | Typed hard error / explicit `HOLD` when canonical WB19->WB12 ordering is broken or stale state duplicates shadow same-name flux symbols at WB13 publication | HPHYS hourly handoff closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-032` | runtime + governance | WB12/WB14 runoff-carryover resolver and publication validator | Typed hard error / explicit `HOLD` when same-pass `wb12_runoff_carryover` is ignored, malformed, or replaced by stale `wb12_runon_input` | HPHYS hourly carryover closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-033` | runtime + governance | MOFE hourly carry-array validator, WB19/WB12 array producer/consumer path, and manifest/publication evidence gate | Typed hard error / explicit `HOLD` when hourly MOFE lanes use aggregate carry substitution, omit any 24-slot carry array, publish malformed array entries, or fail copy-forward provenance | HPHYS MOFE hourly carry-array closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-034` | runtime + governance | HPHYS0242 scheduler order, WB19 surface-saturation array producer, WB14 runoff assembler, and WB12 storage consumer | Typed hard error / explicit `HOLD` when hourly lane uses stale runoff/storage surfaces, omits positive `ui_SCrunf(ii)` addback, or violates same-pass `Q`/`ET`/`D`/`Qd` storage lineage | HPHYS cadence/order closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-035` | runtime + governance | H39 hourly closure gate spanning snow activation and WB19 lateral capacity lineage | Typed hard error / explicit `HOLD` when winter triggers are bypassed by sidecar-presence-only logic or WB19 emits lateral flux from non-`meblfc` active layers | HPHYS0247 H39 closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-036` | runtime + governance | H39 hourly `Dp`/`Pe` restrictive-bottom gate spanning WB18 percolation, WB12 storage, and WB13 publication evidence | Typed hard error / explicit `HOLD` when H39 hourly `D`/`Pe` lineage bypasses baseline `ui_bdrkth`/`kslast` bottom-layer effective conductivity | HPHYS0248 H39 `Dp`/`Pe` closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-037` | runtime + governance | WB17-to-WB13 aggregate storage validator spanning `Ep`/`Es`, layer storage, `watcon`, `Total-Soil`, and `SoilWaterTotal` | Typed hard error / explicit `HOLD` when WB17 ET bypasses layer mutation, aggregate storage is not recomputed after ET, or WB13 storage reflects pre-ET/stale scalar state | HPHYS0249 WB17/storage closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-038` | runtime + governance | WB13 final-`Ep` validator spanning scheduler PL activation, post-WB19 `PlantRootUptake`, and flux-authoritative daily publication | Typed hard error / explicit `HOLD` when WB13 `Ep` is stale, pre-root-uptake, state-shadowed, or produced under suppressed growth/root-depth execution | HPHYS0250 `Ep` lineage closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-039` | runtime + governance | WB17/WB13 `swu.for` uptake-magnitude validator spanning effective `pltol`, layer uptake traces, final `Ep`, and post-uptake aggregate storage | Typed hard error / explicit `HOLD` when crop `pltol` is masked, layer `UPi`/`Ui` traces are absent, or WB13 storage/`Ep` publication consumes pre-uptake state | HPHYS0251 uptake/storage closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-040` | runtime + governance | WB19-to-WB17 storage-availability validator spanning frozen-adjusted lateral storage, post-WB19 layer `st(i)`, `watcon`, `Total-Soil`, and root uptake availability | Typed hard error / explicit `HOLD` when WB19 lateral capacity/withdrawal omits `SC-SUBHYD-001#INV-SUBHYD-025` `fzdrfc(i)` lineage or WB17/WB13 consume pre-WB19/stale aggregate storage | HPHYS0252 WB19 storage-availability closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-041` | runtime + governance | WB11 initial-storage projection validator spanning normalized primary layer grid, seeded layer storage, aggregate `watcon`, and WB13 storage publication | Typed hard error / explicit `HOLD` when WB11 seed layers truncate normalized profile depth, mix parser-depth `dg` with normalized profile aggregates, or seed aggregate storage from non-layer-authoritative compensation | HPHYS0254 WB11 initial-storage projection closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-042` | runtime + governance | MOFE storage-lineage validator spanning WB11 seed aliases, scoped OFE soil diagnostics, WB13/H.wat storage publication, and MOFE04 provenance | Typed hard error / explicit `HOLD` when storage publication is silently reinterpreted as per-OFE static aggregation or lacks declared runtime-lineage policy under multi-OFE provenance | HPHYS0255 MOFE storage projection closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-043` | runtime + governance | WB13 `latqcc` publication validator spanning WB19 daily/hourly lane provenance and subsurface lateral flux authority | Typed hard error / explicit `HOLD` when daily `latqcc` is produced with hourly lateral selection, lane provenance is missing, or `latqcc` residual evidence ignores `SC-SUBHYD-001#INV-SUBHYD-026` | HPHYS0256 `latqcc` lane-branch closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-044` | runtime + governance | WB13 `latqcc`/`Qd` realized-publication validator spanning WB19 potential/target/`tdvv` diagnostics and post-withdrawal layer storage | Typed hard error / explicit `HOLD` when WB13 consumes potential/target instead of realized `q`, stale `Qd`, or storage not reconciled to per-layer WB19 withdrawal | HPHYS0258 hourly cap/withdrawal publication closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-045` | runtime + governance | H1/H7/H39 residual classifier spanning WB19 trace evidence, WB13 `latqcc`/`Qd`, WB17 `Ep`, WB18 `Dp`, and aggregate storage | Typed hard error / explicit `HOLD` when continuation assigns residual ownership without trace-grade WB19 identity checks or ignores downstream Ep/Dp/storage dominance after WB19 identities close | HPHYS0259 trace localization gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-046` | runtime + governance | H1/H7/H39 residual classifier spanning WB17 layer uptake, WB18 percolation/storage, aggregate `watcon`, and WB13 storage publication | Typed hard error / explicit `HOLD` when trace evidence omits required WB17/WB18/storage maps, when identities do not reconcile, or when residual ownership is assigned to publication/shadowing after identities close | HPHYS0260 WB17/WB18/storage residual-classification gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-047` | runtime + governance | H1/H7/H39 WB17 `Ep` magnitude/initialization classifier spanning candidate/baseline WAT `Ep`, trace `Etp`, final `Ep`, `ΣUi`, plant state, `pltol`, `ul(i)`, and stress thresholds | Typed hard error / explicit `HOLD` when residual ownership or WB13 compensation is assigned without trace-grade `SC-EVAP-001#INV-EVAP-019` evidence and legacy call-order provenance | HPHYS0261 `Ep` magnitude/initialization gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-048` | runtime + governance | H1/H7/H39 WB17 PMET demand-seeding classifier spanning candidate/baseline WAT `Ep`, trace `wb11_et_demand`, PMET mode/crop coefficients, actual seed branch, final `Ep`, and `ΣUi` | Typed hard error / explicit `HOLD` when PMET lineage is hidden or when closure uses Priestley-Taylor/proxy demand while `pmetpara` selects `evappm` | HPHYS0262 PMET demand-seeding gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-049` | runtime + governance | H1/H7/H39 EVAPPM migration classifier spanning candidate/baseline WAT `Ep`, trace `wb11_et_demand`, actual `evappm_pmet` seed branch, PMET intermediate evidence, final `Ep`, and `ΣUi` | Typed hard error / explicit `HOLD` when PMET mode lacks migrated `evappm` demand evidence or when remaining residual ownership is asserted without PMET branch proof | HPHYS0263 EVAPPM migration gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-050` | runtime + governance | H1/H7/H39 PMET seam classifier spanning candidate/baseline WAT `Ep`/`Es`, trace `pmet.es_m`, `pmet.ep_m`, `Etp`, `Ui`, final `Ep`, non-negative `Es` with bounded roundoff canonicalization, non-negative `Er`, and branch proof | Typed hard error / explicit `HOLD` when PMET mode double partitions `ep`, omits `pmet.es_m`, accepts material negative `Es`, bypasses SWU final `Ep`, or assigns residual ownership without seam proof | HPHYS0264 PMET seam correction gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-051` | governance | H1/H7/H39 first-large longer-season `Ep` divergence classifier spanning WAT row context and multi-day trace WB17/SWU/storage/snow/runoff/percolation/lateral terms | Explicit `HOLD` when seasonal `Ep` residual ownership is assigned without first-divergence context, or when WB17 identities close but storage/snow/runoff/lateral context is omitted | HPHYS0265 first-divergence localization gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-052` | governance | H1/H7/H39 layer/lateral/snow first-divergence classifier spanning WB11/WB18 aggregate closure, WB17 stress layers, WB19 realized lateral identities, and same-day WAT context | Explicit `HOLD` when WB17/WB19 identities close but layer distribution, snow/runoff, and lateral magnitude context remains unresolved; production edits require baseline-authoritative defect proof | HPHYS0266 layer/lateral/snow coupling gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-053` | governance | H1/H7/H39 post-lateral/pre-SWU threshold-lineage classifier spanning WB19 `drfc`/`fzdrfc`, pre/post-lateral layer storage, lateral withdrawal, and WB17 stress thresholds | Explicit `HOLD` when threshold lineage evidence does not prove a baseline-authoritative production defect; no WB17/WB19/WB13 compensation edits | HPHYS0267 threshold-lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-054` | governance | H1/H7/H39 spring snowpack classifier spanning WB13 `RM`/`Snow-Water`, runtime snow carry state, hourly rain/snow/melt totals, and signed `S` | Explicit `HOLD` when material spring `Ep` divergence evidence omits snowpack/SWE/`RM` lineage; no WB17 `Ep`, aggregate-storage, or WB13 publication compensation edits | HPHYS0268 spring snowpack gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-055` | runtime + governance | H1/H7/H39 winter melt/snowpack classifier spanning retained rain, raw signed melt, redistributed melt, signed `S`, WB13 `RM`, and WB13 `Snow-Water` | Typed hard error / explicit `HOLD` when retained-rain or signed-melt lineage is missing, non-finite, or substituted by precipitation passthrough; no WB17 `Ep` or aggregate-storage compensation edits | HPHYS0269 winter melt/snowpack baselining gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-056` | governance | H1/H7/H39 daily snowpack carry-state classifier spanning pre-day/post-day SWE, depth, density, settle count, deltas, WB13 `RM`/`Snow-Water`, and WB17 `Ep` context | Explicit `HOLD` when residual ownership is asserted without daily carry-state evidence; no WB17 `Ep`, aggregate-storage, or WB13 publication compensation edits | HPHYS0270 daily snowpack state gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-057` | governance | H1 day-36 melt-forcing classifier spanning WB13 `RM`/`Snow-Water`, raw/redistributed melt, `melt.for` terms, hourly forcing, radiation units, and downstream `Ep`/storage context | Explicit `HOLD` when residual ownership is asserted without day-36 melt-term/hourly-forcing and radiation-unit evidence; no WB17 `Ep`, aggregate-storage, WB13 publication, negative-melt compensation, or heuristic radiation clipping edits | HPHYS0271/HPHYS0272 day-36 melt-forcing gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-058` | runtime + governance | Spring meltwater partition classifier spanning redistributed melt, WB12 infiltration/runoff partition, WB18 same-pass layer ingress, signed `S`, WB13 `RM`, and `Total-Soil` | Typed hard error / explicit `HOLD` when routed snowmelt bypasses infiltration/layer ingress or spring storage-collapse ownership is asserted without partition evidence | HPHYS0283 spring partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-059` | runtime + governance | H1/H7/H39 spring snow-retention classifier spanning positive/negative hourly melt totals, routed `S`/`RM`, runtime SWE/depth state loss, WB13 `Snow-Water`, and spring storage context | Typed hard error / explicit `HOLD` when snowpack storage is closed from routed net melt alone under mixed positive/negative melt days or when spring snow-retention ownership is asserted without corrected state-lineage evidence | HPHYS0284 spring snow-retention gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-060` | runtime + governance | H1/H7/H39 spring soil-retention classifier spanning positive local WB12/WB14 infiltration, WB18 same-pass layer ingress, aggregate `watcon`, `Total-Soil`, and `SoilWaterTotal` | Typed hard error / explicit `HOLD` when same-pass infiltration remains active-snow-gated or publication-only before storage publication | HPHYS0285 spring soil-retention gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-061` | runtime + governance | Post-WB17/pre-WB19 layer-cap redistribution and aggregate storage lineage validator | Typed hard error / explicit `HOLD` when lower-layer excess above the baseline active cap remains, is discarded, or is publication-compensated instead of moving upward before WB19/WB13 consumers | HPHYS0286 post-ingress retention/WB18-WB17 coupling gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-062` | runtime + governance | WB12/WB14 snow-liquid partition and WB13 publication classifier spanning runtime snow state, `RM`, `Snow-Water`, infiltration, and aggregate storage | Typed hard error / explicit `HOLD` when domain-invalid snow state is silently zeroed or used as publication compensation before WB13 closure | HPHYS0287 snow liquid partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-063` | runtime + governance | Rain-on-snow direct-rain vs routed-melt closure spanning WB12 runoff, WB18 storage ingress, and WB13 `RM` | Typed hard error / explicit `HOLD` when residual rain-on-snow is omitted from `wmelt`, left exclusively as direct rain, or double counted | HPHYS0288 rain-on-snow partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-064` | runtime + governance | WB13 `RM`/`Snow-Water` publication mapper consuming post-winter rain, routed `wmelt`, irrigation, and runtime snowpack storage | Typed hard error / explicit `HOLD` when WB13 `RM` uses raw-precipitation/SWE-delta proxy math or missing routed-melt authority | HPHYS0289 WB13 publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-065` | runtime + governance | WB13 `RM` publication mapper consuming explicit `snow.post_winter_rain_m`, explicit `snow.routed_melt_m`, and irrigation | Typed hard error / explicit `HOLD` when post-winter rain is inferred from raw precipitation/SWE/snow-active state or when the explicit surface is missing, negative, non-finite, or shadowed by stale state | HPHYS0290 WB13 post-winter-rain publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-066` | runtime + governance | Same-day runoff-producer to WB13 snow publication lifecycle for `snow.post_winter_rain_m` and `snow.routed_melt_m` | Typed hard error / explicit `HOLD` when WB13 is satisfied by absent/stale/default state instead of producer fluxes or when residual ownership is asserted without lifecycle evidence | HPHYS0291 snow publication lifecycle gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-067` | governance | H1/H7/H39 spring melt/capacity/storage classifier spanning winter producer terms, WB12 capacity, `Q`, WB18 ingress, and WB13 storage outputs | Explicit `HOLD` when spring storage-collapse ownership is asserted without distinguishing routed-melt timing from infiltration-capacity limitation | HPHYS0292 spring snowmelt/infiltration capacity gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-068` | governance | Snow producer versus post-ingress storage classifier after HPHYS0292 `Q` closure | Explicit `HOLD` when WB18/WB19/WB17 ownership is asserted before snow producer `Snow-Water`/`RM` timing is corrected or excluded by trace evidence | HPHYS0293 winter melt magnitude/timing gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-069` | governance | Post-ingress storage/percolation/lateral classifier with snow-excluded residual masks | Explicit `HOLD` when storage/lateral/percolation ownership is asserted from comparator deltas alone or without WB18/WB19 trace-grade magnitude accounting | HPHYS0294 post-ingress storage/percolation/lateral gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-070` | governance | Cumulative H1/H7/H39 row-to-row storage-budget classifier across ET, percolation, lateral, `Q`, `RM`, `Snow-Water`, WB18/WB19 identities, and excluded snow masks | Explicit `HOLD` when production ownership is asserted before cumulative budget evidence proves a process owner that survives timing and excluded residual separation | HPHYS0295 cumulative storage-budget ownership gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-071` | governance | HPHYS0296/HPHYS0303 snow/`RM` semantic acceptance classifier joining cumulative budget dominance, fixed-comparator negative-melt evidence, candidate `RM` publication identity, snow-state closure, reconstruction evidence, and independent correctness adjudication | Explicit `HOLD` unless a per-window defective-model verdict is proven and auditable; correlation plus internal closure is insufficient; downstream compensation remains invalid | HPHYS0296 snow/`RM` acceptance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-072` | governance | HPHYS0297 water-balance ledger for observed/reconstructed `RM`, closed `Q`, producer-consumer identity, and final verdict | Explicit `HOLD` when reconstruction fails tolerance or verdict is absent; residuals remain in semantic accounting | HPHYS0297 snow/`RM` defect-ledger gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-073` | governance | HPHYS0298 partition ledger for baseline observe identity, same-HEAD full-suite metrics, ordered first-divergent cut-point, canonical values/source lines, closed downstream identities, and final verdict | Explicit `HOLD` when observe identity fails, first-divergence evidence is absent, verdict is absent, or downstream compensation is asserted | HPHYS0298 paired snow/`RM` lineage partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-074` | governance | HPHYS0299 corrected unit/provenance ledger separating canonical `hrsnow` snowfall depth from derived snowfall water-equivalent summaries before continuation routing | Explicit `HOLD` when `hrsnow` is mapped to water-equivalent snowfall, corrected depth-vs-depth evidence is absent, or HPHYS0298 production-migration authority is reused without corrected evidence | HPHYS0299 hourly snow partition unit/provenance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-075` | governance | HPHYS0300 raw/post-raw water-balance ledger spanning corrected forcing status, raw `hrmlt`, post-raw `wmelt`/routed melt, WB13 `RM`, `Q`, `Snow-Water`, full-suite metrics, and term/state evidence requirements | Explicit `HOLD` when downstream WB17/WB18/WB19/WB13 ownership is asserted before raw/post-raw producer closure or when H7/H39 special cases are collapsed into generic melt acceptance | HPHYS0300 raw hourly melt/post-raw routing gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-076` | governance | HPHYS0301 H39 residual-rain/release water-balance ledger before forcing, raw-melt, routed-melt, or downstream edits | Explicit `HOLD` when raw rain aggregates are treated as forcing authority, residual rain is compared to raw rain, or downstream compensation is asserted before rain-release/post-raw producer closure | HPHYS0301 H39 rain-release water-balance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-077` | governance | HPHYS0302 comparator-surface audit across daily output surfaces, aggregate raw/post-raw snowmelt surfaces, and missing melt term/state surfaces | Explicit `HOLD` when aggregate/output deltas are treated as producer authority or downstream compensation is asserted before paired term/state evidence | HPHYS0302 water-balance comparator-surface gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-078` | governance | HPHYS0305 paired fixed-baseline/openWEPP melt-term, forcing, and snow-state instrumentation ledger for all nine H1/H7/H39 target windows | Explicit `HOLD` when paired surfaces are missing, units are not same-quantity, a first divergent source is absent, or downstream compensation is asserted from aggregate residuals | HPHYS0305 paired melt-term/state gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-079` | governance | HPHYS0306 branch-active fixed-baseline/openWEPP melt-term observe ledger using baseline melt-call keys and openWEPP `snow_hourly_melt_branch_active` keys before term magnitude comparison | Explicit `HOLD` when branch-active masks differ, inactive baseline hours are zero-imputed, paired active-domain surfaces are incomplete, or downstream compensation is asserted | HPHYS0306 branch-active observe semantics gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-080` | governance | HPHYS0307 branch-activation control-flow ledger using baseline `winter.for`/`snowd.for` and openWEPP `compute_simimpl29_melt_hour` branch predicates | Explicit `HOLD` when baseline-extra/openWEPP-extra active masks lack source-line-owned defect proof, matched masks still diverge in term/state lanes, trace parsing conflicts, or downstream compensation is asserted | HPHYS0307 melt-call branch activation lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-081` | governance | HPHYS0308 branch-extra key-level state-ordering ledger spanning baseline melt-call predicates, baseline snow state/terms, and openWEPP before/available/after snow depth-density surfaces | Explicit `HOLD` when branch-extra keys are snow-state carry/depletion holds, baseline branch instrumentation is incomplete, matched masks still diverge in same-hour term/state lanes, or downstream compensation is asserted | HPHYS0308 snowd branch predicate state-ordering gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-082` | governance | HPHYS0309 snow carry/depletion water-balance ledger comparing fixed-comparator carry state against openWEPP runtime/hourly snow depletion timing | Explicit `HOLD` when rows are pre-day carry deficits, prior-day openWEPP meltout, incomplete state evidence, or downstream compensation is asserted before carry-state closure | HPHYS0309 snow carry/depletion lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-083` | governance | HPHYS0310 prior-day snow carry divergence ledger covering affected H1/H7/H39 groups and first material paired carry-state divergence before key days | Explicit `HOLD` when rows are initial carry-state projection, accumulation/settling onset, corrected negative-melt state loss, retained-liquid handling, raw/routed melt magnitude, incomplete evidence, or downstream compensation is asserted before source-line proof | HPHYS0310 prior-day carry divergence gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-084` | governance | HPHYS0311 source-line carry-state parity ledger distinguishing inherited prior-year terminal deltas, year-boundary projection defects, settling/depth equation defects, and fixed-observe precision holds before water-balance ownership changes | Explicit `HOLD` when deltas are inherited, source-line proof is absent, fixed-observe precision is insufficient, or downstream compensation is asserted before openWEPP-owned carry-state proof | HPHYS0311 snow carry source-line parity water-balance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-085` | governance | HPHYS0312 prior-year terminal snowpack lineage ledger identifying first material inherited terminal-state divergence within each scanned prior calendar year | Explicit `HOLD` when divergence is year-start inherited, settling/depth update without full-precision reconstruction, source-line proof is absent, or downstream compensation is asserted | HPHYS0312 prior-year terminal snowpack water-balance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WATBAL-086` | governance | HPHYS0313 split-route snowpack settling/carry recursion ledger covering full-precision settling reconstruction, branch-gated hourly snowfall/drift lineage, and 2014 terminal carry-state recursion | Explicit `HOLD` when settling reconstruction lacks source-owned proof, branch-gated snowfall/drift lineage is unresolved, earlier carry-state inheritance remains unresolved, rounded observe output is treated as sufficient, or downstream compensation is asserted | HPHYS0313 split-route snowpack water-balance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-5 WEPP notation and legacy
lineage names by default. EROD11 ratifies Wave-0 erosion-lane alias ownership
for required runoff and peak-duration coupling surfaces while remaining
water-balance symbols retain existing canonical or explicitly typed mappings.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `Θ` | `Θ` (identity) | root-zone daily state surface | `m` -> `m` | `[DIRECT][Static]` |
| `Θin` | `Θin` (identity) | root-zone daily initial-state surface | `m` -> `m` | `[DIRECT][Static]` |
| `P`, `I`, `S`, `Q`, `ET`, `D`, `Qd` | identity names; WAT aliases `hillslope_wat.P`, `hillslope_wat.P:mm`, `hillslope_wat.Q`, `hillslope_wat.Q:mm` | daily closure terms and WB13/WAT publication terms | closure `m` preserved; WAT `P`/`Q` publication uses `mm` | `[DIRECT][Static]` |
| `Q` (typed runoff flux alias) | `HillslopeProductionFluxSymbol::Wb12RunoffQ -> Q`; WAT aliases `hillslope_wat.Q`, `hillslope_wat.Q:mm` | runoff-depth coupling surface exported to runoff/erosion consumers | closure `m` preserved; WAT publication uses `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `wb20_forward_solver_lane_enabled` | `wb20_forward_solver_lane_enabled` | WB20 parity-lane selector (`1` forward-solver lane, `0` compatibility lane) for WB12 closure-delta semantics | scalar in `{0,1}` preserved | `[INFERENCE][Static]` |
| `Eu` | `wb11_et_demand` | WB17 ET demand input consumed by partition runtime | `m d^-1` -> `m d^-1` | `[DIRECT][Static]` |
| `L` | `lai` | WB17 LAI partition driver | `m^2 m^-2` -> `m^2 m^-2` | `[DIRECT][Static]` |
| `Er` | `wb17_residue_interception` (input) + `Er` (flux output) | WB17 residue evaporation partition | `m d^-1` input -> `m` daily flux output | `[DIRECT][Static] + [INFERENCE][Static]` |
| `VE` | `VE` (identity) | interception input surface | `kg m^-2` -> `kg m^-2` | `[DIRECT][Static]` |
| `Es`, `Esb`, `Esp`, `Etp` | identity names | ET partition surfaces | `m d^-1` -> `m d^-1` | `[DIRECT][Static]` |
| `Ep` | `Ep` (flux output) | WB17 plant-transpiration component surface | `m` daily flux output | `[DIRECT][Static] + [INFERENCE][Static]` |
| `UPi`, `Ui` | identity names | layer-wise uptake surfaces | `m d^-1` -> `m d^-1` | `[DIRECT][Static]` |
| `dx`, `ds` | identity names | evaporation-depth surfaces | `m` -> `m` | `[DIRECT][Static]` |
| `Θr`, `Θi`, `FCi`, `ULi` | identity names | layer-state and thresholds | chapter-declared units preserved | `[DIRECT][Static]` |
| `Θc` | `Θc` (identity) | layer critical-water threshold surface | `m^3 m^-3` -> `m^3 m^-3` | `[DIRECT][Static]` |
| `Θi` | `wb18_perc_theta_####` | WB18 per-layer moisture routing surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `FCi` | `wb18_perc_fc_####` | WB18 per-layer field-capacity threshold surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ULi` | `wb18_perc_ul_####` | WB18 per-layer upper-limit storage surfaces | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `st(i)` | `wb18_perc_theta_####` | WB11/WB18 canonical layer-storage alias family used by ET and hydrology mutations | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `watcon` | `wb11_soil_water` | WB11 aggregate unfrozen root-zone storage state | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `watcon` publication lineage | `Total-Soil`, `SoilWaterTotal` | WB13/hydout-equivalent aggregate publication alias family | `mm` publication units preserve declared depth-conversion semantics | `[DIRECT][Static] + [INFERENCE][Static]` |
| `RM` | `hillslope_wat.RM`, `hillslope_wat.RM:mm` | WB13/WAT rainfall + irrigation + snowmelt publication term | `mm` publication units preserve declared depth-conversion semantics | `[DIRECT][Static] + [INFERENCE][Static]` |
| `UpStrmQ`, `QOFE`, `Irr` | `hillslope_wat.UpStrmQ`, `hillslope_wat.UpStrmQ:mm`, `hillslope_wat.QOFE`, `hillslope_wat.QOFE:mm`, `hillslope_wat.Irr`, `hillslope_wat.Irr:mm` | WB13/WAT runoff/irrigation publication terms | `mm` publication units preserve declared depth-conversion semantics | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Total-Soil`, `SoilWaterTotal`, `frozwt` | `hillslope_wat.Total-Soil`, `hillslope_wat.Total-Soil:mm`, `hillslope_wat.SoilWaterTotal`, `hillslope_wat.SoilWaterTotal:mm`, `hillslope_wat.frozwt`, `hillslope_wat.frozwt:mm` | WB13/WAT aggregate storage publication terms | `mm` publication units preserve declared depth-conversion semantics | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Area` | `hillslope_wat.Area`, `hillslope_wat.Area:m^2` | WB13/WAT contributing-area publication term | `m^2` publication units preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`, `wb13_profile_fc_tail_mm` | `hillslope_wat.ProfilePorosityCap`, `wb13_profile_porosity_cap_mm`, `hillslope_wat.ProfilePorosityCap:mm`, `hillslope_wat.ProfileFCStore`, `wb13_profile_fc_store_mm`, `hillslope_wat.ProfileFCStore:mm`, `hillslope_wat.ProfileWPStore`, `wb13_profile_wp_store_mm`, `hillslope_wat.ProfileWPStore:mm` | WB13 profile-storage publication and diagnostic surfaces | `mm` publication units preserve declared depth-conversion semantics | `[DIRECT][Static] + [INFERENCE][Static]` |
| `InterceptionStorage` | `hillslope_wat.InterceptionStorage`, `hillslope_wat.InterceptionStorage:mm` | WB13/WAT interception-storage publication term | `mm` publication units preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_SUrunf(ii)` | `ui_SUrunf_{hour:04}` state symbol | MOFE hourly upstream saturation-runoff carry consumed at WB12/WB14 runoff reconciliation | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_SCrunf(ii)` | `ui_SCrunf_{hour:04}` state symbol | MOFE hourly current-OFE saturation-runoff carry published for copy-forward | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `surdra = Σui_SCrunf(ii)` | `Σ ui_SCrunf_{hour:04}` plus `Q` addback | MOFE hourly current-OFE surface-saturation excess clipped from top-layer storage and added to runoff closure | `m` preserved; 24-hour sum contributes to daily `Q`; WAT `Q` publication remains `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_LfUrf(ii)` | `ui_LfUrf_{hour:04}` state symbol | MOFE hourly upstream lateral-flow carry consumed at WB12/WB14 runoff reconciliation | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_LfCrf(ii)` | `ui_LfCrf_{hour:04}` state symbol | MOFE hourly current-OFE lateral-flow carry published from WB19 substeps for copy-forward | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ksi` | `wb18_perc_ssc_####` | WB18 per-layer conductivity surfaces | `m s^-1` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_ssh(i)` | `wb19_lateral_ssh_####` | WB19 hourly horizontal saturated conductivity after layer `ui_anisrt(i)` projection | `m s^-1` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `subq`/`latqcc`/`tdvv` diagnostics | `wb19_q_lateral_potential`, `wb19_q_lateral_target`, `wb19_lateral_capacity_tdv`, `wb19_tdvv`, `wb19_q_lateral_unrealized`, `wb19_lateral_withdrawal_####` | WB19 potential/target/cap/realized-withdrawal lineage used to validate WB13 `latqcc`/`Qd` publication | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0259 trace WB19 diagnostics | `wb19_q_lateral_potential_m`, `wb19_q_lateral_target_m`, `wb19_lateral_capacity_tdv_m`, `wb19_tdvv_m`, `wb19_q_lateral_unrealized_m`, `wb19_lateral_withdrawal_layers_m`, `q_m`, `qdd_m`, `qd_m` | Opt-in run-trace evidence for classifying `latqcc` residual ownership before shifting focus to `Ep`/`Dp`/storage | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0260 trace WB17/WB18/storage diagnostics | `wb17_upi_layers_m`, `wb17_ui_layers_m`, `wb18_thetdr_layers`, `wb18_dg_layers_m`, `wb18_frozen_depth_layers_m`, `wb18_recomputed_soil_water_m`, `wb18_recomputed_minus_wb11_m`, `upi_m`, `ui_m`, `ep_m`, `etp_m`, `ws`, `d_m`, `pe_m`, `wb13_total_soil_mm`, `wb13_soil_water_total_mm` | Opt-in run-trace evidence for classifying `Ep`, `Dp`, `Total-Soil`, and `SoilWaterTotal` residual ownership after WB19 identities close | `m`, `mm`, and dimensionless stress units preserved as named | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0261 trace WB17 magnitude/initialization diagnostics | `pl_pltol`, `pl_swu_effective_pltol`, `wb18_ul_layers_m`, `wb17_swu_stress_threshold_layers_m`, `wb17_swu_storage_to_threshold_layers`, `pl_lai`, `pl_rtd`, `etp_m`, `ep_m`, `ui_m`, `wb17_ui_layers_m`, `ws` | Opt-in run-trace evidence for classifying H1/H7/H39 `Ep` magnitude residuals at the `evap`/`swu` seam before changing equations or WB13 compensation | `m`, `mm`, and dimensionless plant/stress units preserved as named | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0262 trace PMET demand-seeding diagnostics | `pmet_sidecar_present`, `pmet_iflget`, `pmet_selected_kcb`, `pmet_selected_rawp`, `pmet_selected_line_index`, `pmet_lookup_fallback_first_row_used`, `wb11_et_demand_m`, `wb11_et_seed_branch` | Opt-in run-trace evidence for classifying whether WB13 `Ep` and storage residuals inherit PMET-demand lineage divergence rather than WB13 publication or SWU clipping | mode flags dimensionless; coefficients dimensionless; demand `m d^-1` | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0263 trace EVAPPM migration diagnostics | `pmet_etorc_mm`, `pmet_rn_mj_m2`, `pmet_fwv_m_s`, `pmet_rhd_pct`, `pmet_kcbadj`, `pmet_kcbcon`, `pmet_etke`, `pmet_etkr`, `pmet_etks`, `pmet_tew_mm`, `pmet_rew_mm`, `pmet_wfevp_mm`, `pmet_taw_mm`, `pmet_raw_mm`, `pmet_wftrp_mm`, `pmet_es_m`, `pmet_ep_m` | Opt-in run-trace evidence for assigning WB13 `Ep`, `Total-Soil`, and `SoilWaterTotal` residual ownership after PMET demand migration | metric intermediates and `m d^-1` final demand units preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0264 trace PMET seam diagnostics | `wb11_et_seed_branch`, `wb11_et_demand_m`, `pmet_es_m`, `pmet_ep_m`, `es_m`, `er_m`, `etp_m`, `ui_m`, `ep_m`, `ws` | Opt-in run-trace evidence that PMET mode consumes EVAPPM `es`/`ep` components at the WB17 seam, preserves `swu` as final `Ep`, and does not use PT repartition under `evappm_pmet` | `m d^-1` ET surfaces and dimensionless stress units preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0265 first-divergence diagnostics | candidate/baseline WAT `Ep`, `Total-Soil`, `SoilWaterTotal`, `Dp`, `latqcc`, `Q`, `RM`, `Snow-Water`; trace `pmet_ep_m`, `etp_m`, `ep_m`, `ui_m`, `wb17_ui_layers_m`, `ws`, `pl_lai`, `pl_rtd`, `pl_pltol`, `wb17_swu_storage_to_threshold_layers`, `wb13_total_soil_mm` | Required evidence to separate seasonal `Ep` residual ownership from aggregate storage, snow/runoff timing, percolation, and lateral-flow coupling after HPHYS0264 seam closure | `mm` WAT terms; `m d^-1` trace ET terms | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0266 layer/lateral/snow diagnostics | `wb11_soil_water_mm`, `wb18_theta_sum_m`, `wb18_recomputed_minus_wb11_m`, `wb17_swu_storage_to_threshold_layers`, `wb19_q_lateral_potential_m`, `wb19_q_lateral_target_m`, `wb19_q_lateral_unrealized_m`, `wb19_lateral_capacity_active_count_layers`, `wb19_lateral_conductivity_active_count_layers`, `wb19_lateral_withdrawal_layers_m`, `q_m`, `qdd_m`, `qd_m` | Required evidence to separate root-zone SWU stress from bottom-zone WB19 lateral activity and same-day snow/runoff/storage context after HPHYS0265 | `mm`/`m` WAT and trace terms; counts and ratios preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0267 threshold-lineage diagnostics | `wb18_fc_layers_m`, `wb19_coca_layers`, `wb19_drfc_layers_m`, `wb19_frzw_layers_m`, `wb19_fzdrfc_layers_m`, pre/post-lateral `wb18_theta_layers_m`, `wb19_lateral_withdrawal_layers_m`, `wb18_ul_layers_m`, `wb17_swu_stress_threshold_layers_m`, `wb17_swu_storage_to_threshold_layers` | Required evidence to classify post-lateral/pre-SWU layer-threshold lineage before assigning residual ownership or patching production physics | `m` layer storage/thresholds and dimensionless ratios preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0268/HPHYS0269/HPHYS0290 snowpack diagnostics | `snow_runtime_swe_m`, `snow_runtime_depth_m`, `snow_runtime_density_kg_m3`, `snow_runtime_settle_day_count`, `snow_s_m`, `snow_routed_melt_m`, `snow_post_winter_rain_m`, `snow_hourly_rain_sum_m`, `snow_hourly_rain_retained_sum_m`, `snow_hourly_snowfall_water_equiv_sum_m`, `snow_hourly_melt_raw_sum_m`, `snow_hourly_melt_sum_m`, `snow_runtime_swe_closure_error_m`, `wb13_p_mm`, `wb13_rm_mm`, `wb13_snow_water_mm` | Required evidence to classify whether first material H1/H7/H39 `Ep` divergence is inherited from snowpack/SWE/`RM` lineage and whether winter melt/rain-retention/post-winter-rain migration is authoritative before returning to WB17 `Ep` | runtime state `m`/`kg m^-3`/count, daily snow coupling `m`, WB13 publication `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0270 daily snowpack state diagnostics | `snow_runtime_swe_before_m`, `snow_runtime_depth_before_m`, `snow_runtime_density_before_kg_m3`, `snow_runtime_settle_day_count_before`, `snow_runtime_swe_m`, `snow_runtime_depth_m`, `snow_runtime_density_kg_m3`, `snow_runtime_settle_day_count`, `snow_runtime_swe_delta_m`, `snow_runtime_depth_delta_m`, `snow_runtime_density_delta_kg_m3`, `snow_runtime_settle_day_count_delta`, `wb13_rm_mm`, `wb13_snow_water_mm` | Required evidence to classify whether WB13 `RM`/`Snow-Water` and WB17 `Ep` residuals inherit day-begin snowpack carry-state divergence or same-day snowpack mutation before any production physics or publication compensation edit | runtime state `m`/`kg m^-3`/count, deltas in same units, WB13 publication `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0271/HPHYS0272/HPHYS0305 melt-forcing diagnostics | `snow_hourly_rain_m`, `snow_hourly_snowfall_depth_m`, `snow_hourly_depth_before_m`, `snow_hourly_depth_available_m`, `snow_hourly_depth_after_m`, `snow_hourly_density_before_kg_m3`, `snow_hourly_density_after_kg_m3`, `snow_hourly_melt_raw_m`, `snow_hourly_melt_m`, `snow_hourly_melt_amelt_in`, `snow_hourly_melt_bmelt_in`, `snow_hourly_melt_cmelt_in`, `snow_hourly_melt_dmelt_in`, `snow_hourly_melt_hrtef_f`, `snow_hourly_melt_hrdtf_f`, `snow_hourly_melt_vwmph`, `snow_hourly_melt_rainin`, `snow_hourly_melt_wind_adjustment`, `snow_hourly_melt_branch_active`, `winter_hourly_air_temp_c`, `winter_hourly_dewpoint_c`, `winter_hourly_wind_m_s`, `winter_hourly_rad_mj_m2`, `winter_hourly_cloud_fraction`, `wb13_rm_mm`, `wb13_snow_water_mm` | Required trace evidence to classify whether H1/H7/H39 `RM`/`Snow-Water` divergence is caused by melt energy-balance term magnitude, hourly-forcing inputs, radiation unit conversion, or snowpack depth/density state before any publication, ET, storage, or redistribution compensation edit | hourly maps and WB13 publication `mm`; radiation `MJ m^-2 h^-1`; melt depths and rain/snow depths `m`; density `kg m^-3`; melt terms in inch-equivalent pre-`0.0254` units | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ksbot` | `kslast` | restrictive-layer conductivity consumed by WB18 bottom-layer seepage when `slflag=1` | `m s^-1` preserved | `[DIRECT][Static]` |
| `Bbot` | `ui_bdrkth` | restrictive-layer thickness consumed by hourly WB18 bottom-layer seepage when `slflag=1` | `m` preserved | `[DIRECT][Static]` |
| `dg_i` | `dg_####` | WB19 per-layer thickness surfaces used by lateral/drainage withdrawal and conductivity weighting | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `por_i` | `por_####` | WB19 per-layer porosity surfaces consumed by water-yield coupling (`watyld`) | dimensionless preserved (`0 < por <= 1`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `coca_i` | `coca_####` | WB19 entrapped-air correction surfaces consumed by WB19 drain-threshold lineage | dimensionless preserved (`0 < coca <= 1`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `thetfc_i` | `thetfc_####` | WB19 per-layer FC theta surfaces consumed by `avfca` water-yield coupling | dimensionless preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `thetdr_i` | `thetdr_####` | WB19 per-layer WP theta surfaces used for FC/WP consistency checks against `wb18_perc_fc_####` | dimensionless preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `drfc_i` | `wb18_perc_fc_#### + (1-coca_####)*dg_####` | WB19 layer drain-threshold authority (legacy `drfc`) for saturation checks and withdrawals | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `watyld` | `wb19_watyld` | WB19 water-yield coupling state used in `solwpv < 2006` saturated-depth updates | dimensionless preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `fcdep`, `unsdep` | `wb19_fcdep`, `wb19_unsdep` | WB19 saturated/unsaturated depth coupling states after lateral mutation | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `pei` | `wb18_perc_pei_####` | WB18 per-layer percolation flux outputs | `m` per step preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ti`, `Δt`, `Ksai`, `Bi` | WB18 derived runtime intermediates | per-layer percolation routing diagnostics/intermediate terms | chapter-declared units preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `anisrt`, `ddrain`, `sdrain`, `drdiam` | `wb19_lateral_anisotropy_ratio`, `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter` | WB19 lateral/drainage geometry + anisotropy control surfaces | declared units preserved (`-`, `m`, `m`, `m`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Qdd`, `Qd` (WB19) | `Qdd`, `Qd` | WB19 drainage and aggregate subsurface-loss outputs consumed by closure | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ws` | `Ws` (identity) | plant-stress coupling surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `peakro`, `watdur` | `HillslopeProductionStateSymbol::{Wb16Peakro,Wb16Watdur}` | WB16 peak-duration diagnostics used by erosion/routing consumers | `m^3 s^-1`, `s` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionStateSymbol::{Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | WB16 branch-traceability surfaces for downstream contract diagnostics | branch metadata + scalar continuity diagnostics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |

## EROD11 Alias Ownership Register

| Boundary ID | Canonical symbols | Runtime alias surface | Producer ownership | Consumer ownership | Evidence |
|---|---|---|---|---|---|
| `EROD-BND-001` | `Q`, `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionFluxSymbol::Wb12RunoffQ`; `HillslopeProductionStateSymbol::{Wb16Peakro,Wb16Watdur,Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | `SC-WATBAL-001` via WB12/WB16 kernel outputs | `SC-RUNOFFPART-001`, `SC-SED-001`, `SC-ROUTE-001` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-005` | `D`, `Qd`, `ET`, `I` (daily closure companions) | `HillslopeProductionFluxSymbol::{Wb11PercLossD,Wb11SubhydQd,Wb11Et,Wb15InterceptionI}` | `SC-WATBAL-001` | downstream hydrology consumers and closure diagnostics | `[DIRECT][Static] + [INFERENCE][Static]` |

## EROD12 Cross-Domain Ownership and Guard Closure Addendum

| Cross-domain lane | Producer ownership | Consumer guard ownership | Closure posture | Evidence |
|---|---|---|---|---|
| WB12/WB16 runoff + peak-duration export (`Q`, `peakro`, `watdur`, `wb16_*`) | `SC-WATBAL-001` (`INV-WATBAL-007`, `INV-WATBAL-016`) | `SC-RUNOFFPART-001`, `SC-SED-001`, `SC-ROUTE-001`, `SC-HYDRAULICS-001` | Required Wave-0 erosion-lane hydrology boundary ownership and guard semantics are explicit. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Daily closure companion export (`D`, `Qd`, `ET`, `I`) | `SC-WATBAL-001` | downstream hydrology/system consumers (`SC-SUBHYD-001`, `SC-SYSTEM-001`) | Cross-domain closure companion ownership and guard mapping remain explicit for consumed surfaces. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| Dry day with depletion | `P = 0` and no snowmelt contribution, while `ET` and/or `D` reduce `Θ`. | Valid no-rain daily balance mode. |
| Snow-accumulation day | `S < 0` (accumulation), with rainfall partition and closure still maintained. | Chapter-5 explicitly allows signed snow-storage contribution. |
| Snowmelt-driven input day | `S > 0` contributes melt water as rainfall-equivalent for runoff/percolation accounting. | Chapter-5 text defines melt treatment in Eq. [5.1.1]. |
| Field-capacity-limited layer | `Θi <= FCi` causing `pei = 0` for the layer. | Explicit Eq. [5.4.1] branch behavior. |
| Water-stress day | `Ws` near `0` with limited `Ui` relative to `Etp`. | Valid stress response passed to plant-growth component. |
| Zero-demand transpiration day | `Etp = 0`, `Σ Ui = 0`, and `Ws = 1` by explicit branch semantics. | Prevents undefined ratio while preserving physically consistent no-demand behavior. |

## Invalid States

- Daily closure residual for Eq. [5.1.1] exceeding declared tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative flux/storage magnitudes for `Θ`, `Θin`, `P`, `I`, `Q`, `ET`, `D`, or `Qd` beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Interception outside `[0,P]` or interception computed from invalid biomass domain (`VE < 0`). `[DIRECT][Static] + [INFERENCE][Static]`
- Invalid layer-use relationship (`Ui > UPi` or negative `Ui`/`UPi`) or stress factor outside `[0,1]`. `[DIRECT][Static] + [INFERENCE][Static]`
- `Etp = 0` day with non-zero `Σ Ui`, or undefined/non-explicit `Ws` branch handling. `[DIRECT][Static] + [INFERENCE][Static]`
- Percolation emitted when `Θi <= FCi` or non-physical conductivity/percolation domain failure in Eq. [5.4.*]. `[DIRECT][Static]`
- Missing required coupling payloads from climate/runoff/subsurface/plant surfaces at daily closure assembly time. `[DIRECT][Static] + [INFERENCE][Static]`
- Water-balance boundary/output publication attempted without executed scheduler/kernel lane provenance (`INV-WATBAL-018`). `[DIRECT][Static] + [INFERENCE][Static]`
- Effective `wepp_ui` mode missing at lane selection boundary, or selected lane inconsistent with effective mode (`INV-WATBAL-019`). `[DIRECT][Static] + [INFERENCE][Static]`
- WB13/H.wat required candidate surfaces emitted from projection-only/synthetic reconstruction rather than simulation-owned execution surfaces (`INV-WATBAL-020`). `[DIRECT][Static] + [INFERENCE][Static]`
- Consolidated watbal intake/policy adoption performed without explicit provenance triage and typed guard disposition (`INV-WATBAL-021`). `[DIRECT][Static] + [INFERENCE][Static]`
- Continuous-run publication emits fewer rows than executed climate days, emits non-monotonic `sim_day_index`, or exports calendar-year keyed WB13 rows instead of simulation-year keys (`INV-WATBAL-022`). `[DIRECT][Static] + [INFERENCE][Static]`
- Replay staging omits explicit strict/parquet lane-policy classification or omits candidate source-class provenance (`native-runtime-dat`, `conversion-derived-dat`, `native-runtime-parquet`) (`INV-WATBAL-023`). `[DIRECT][Static] + [INFERENCE][Static]`
- Semantic comparator evidence reports unresolved `Total-Soil` alias continuity or placeholder-only parquet width diagnostics (`INV-WATBAL-024`). `[DIRECT][Static] + [INFERENCE][Static]`
- Replay promotability evidence is asserted without contract-derived closure tests for span/key overlap, strict-lane compensation, alias continuity, and conversion-derived dat row-consistency classification (`INV-WATBAL-025`). `[DIRECT][Static] + [INFERENCE][Static]`
- WB13/hydout publication maps `Snow-Water` to static sidecar controls (`snow.options.ssd`) rather than runtime snow-state surfaces (`snow.runtime_swe`) (`INV-WATBAL-026`). `[DIRECT][Static] + [INFERENCE][Static]`
- Published storage tuple (`Total-Soil`, `frozwt`, `Snow-Water`, `SoilWaterTotal`) remains invariant under non-zero forcing and thermal transitions (`INV-WATBAL-027`). `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-WATBAL-P-001: Emit daily closure terms (`Θ`, `Θin`, `P`, `I`, `S`, `Q`, `ET`, `D`, `Qd`) with declared units and sign conventions. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-002: Compute and retain daily closure residual for Eq. [5.1.1] and fail explicitly on violation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-003: Enforce all runtime guard checks before publishing downstream daily boundary outputs. `[INFERENCE][Static]`
- OBL-WATBAL-P-004: Propagate invariant violations as typed errors; no silent clamping/defaulting of hydrologic terms. `[INFERENCE][Static]`
- OBL-WATBAL-P-005: Replay-evidence producers for WB13 comparison lanes must
  emit semantic comparator artifacts with row-key deltas, per-column tolerance
  verdicts, and top divergent rows; silent omission of semantic diagnostics is
  invalid. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-006: Production execution pathways must publish explicit
  scheduler/kernel lane provenance for required water-balance outputs and
  must reject projection-only publication for required candidate surfaces.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-007: Effective `wepp_ui` mode must be propagated unchanged from
  parser boundary into lane selection and mismatch must surface typed failure;
  no silent daily fallback is permitted for missing/invalid mode closure.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-008: Consolidated kernel/policy intake from candidate sources
  must remain selective and triaged with explicit `adopt`/`defer`/`reject`
  disposition before runtime enablement claims are made. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-009: Continuous-run publication must emit one WB13/H.wat row
  per executed climate day, preserve monotonic `sim_day_index` (`1..N`), map
  `Y` to simulation-year key semantics, and publish continuity assertions in
  run provenance (executed day count, first/last key tuple, and monotonicity
  verdict). `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-010: Replay tooling producers must emit explicit strict-lane
  policy mode and candidate source classification in provenance for every run;
  strict/parquet mode defaults are forbidden and conversion-derived dat
  evidence must be tagged as non-promotable for final Tier-A closure claims.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-011: Semantic comparator producers must preserve investigation
  alias continuity for `Total-Soil` (including legacy `Total-Soil Water`
  inputs) and publish observed-row width diagnostics for both dat and parquet
  lanes without placeholder sentinel classes.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-012: Replay governance/test producers must maintain explicit
  contract-derived closure tests for `SIMIMPL13-TEST-001..005`, including
  span/key overlap assertions, strict-lane compensation checks, alias
  continuity checks, and conversion-derived dat row-consistency provenance
  gating before promotable Tier-A claims are emitted.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-013: WB13 publication producers must emit day-key `RM` from
  runtime liquid partition (`rain + melt`) and derive `Snow-Water` from runtime
  SWE state mapping; static sidecar controls are non-authoritative for dynamic
  storage publication.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-014: Multi-day publication producers must preserve dynamic
  mutation of storage-state surfaces (`Total-Soil`, `frozwt`, `Snow-Water`,
  `SoilWaterTotal`) when forcing/thermal drivers vary and must surface typed
  hard-fail evidence for static publication leakage.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-015: WB11 ET/soil-water migration producers must preserve
  baseline ordering and lineage authority (`purk -> evap/evappm -> drain/lateral -> swu -> watcon`)
  and must publish explicit lineage diagnostics proving `st(i)`/`soilw(i)` to
  `Total-Soil`/`SoilWaterTotal` continuity.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-016: WB18 aggregate storage producers must publish
  `wb11_soil_water` from baseline `watcon = Σsoilw(i)` semantics after
  percolation, using `wb18_perc_theta_####`, `thetdr_####`, `dg_####`, and
  declared frozen-depth lineage when present; `Σtheta`-only WB18 aggregate
  publication is non-authoritative.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-017: WB18 hourly restrictive-bottom producers must consume
  `slflag`, `kslast`, and `ui_bdrkth` for bottom-layer hourly `D`/`Pe` lineage
  when restrictive-layer metadata is present, and must fail closed on malformed
  restrictive domains rather than silently reverting to unrestricted `Ksi`.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-018: Post-ingress storage attribution producers must preserve
  trace evidence separating WB18 aggregate identity, `D=Pe`/`pei`, WB19
  lateral lineage, WB13 storage publication, and excluded snow/`RM` residual
  masks before assigning H1/H7/H39 `Total-Soil`/`SoilWaterTotal` residual
  ownership to production WB18/WB19 code.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-019: Cumulative storage-budget producers must preserve
  candidate/baseline row joins and trace evidence for storage deltas, `Ep`,
  `Es`, `Er`, `D`, `latqcc`, `Q`, `RM`, `Snow-Water`, WB18/WB19 identities,
  and excluded snow masks before asserting a WB17/WB18/WB19/WB13 production
  owner for H1/H7/H39 residuals.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-020: HPHYS0296/HPHYS0303 closure must preserve snow/`RM` acceptance
  evidence across fixed-comparator negative-melt lineage, runtime SWE closure, WB13
  `RM`/`Snow-Water` publication identity, `Q` closure, cumulative storage
  windows, reconstruction controlled experiment, independent correctness
  adjudication, and explicit defective-model disposition before any residual
  leaves the failing set.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-P-021: HPHYS0297 closure must publish the snow/`RM` defect ledger
  before any semantic re-tiering: observed candidate/baseline `RM`,
  reconstructed `/workdir/wepp-forest_260430_baseline` branch `RM`, named
  tolerance, reconstruction residual, closed `Q`, closed producer-consumer
  identity, correctness rationale, and final verdict. `[DIRECT][Static] +
  [INFERENCE][Static]`
- OBL-WATBAL-P-022: HPHYS0298 closure must publish the paired snow/`RM`
  partition ledger for all nine H1/H7/H39 target windows before any semantic
  re-tiering or downstream hydrology focus: baseline observe identity, same-HEAD
  full-suite metrics, candidate/baseline `RM`, ordered first-divergent
  cut-point, canonical values/units/source lines, closed `Q` and WB13 `RM`
  identity status, prohibited compensation paths, correctness rationale, and
  final verdict. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-WATBAL-C-001: Plant-growth consumers must use `Ws` only within declared domain and reject malformed stress payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-C-002: Infiltration/runoff consumers must treat `Θ`/near-surface moisture linkage in declared units without hidden reinterpretation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-C-003: Subsurface/drainage consumers must preserve `Qd` accounting semantics and avoid untracked reinjection into root-zone closure. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-C-004: All consumers must fail explicitly on invariant-violating payloads and propagate invariant IDs in error context. `[INFERENCE][Static]`
- OBL-WATBAL-C-005: Publication/replay consumers must preserve ET component and
  soil-water aggregate lineage semantics from simulation-owned runtime surfaces
  and reject projection-side recomputation that bypasses declared lineage checks.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-WATBAL-C-006: Water-balance storage reconciliation must consume runoff
  outputs produced from a melt-aware infiltration/runoff partition and WB18
  layer storage mutated by the same infiltrated liquid; it may not compensate
  for a missing `wmelt` infiltration/layer-ingress path by changing `Ep`, WB13
  publication, or aggregate storage formulas. `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Daily closure and sign domains (`INV-WATBAL-001/002`) | daily balance assembly | Hard error; reject day-state publish; log invariant ID and residual | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Interception and ET/uptake domains (`INV-WATBAL-003/004/005`) | ET/interception update stage | Hard error on invalid domains or non-physical extraction | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Percolation routing (`INV-WATBAL-006`) | layer routing stage | Hard error on eligibility/domain failure; block downstream percolation publish | Tier-A gate | `[DIRECT][Static]` |
| Coupling completeness (`INV-WATBAL-007`) | daily boundary handoff | Hard error on missing malformed field/units mismatch | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Governance boundary (`INV-WATBAL-008`) | review/verification/promotion | Governance `HOLD` until cross-contract ownership of `D` reuse is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB17/WB18/WB19 hydrology production execution + guards (`INV-WATBAL-009/010`) | ET/perc/lateral/drain kernel execution and routing/guard validation | Hard error on malformed hydrology domains or unsupported hydrology phase classes | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB13 replay-candidate schema/order and artifact completeness (`INV-WATBAL-012`) | WB13 output staging and replay boundary | Hard error when strict replay staging sees missing required WB13 symbols/artifacts or schema/ordering violations | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| CLIM05 snow-coupled WB12 storage closure (`INV-WATBAL-013`) | WB12 storage reconciliation stage | Hard error on missing/non-finite/domain-invalid signed `S` term or CLIM05 storage equation violation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL14R strict interchange-surface completeness (`INV-WATBAL-014`) | strict replay candidate staging boundary | Hard error / `HOLD` when candidate rerun artifact set omits `interchange/H.wat.parquet` or `interchange/H.pass.parquet`, or when fallback artifacts are substituted | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL15R schema-aligned WB13 supersession (`INV-WATBAL-015`) | strict replay delta reclassification boundary | Governance `HOLD` when residual WB13 blockers are asserted without evaluating superseding 25-column schema-aligned strict replay and keyed day-by-day parity evidence | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB20 forward-solver lane closure semantics (`INV-WATBAL-016`) | WB12 runoff/storage closure-delta assembly boundary | Hard error when forward-solver lane consumes observed targets in acceptance logic or emits non-residual closure deltas | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL14S semantic replay diagnostics completeness (`INV-WATBAL-017`) | semantic comparator report publication boundary | Hard error / `HOLD` when semantic replay evidence omits row-presence deltas, per-column verdicts, required investigation diagnostics, or baseline-only column disclosure | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMPIPE runner execution ownership closure (`INV-WATBAL-018`) | runner publication boundary for hydrology outputs | Hard error when required water-balance outputs are published without executed scheduler/kernel provenance | SIMIMPL execution gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMMODE lane-propagation closure (`INV-WATBAL-019`) | runner/orchestrator lane selector boundary | Hard error when selected lane diverges from effective `wepp_ui` mode or mode surfaces are missing | SIMIMPL execution gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMOUT simulation-owned WB13 provenance closure (`INV-WATBAL-020`) | WB13 output publication boundary | Hard error / `HOLD` when WB13 candidate surfaces are projection/synthesis-first rather than simulation-owned | Tier-A replay integrity gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMCONS consolidated-intake guard closure (`INV-WATBAL-021`) | consolidated-kernel adoption boundary | Governance `HOLD` when candidate kernel/policy intake lacks explicit triage/provenance disposition | Consolidated-intake gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL14 continuous WB13 span/key closure (`INV-WATBAL-022`) | continuous runner publication boundary | Hard error when run-span row count under-runs executed climate days, `sim_day_index` is non-monotonic, or `Y` key mapping diverges from simulation-year semantics | Tier-A replay span/key gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL15 strict-lane policy and source-provenance closure (`INV-WATBAL-023`) | replay staging + provenance manifest boundary | Hard error / `HOLD` when strict/parquet lane policy or candidate source classification is absent/ambiguous; conversion-derived dat strict evidence is explicitly non-promotable for final Tier-A closure | Tier-A replay tooling alignment gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL15 parquet alias/diagnostic continuity closure (`INV-WATBAL-024`) | semantic comparator report publication boundary | Hard error / `HOLD` when `Total-Soil` alias continuity is unresolved or parquet width diagnostics are placeholder-based instead of observed-width based | Tier-A replay tooling alignment gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL16 replay contract-derived test-coverage closure (`INV-WATBAL-025`) | replay governance/test evidence boundary | Hard error / `HOLD` when closure tests for span/key overlap, strict-lane compensation, alias continuity, or conversion-derived dat row-consistency are missing/failing | Tier-A replay contract-test closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL18 day-key partition/publication-source closure (`INV-WATBAL-026`) | WB13 day-key publication + hydout-equivalent mapping boundary | Hard error when `RM` is sourced from raw precipitation passthrough under snow-active cold branches or when `Snow-Water` publication leaks static sidecar controls | Tier-A hydrology publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL18 storage-state mutation closure (`INV-WATBAL-027`) | multi-day WB13 storage publication boundary | Hard error / `HOLD` when storage tuple publication is invariant across non-zero forcing/thermal variation and runtime mutation closure is not demonstrable | Tier-A hydrology mutation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL21 baseline WB11 ordering closure (`INV-WATBAL-028`) | WB11 ET/soil-water migration staging boundary | Hard error / `HOLD` when promoted ordering differs from canonical baseline sequence (`purk -> evap/evappm -> drain/lateral -> swu -> watcon`) | SIMIMPL ET/soil-water migration gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL21 aggregate lineage closure (`INV-WATBAL-029`) | WB13/hydout aggregate publication boundary | Hard error / `HOLD` when `Total-Soil`/`SoilWaterTotal` cannot be traced to declared layer-authoritative `st(i)`/`soilw(i)`/`watcon` lineage | SIMIMPL hydrology publication-lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Constants and Parameters Table

| Constant/parameter | Units | Domain | Contract use | Authority |
|---|---|---|---|---|
| `WB19_HYDRO_UNSUPPORTED_ROUTING_CODE` | status message id (legacy ID retained) | `HS-HYDRO-E-001` | Typed failure code for unsupported hydrology phase-class routing states | REF-WATBAL-PHYS-BOUNDS |
| `WB11_ET_GUARD_CODES` | status message id range | `HKERNEL-WB11-ET-E-001..003` | Typed ET guard codes for missing/non-finite/domain failures | REF-WATBAL-PHYS-BOUNDS |
| `WB17_ET_PARTITION_EXP_COEFF` | coefficient | `0.4` | WB17 ET LAI partition coefficient (`Esp = Eu * exp(-0.4 * L)`) | REF-WATBAL-CH5-ETDIST |
| `WB18_PERC_GUARD_CODES` | status message id range | `HKERNEL-WB11-PERC-E-001..003` | Typed WB18 per-layer percolation guard codes for missing/non-finite/domain failures | REF-WATBAL-PHYS-BOUNDS |
| `WB19_LATERAL_GUARD_CODES` | status message id range (legacy IDs retained) | `HKERNEL-WB11-LAT-E-001..003` | Typed WB19 lateral guard codes for missing/non-finite/domain failures | REF-WATBAL-PHYS-BOUNDS |
| `WB19_DRAINAGE_GUARD_CODES` | status message id range (legacy IDs retained) | `HKERNEL-WB11-DRAIN-E-001..003` | Typed WB19 drainage guard codes for missing/non-finite/domain failures | REF-WATBAL-PHYS-BOUNDS |
| `WB20_FORWARD_SOLVER_LANE_FLAG` | scalar | `{0,1}` (optional; absence selects compatibility lane) | WB20 selector controlling whether WB12 closure deltas are observed-target-driven (`0`) or solver-residual-derived (`1`) | REF-WATBAL-CH5-BAL, REF-WATBAL-PHYS-BOUNDS |
| `WB13_OUTPUT_STATUS_OK` | status message id | `HKERNEL-WB13-HWAT-OK-001` | Typed nominal status for WB13 daily output-row emission success | REF-WATBAL-PHYS-BOUNDS |
| `WB13_OUTPUT_GUARD_MISSING` | status message id | `HKERNEL-WB13-HWAT-E-001` | Typed missing-required-symbol guard code for WB13 daily output rows | REF-WATBAL-PHYS-BOUNDS |
| `WB13_OUTPUT_GUARD_NONFINITE` | status message id | `HKERNEL-WB13-HWAT-E-002` | Typed non-finite-value guard code for WB13 daily output rows | REF-WATBAL-PHYS-BOUNDS |
| `WB13_OUTPUT_GUARD_DOMAIN` | status message id | `HKERNEL-WB13-HWAT-E-003` | Typed domain/order/schema guard code for WB13 daily output rows | REF-WATBAL-PHYS-BOUNDS |
| `SIMPIPE_EXECUTION_OWNERSHIP_GUARD` | status message id | `HS-SIMPIPE-E-001` | Typed guard code for publication without executed scheduler/kernel provenance | REF-WATBAL-PHYS-BOUNDS |
| `SIMMODE_LANE_CLOSURE_GUARD` | status message id | `HS-SIMMODE-E-001` | Typed guard code for effective-mode to lane-selection mismatch | REF-WATBAL-INFILE-WEPPUI, REF-WATBAL-PHYS-BOUNDS |
| `SIMOUT_WB13_PROVENANCE_GUARD` | status message id | `HS-SIMOUT-E-001` | Typed guard code for projection-first/synthetic WB13 candidate publication | REF-WATBAL-PHYS-BOUNDS |
| `SIMIMPL14_WB13_SPAN_KEY_GUARD` | status message id | `HS-SIMOUT-E-001` | Typed guard code for continuous-run WB13 span/key closure failures (row-count under-run, non-monotonic keys, or non-simulation-year mapping) | REF-WATBAL-PHYS-BOUNDS |
| `SIMCONS_INTAKE_TRIAGE_GUARD` | status message id | `HS-SIMCONS-E-001` | Typed governance code for untriaged consolidated intake/adoption claims | REF-WATBAL-PHYS-BOUNDS |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not bitwise
parity). Contract-specific tolerances used for comparator interpretation:

| Tolerance ID | Definition | Value | Notes |
|---|---|---|---|
| TOL-WATBAL-001 | Daily closure residual for Eq. [5.1.1] | `<= 1e-9 m` | Residual is computed each daily step; runtime still hard-fails on material violation. |
| TOL-WATBAL-002 | Non-negative-domain comparator tolerance for state/flux magnitudes | lower bound `>= -1e-12 m` | Comparator-noise allowance only; no silent runtime clamping. |
| TOL-WATBAL-003 | Layer percolation non-negativity comparator tolerance (`pei`) | lower bound `>= -1e-12 m d^-1` | Negative values beyond tolerance are invalid-state failures. |
| TOL-WATBAL-004 | Stress-factor bound tolerance for `Ws` | `abs(bound violation) <= 1e-12` | Domain expectation remains `[0,1]`. |
| TOL-WATBAL-005 | Zero-demand transpiration threshold for denominator guard | `Etp <= 1e-12 m d^-1` treated as zero-demand branch | Runtime still requires explicit `Σ Ui = 0` and `Ws = 1` behavior. |
| TOL-WATBAL-006 | WB12/WB14 reconciled runoff near-zero canonicalization tolerance (`Q`, `wb12_runoff_reconciled`) | normalize to `0` when `-1e-12 m <= value < 0` before writeback/publication; `value < -1e-12 m` is domain-invalid | Explicit roundoff canonicalization only; not a fallback for material negative runoff. |

## Test-Vector Obligations

Minimum WB17/WB18/WB19 hydrology production-kernel conformance vectors:

1. WB17 ET phase emits deterministic partition outputs (`Ep`, `Es`, `Er`) plus
   closure outputs (`ET`, `Ws`) and mutates `wb11_soil_water` via typed
   writeback.
2. WB18 percolation plus WB19 lateral/drain phases emit deterministic outputs
   and mutate only declared WB18/WB19 state surfaces via typed writeback.
3. Non-finite and domain-invalid hydrology inputs hard-fail with typed
   guard codes and halt at the affected phase.
4. Unsupported hydrology phase-class combinations hard-fail with typed routing
   status (`HS-HYDRO-E-001`) and no fallback/default class rewrite.
5. INT10 coupled replay vectors:
   - canonical replay demonstrates watbal lane execution after successful plant
     transition phases in scheduler order;
   - hydrology phases observe state written by prior decomposition/growth
     transitions (state-transfer continuity);
   - missing or non-finite coupled ordering symbols hard-fail before watbal
     lane completion.
6. HPHYS0260 trace-localization vector proves opt-in trace rows serialize WB17
   layer uptake, WB18 layer flux/storage, residual/depth/frozen aggregate
   components, and final WB13 storage publication fields needed to classify
   H1/H7/H39 `Ep`/`Dp`/storage residuals.

## WB12 Reconciliation Authority Addendum

### WB12 Required Surfaces

| Surface | Symbols |
|---|---|
| Runoff reconciliation required/carryover inputs | `wb12_rainfall_input`, `wb12_runon_input`, same-pass flux `wb12_runoff_carryover` when published, `wb12_infiltration`, `wb12_depression_storage_delta`, `wb12_runoff_closure_tolerance` |
| Storage reconciliation required inputs | `wb12_storage_initial`, `wb12_storage_closure_tolerance`, `wb12_precip_input`, `S`, `Q`, `ET`, `D`, `Qd` |
| WB20 lane selector | `wb20_forward_solver_lane_enabled` (`0` compatibility lane, `1` forward-solver lane); symbol absence is compatibility lane |
| Compatibility-lane observed targets (optional outside forward lane) | `wb12_runoff_observed`, `wb12_storage_observed` |
| Runoff reconciliation outputs | `Q`, `wb12_runoff_carryover`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled` |
| Storage reconciliation outputs | `wb12_storage_closure_delta`, `wb12_storage_reconciled` |

### WB12 Deterministic Reconciliation Rules

1. Runoff reconciliation emits:
   - `runoff_carryover = wb12_runoff_carryover` when the same-pass flux is
     present and finite/non-negative; otherwise `runoff_carryover =
     wb12_runon_input` when the state surface is finite/non-negative.
   - `Q = wb12_rainfall_input + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta`
   - apply explicit near-zero canonicalization before writeback/closure-delta
     publication: if `Q` is in `[-1e-12, 0)`, set `Q = 0` and
     `wb12_runoff_reconciled = 0`; `Q < -1e-12` is a domain violation.
   - publish resolved `runoff_carryover` as same-pass flux
     `wb12_runoff_carryover`.
2. Storage reconciliation emits:
   - `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - Q - ET - D - Qd`
3. Closure-delta semantics are lane-scoped:
   - forward-solver lane (`wb20_forward_solver_lane_enabled = 1`):
     - `wb12_runoff_closure_delta = (wb12_rainfall_input + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta) - Q`
     - `wb12_storage_closure_delta = (wb12_storage_initial + wb12_precip_input + S - Q - ET - D - Qd) - wb12_storage_reconciled`
     - observed targets are excluded from acceptance-driving inputs.
   - compatibility lane (`wb20_forward_solver_lane_enabled = 0` or symbol absent):
     - `wb12_runoff_closure_delta = Q - wb12_runoff_observed`
     - `wb12_storage_closure_delta = wb12_storage_reconciled - wb12_storage_observed`
4. Absolute closure deltas above declared per-phase tolerances are invalid closure states.
5. Missing/non-finite/out-of-range inputs and invalid closure states hard-fail with typed status and do not apply writeback.

### WB12 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB12-RUNOFF-E-001` | `HKERNEL-WB12-RUNOFF-E-002` | `HKERNEL-WB12-RUNOFF-E-003` |
| Storage reconciliation | `HKERNEL-WB12-STORAGE-E-001` | `HKERNEL-WB12-STORAGE-E-002` | `HKERNEL-WB12-STORAGE-E-003` |

### WB12 Contract-Test Vectors

1. Valid WB12 runoff/storage inputs produce deterministic reconciliation outputs and state updates.
2. Non-finite WB12 runoff/state input hard-fails at the corresponding reconciliation phase with typed non-finite guard code.
3. Forward-solver lane vectors with perturbed `wb12_runoff_observed` and `wb12_storage_observed` still emit solver-residual closure deltas and remain acceptance-valid when other required inputs are valid.
4. Compatibility-lane vectors remain observed-target-driven and fail on closure-delta overflow beyond tolerance with typed domain/closure guard code and no writeback mutation.

## CLIM05 Snow-Coupled Reconciliation Addendum

### CLIM05 Required Coupling Surface

| Surface | Symbols |
|---|---|
| Signed snow-water coupling term | `S` (daily `+` melt, `-` accumulation) |

### CLIM05 Deterministic Coupling Rule

1. Runoff reconciliation consumes snow-coupled liquid input through signed `S`
   authority and emits `S` to downstream storage reconciliation.
2. WB12 storage reconciliation must include `S` exactly once in the storage
   closure equation (additive signed term).
3. Missing/non-finite/domain-invalid `S` is a hard-fail storage state.

### CLIM05 Contract-Test Vectors

1. Active snow coupling nominal vector changes both runoff closure (`Q`) and
   WB12 storage closure through signed `S`.
2. Missing `S` at storage reconciliation hard-fails with typed missing-input
   posture.
3. Non-finite or domain-invalid `S` hard-fails with typed non-finite/domain
   posture and no writeback mutation.

## CLIM06 Frozen-Soil Infiltration Coupling Addendum

### CLIM06 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Parsed frost controls | `frost.options.wintRed`, `frost.options.fineTop`, `frost.options.fineBot`, `frost.options.kfactor1`, `frost.options.kfactor2`, `frost.options.kfactor3`, `frost.options.frost_file_present` |
| Frozen-state runtime outputs | `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` |
| Runoff/storage reconciliation symbols | `wb12_infiltration`, `Q`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled`, `wb12_storage_reconciled` |

### CLIM06 Deterministic Coupling Rules

1. Active CLIM06 coupling is explicit when
   `frost.options.frost_file_present = 1` and `frost.options.wintRed = 1`.
2. WB14 runoff reconciliation must consume `frost.runtime_infcap_frz` as the
   frozen-soil effective infiltration-capacity term when active CLIM06 coupling
   is enabled.
3. CLIM06 frozen-state domains are bounded and non-negative:
   - `0 <= frost.runtime_dfrost <= 0.20`
   - `0 <= frost.runtime_dthaw <= 0.20`
   - `frost.runtime_nft >= 0`
   - `frost.runtime_ws_frz >= 0`
   - `0 <= frost.runtime_infcap_frz <= ssc`
4. Missing/non-finite/out-of-domain active-coupling frost symbols are
   hard-fail states in WB14 reconciliation; no fallback/default branch is
   allowed.

### CLIM06 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB14-RUNOFF-E-001` | `HKERNEL-WB14-RUNOFF-E-002` | `HKERNEL-WB14-RUNOFF-E-003` |

### CLIM06 Contract-Test Vectors

1. Active CLIM06 vector reduces infiltration-capacity and updates frozen-state
   outputs while preserving typed runoff/storage reconciliation closure.
2. Missing required active-coupling frost symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite active-coupling frost symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-002`.
4. Out-of-domain active-coupling frost symbol/state hard-fails with
   `HKERNEL-WB14-RUNOFF-E-003`.

## WB14 Infiltration and Hyetograph Coupling Addendum

### WB14 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Runoff reconciliation forcing | `ninten` or `nbrkpt`; `timem_####`; `intsty_####`; `ssc`; `dg`; `thetdr`; `thetfc` |
| Disturbed-soil conductivity-adjustment forcing | `solwpv`, `ksatadj`, `ksatfac`, `ksatrec`, `lkeff`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `dg_####` |
| Runoff reconciliation state/carryover inputs | `wb12_rainfall_input`, `wb12_runon_input`, same-pass flux `wb12_runoff_carryover` when published, `wb12_depression_storage_delta`, `wb12_runoff_closure_tolerance`, `wb20_forward_solver_lane_enabled` (`0`/absent compatibility, `1` forward-solver) |
| Compatibility-lane observed target input | `wb12_runoff_observed` (required only when compatibility-lane closure semantics are active) |
| Runoff reconciliation outputs | `wb12_infiltration`, `Q`, `wb12_runoff_carryover`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled` |

### WB14 Deterministic Coupling Rules

1. Runoff reconciliation computes infiltration from subdaily hyetograph forcing
   within the runoff kernel branch; externally seeded `wb12_infiltration` is no
   longer a required input for acceptance paths.
2. Runoff reconciliation derives WB14 effective infiltration conductivity
   `Ke` with baseline-authoritative `ksatadj` regime selection:
   - default path: `Ke = ssc`;
   - active disturbed path is gated by `ksatadj = 1`;
   - `sat_frac = min((theta_1 + theta_2)/(ul_1 + ul_2), 1.0)` from WB18
     first-two-layer stores;
   - `solwpv = 9001`: exponential recovery using `ksatfac` and `ksatrec`;
   - `solwpv >= 9002`: Saxton-Rawls Brooks-Corey effective conductivity with
     `psi = ln(1500/33)/ln(avthetafc/avthetadr)`, `lambda = 1/psi`,
     `keff = (ssc*3.6e6) * sat_frac^(2*lambda+3)`; `solwpv = 9003` applies
     `keff = max(keff, lkeff)` when `lkeff > 0`;
   - active-path domain violations are typed hard-fail states; no silent
     defaults/clamping.
3. Reconciliation uses computed infiltration and hyetograph rainfall depth in:
   - `runoff_carryover = wb12_runoff_carryover` when the same-pass flux is
     present and finite/non-negative; otherwise `runoff_carryover =
     wb12_runon_input` when the state surface is finite/non-negative.
   - `Q = wb14_hyetograph_rainfall + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta`
   - apply explicit near-zero canonicalization before writeback/closure-delta
     publication: if `Q` is in `[-1e-12, 0)`, set `Q = 0` and
     `wb12_runoff_reconciled = 0`; `Q < -1e-12` is a domain violation.
   - publish resolved `runoff_carryover` as same-pass flux
     `wb12_runoff_carryover`.
4. `wb12_rainfall_input` remains a required closure-consistency surface and must
   match hyetograph-integrated rainfall depth within
   `wb12_runoff_closure_tolerance`.
5. WB20 lane branch semantics apply to runoff closure delta:
   - forward-solver lane (`wb20_forward_solver_lane_enabled = 1`):
    `wb12_runoff_closure_delta = (wb14_hyetograph_rainfall + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta) - Q`
   - compatibility lane (`wb20_forward_solver_lane_enabled = 0` or symbol absent):
     `wb12_runoff_closure_delta = Q - wb12_runoff_observed`
6. Reconciliation and downstream storage closure (`wb12_storage_reconciled`)
   remain deterministic and typed-fail on missing/non-finite/domain-invalid
   inputs.

### WB14 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB14-RUNOFF-E-001` | `HKERNEL-WB14-RUNOFF-E-002` | `HKERNEL-WB14-RUNOFF-E-003` |

### WB14 Contract-Test Vectors

1. Valid hyetograph + soil infiltration forcing emits deterministic
   `wb12_infiltration`, `Q`, and WB12 runoff closure diagnostics.
2. Missing required WB14 forcing symbol hard-fails at runoff reconciliation
   with `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite WB14 forcing/reconciliation symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-002`.
4. Non-monotone hyetograph time, negative intensity, rainfall mismatch, or
   runoff closure overflow hard-fail with `HKERNEL-WB14-RUNOFF-E-003`.
5. Active `ksatadj` regime vectors (`solwpv=9001/9002/9003`) produce
   deterministic conductivity-adjusted infiltration behavior and preserve typed
   hard-fail posture for invalid active-regime domains.
6. Within-tolerance negative reconciled runoff (`-1e-12 <= Q < 0`) is
   canonicalized to zero at writeback/publication boundary; values below
   tolerance remain typed domain failures.

## WB15 Canopy Interception Coupling Addendum

### WB15 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Plant runtime interception inputs | `cancov`, `lai`, `vdmt` |
| Runoff/infiltration forcing inputs | `timem_####`, `intsty_####`, `wb12_rainfall_input` |
| Interception + closure outputs | `I`, `wb12_infiltration`, `Q`, `wb12_storage_reconciled` |

### WB15 Deterministic Coupling Rules

1. Canopy interception is computed before runoff/infiltration reconciliation
   and before daily storage closure acceptance.
2. Biomass context for interception uses live above-ground biomass proxy:
   - `VE = vdmt * 10000` (`kg ha^-1`)
3. Canopy-interception potential follows Chapter-5 Eq. [5.1.2] lineage:
   - `Ipot = cancov * ((0.000627 * VE - 3.73349e-8 * VE^2) / 1000)` (`m`)
4. Runtime interception is bounded by available hyetograph rainfall:
   - `I = min(Ipot, wb14_hyetograph_rainfall)` when `lai > 0` and `cancov > 0`
   - `I = 0` when `lai <= 0` or `cancov <= 0`
   - apply explicit near-zero canonicalization before writeback/publication: if
     snow/rain partition roundoff produces `I` or
     `wb14_hyetograph_liquid_after_interception` in `[-1e-12, 0)`, set the
     affected value to `0`; values below `-1e-12` remain typed domain
     violations.
5. Domain requirements are hard-fail:
   - `0 <= cancov <= 0.999`
   - `lai >= 0`
   - `0 <= vdmt <= 0.8` (`kg m^-2`) so `0 <= VE <= 8000` (`kg ha^-1`)
6. Runoff/infiltration reconciliation consumes interception explicitly:
   - `wb14_hyetograph_liquid_after_interception = wb14_hyetograph_rainfall - I`
   - `Q = wb14_hyetograph_liquid_after_interception + S + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta`
7. Daily storage closure consumes interception as an explicit Chapter-5 term:
   - `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - I - Q - ET - D - Qd`
8. Missing/non-finite/out-of-domain canopy interception symbols are hard-fail
   invalid states. No fallback/default/clamp behavior is allowed.

### WB15 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB14-RUNOFF-E-001` | `HKERNEL-WB14-RUNOFF-E-002` | `HKERNEL-WB14-RUNOFF-E-003` |
| Storage reconciliation | `HKERNEL-WB12-STORAGE-E-001` | `HKERNEL-WB12-STORAGE-E-002` | `HKERNEL-WB12-STORAGE-E-003` |

### WB15 Contract-Test Vectors

1. Valid `cancov`/`lai`/`vdmt` inputs emit finite `I` and deterministic
   coupled `wb12_infiltration`, `Q`, and `wb12_storage_reconciled`.
2. Missing canopy interception input symbol (`cancov`, `lai`, or `vdmt`)
   hard-fails with `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite canopy interception input hard-fails with
   `HKERNEL-WB14-RUNOFF-E-002`.
4. Out-of-domain canopy interception input (`cancov`, `lai`, `vdmt`) or
   coupled closure overflow hard-fails with `HKERNEL-WB14-RUNOFF-E-003` or
   `HKERNEL-WB12-STORAGE-E-003` at the affected phase.
5. Within-tolerance negative interception/liquid values caused by finite
   snow/rain partition roundoff are canonicalized to zero at the publication
   boundary; material negatives remain typed domain failures.

## IRRIG10 Irrigation Storage-Coupling Addendum

### IRRIG10 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Runtime irrigation schedule traces | `irrigation.runtime_schedule_source`, `irrigation.runtime_depth_m`, `Irr` |
| Runoff reconciliation coupled outputs | `Q`, `wb12_runoff_reconciled`, `wb12_runoff_closure_delta` |
| Storage reconciliation outputs | `wb12_storage_reconciled`, `wb12_storage_closure_delta` |

### IRRIG10 Deterministic Storage Rules

1. Storage reconciliation consumes daily irrigation depth as explicit input:
   `Irr = irrigation.runtime_depth_m`.
2. Storage closure equation under irrigation is explicit:
   `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S + Irr - I - Q - ET - D - Qd`.
3. `Irr` must be finite and non-negative whenever published.
4. Missing/non-finite/out-of-domain irrigation storage symbols are hard-fail
   invalid states; no fallback/default branch is allowed.

### IRRIG10 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Runoff reconciliation | `HKERNEL-WB14-RUNOFF-E-001` | `HKERNEL-WB14-RUNOFF-E-002` | `HKERNEL-WB14-RUNOFF-E-003` |
| Storage reconciliation | `HKERNEL-WB12-STORAGE-E-001` | `HKERNEL-WB12-STORAGE-E-002` | `HKERNEL-WB12-STORAGE-E-003` |

### IRRIG10 Contract-Test Vectors

1. Active fixed-date sprinkler event emits positive `Irr` and deterministic
   irrigation-coupled `wb12_storage_reconciled`.
2. Active depletion sprinkler event emits positive `Irr` and deterministic
   irrigation-coupled `wb12_storage_reconciled`.
3. Missing irrigation scheduling/storage symbols hard-fail with typed missing
   guard posture.
4. Non-finite/out-of-domain irrigation scheduling/storage symbols hard-fail
   with typed non-finite/domain guard posture.

## WB16 Peak Runoff Closure-Diagnostics Addendum

### WB16 Required Coupling Surfaces

| Surface | Symbols |
|---|---|
| Closure-diagnostics required inputs | `Q`, `timem_####`, `intsty_####`, `efflen`, `ealpha`, `m`, `I`, `irrigation.runtime_rate_m_per_s` |
| Closure-diagnostics peak outputs | `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` |
| Closure-diagnostics provenance outputs | `wb16_ealpha_compatibility_seed_used`, `wb16_ealpha_seed_policy` |

### WB16 Deterministic Peak-Flow Rules

1. WB16 executes at closure diagnostics and consumes reconciled runoff depth
   `Q` from WB14 plus coupled runtime forcing metadata from the accepted event.
2. Baseline-authoritative near-zero runoff branch from
   `/workdir/wepp-forest_260430_baseline/src/appmth.for` applies first:
   - if `Q < 1.0e-8`, emit `peakro_raw = 0`, then canonicalize
     `peakro = 3.63e-8` and `watdur = 0`.
3. Event duration for WB16 is derived from hyetograph elapsed time:
   - `effdrr = timem_last - timem_first` (`s`)
4. Mean runoff rate and runoff-maximum ratio terms are:
   - `vave = Q / effdrr`
   - `remax = max(intsty_####) + irrigation.runtime_rate_m_per_s`
   - `vstar = vave / remax`
5. Kinematic-wave time ratio and branch selector terms follow Chapter-4
   lineage (`appmth.for`):
   - `te = (efflen / (ealpha * vave^(m-1)))^(1/m)`
   - `tstar = te / effdrr`
   - if `vstar < 1`, `tc = (1 - sqrt(1 - 2.4 * (1 - vstar) * vstar)) / (1.2 * (1 - vstar))`
6. Peak-runoff nondimensional ratio `qpstar` is branch-authoritative:
   - partial-equilibrium branch (`tstar >= 1`): `qpstar = 1 / tstar^m`
   - quasi-equilibrium branch A (`vstar < 1` and `tc < tstar < 1`):
     `qpstar = 1 / tstar`
   - quasi-equilibrium branch B (`vstar < 1` and `0 < tstar <= tc`):
     `qpstar = 1/vstar - 0.6 * ((1 - vstar) / vstar) * tstar`
   - constant-excess branch (`vstar >= 1` and `tstar < 1`): `qpstar = 1`
7. Peak runoff and duration outputs are:
   - `peakro_raw = vave * qpstar`
   - `peakro = max(peakro_raw, 3.63e-8)` (legacy minimum-flow floor from
     `conrun.for`)
   - `watdur = Q / peakro`
8. Duration cap rule is explicit:
   - if `watdur > 86400`, set `watdur = 86400`.
9. WB16 domain posture is hard-fail for missing/non-finite/out-of-domain
   symbols and non-physical intermediates (`effdrr <= 0`, `vave <= 0`,
   `remax <= 0`, `vstar <= 0`, `m <= 0`, `ealpha <= 0`, `efflen <= 0`,
   negative `tc` discriminant for `vstar < 1`, or non-finite `peakro`/`watdur`).
   No fallback/default branch is allowed.
10. Positive near-zero WB16 intermediates are valid and must not hard-fail
    solely due epsilon-threshold comparisons prior to baseline floor
    canonicalization.
11. `m` producer authority is baseline-canonical and constant:
    `/workdir/wepp-forest_260430_baseline/src/rdat.for` assigns `m = 1.5`
    (Chezy depth-discharge exponent), and runtime producers must preserve this
    value unless superseded by canonical contract amendment.
12. `ealpha` producer authority is baseline-canonical as a chain:
    `frcfac -> rdat(alpha) -> alphay -> eplane(optional multi-OFE projection)`
    (`/workdir/wepp-forest_260430_baseline/src/frcfac.for`,
    `rdat.for`, `irs.for`, `eplane.for`).
13. Runtime lanes with complete producer inputs must publish baseline-lineage
    `ealpha` from the authoritative producer chain with explicit provenance:
    - `wb16_ealpha_compatibility_seed_used = false`
    - `wb16_ealpha_seed_policy = "runtime_provided"`
14. Compatibility seeding (`ealpha = 1.0`) is allowed only as a typed
    degradation branch when required producer inputs are unavailable, and only
    with explicit provenance publication and warning:
    - `wb16_ealpha_compatibility_seed_used = true`
    - `wb16_ealpha_seed_policy = "compatibility_seed_1p0"`
    - warning text containing `SIMPIPE-W-003`
    Compatibility-seed runs are non-promotable for full WB16
    input-provenance parity closure.

### WB16 Guard Codes

| Phase | Missing | Non-finite | Domain/closure |
|---|---|---|---|
| Closure diagnostics | `HKERNEL-WB16-PEAK-E-001` | `HKERNEL-WB16-PEAK-E-002` | `HKERNEL-WB16-PEAK-E-003` |

### WB16 Contract-Test Vectors

1. Nominal WB16 vector emits finite `peakro` and `watdur` with continuity
   `watdur = Q/peakro` and one authoritative method branch id.
2. Branch-selector vectors independently trigger:
   - `tstar >= 1`,
   - `vstar < 1` with `tc < tstar < 1`,
   - `vstar < 1` with `0 < tstar <= tc`,
   - `vstar >= 1` with `tstar < 1`.
3. Missing required WB16 symbol hard-fails in closure diagnostics with
   `HKERNEL-WB16-PEAK-E-001`.
4. Non-finite WB16 required symbol hard-fails with `HKERNEL-WB16-PEAK-E-002`.
5. Domain-invalid WB16 symbol/intermediate hard-fails with
   `HKERNEL-WB16-PEAK-E-003`.
6. Near-zero positive runoff vector (`0 < Q < 1.0e-8`) executes the
   baseline-authoritative branch, emits `peakro = 3.63e-8`, `watdur = 0`,
   and does not hard-fail.
7. Runtime-producer provenance vector: when required producer symbols are
   available, runtime emits
   `wb16_ealpha_compatibility_seed_used = false`,
   `wb16_ealpha_seed_policy = "runtime_provided"`, and no `SIMPIPE-W-003`
   warning.
8. Compatibility-seed provenance vector: when `ealpha` is not
   runtime-produced and compatibility seeding is invoked, runtime emits
   `wb16_ealpha_compatibility_seed_used = true`,
   `wb16_ealpha_seed_policy = "compatibility_seed_1p0"`, and warning id
   `SIMPIPE-W-003`.

## WB13 Daily Output-Surface Authority Addendum

### WB13 Canonical Daily Output Schema (`interchange/H.wat.parquet` projection)

WB13 daily output rows are authoritative at exactly 25 numeric columns in this
canonical order:

1. `OFE`
2. `J`
3. `Y`
4. `P`
5. `RM`
6. `Q`
7. `Ep`
8. `Es`
9. `Er`
10. `Dp`
11. `UpStrmQ`
12. `SubRIn`
13. `latqcc`
14. `Total-Soil`
15. `frozwt`
16. `Snow-Water`
17. `QOFE`
18. `Tile`
19. `Irr`
20. `Area`
21. `SoilWaterTotal`
22. `ProfileDepth`
23. `ProfilePorosityCap`
24. `ProfileFCStore`
25. `ProfileWPStore`

### WB13 Deterministic Row Keys and Ordering

1. WB13 row keys are authoritative as `(Y, J, OFE)` and must be strictly
   monotonic non-decreasing in emitted order.
2. Duplicate row keys within one emitted daily surface are invalid.
3. Output rows must remain deterministic under identical accepted upstream
   daily kernel surfaces.

### WB13 Output-Surface Invariants

1. `QOFE = Q` for canonicalized WB13 daily rows, including MOFE multi-OFE
   publication contexts.
2. `SoilWaterTotal = Total-Soil + frozwt` within `1e-6 mm`.
3. `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.
4. Required depth-like and storage-like columns in this WB13 surface are
   non-negative.
5. WB13 subsurface-loss publication and coupling checks are flux-authoritative
   for `D`, `q`, `Qdd`, and `Qd` when both state and flux surfaces expose the
   same symbol name.
6. Missing required symbols, non-finite values, and schema/order violations
   are hard-fail invalid states.

### WB13 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB13-HWAT-E-001` |
| Non-finite required symbol | `HKERNEL-WB13-HWAT-E-002` |
| Domain/order/schema violation | `HKERNEL-WB13-HWAT-E-003` |

### WB13 Contract-Test Vectors

1. Nominal WB13 vector emits deterministic 25-column rows in canonical schema
   order and deterministic `(Y, J, OFE)` ordering.
2. Missing required WB13 symbol (for example `ProfileDepth`) hard-fails with
   `HKERNEL-WB13-HWAT-E-001`.
3. Non-finite WB13 symbol hard-fails with `HKERNEL-WB13-HWAT-E-002`.
4. Domain/order/schema violations hard-fail with `HKERNEL-WB13-HWAT-E-003`
   and do not emit malformed rows.
5. PL14 strict replay staging rejects missing WB13 replay artifacts and does
   not synthesize fallback rows/files to satisfy comparator replay surfaces.
6. PL14R strict rerun staging requires explicit candidate-lane coverage for
   both `interchange/H.wat.parquet` and `interchange/H.pass.parquet`; omission
   of either required surface is a hard-fail + `HOLD`.
7. PL15R recloseout vectors must classify `H.wat.parquet` residual status from
   the schema-aligned strict replay set (parquet comparator JSON plus day-by-day
   parity artifact), not solely from pre-alignment structure-diff signatures.
8. PL14S semantic vectors require persisted report evidence for:
   - row-presence deltas keyed by `(OFE,J,Y)`,
   - per-column tolerance verdicts,
   - required investigation columns,
   - explicit baseline-only column disclosure and top divergent rows.
9. SIMPIPE vectors:
   - required watbal outputs published only when executed lane provenance is
     present;
   - projection-only publication attempt hard-fails with `HS-SIMPIPE-E-001`.
10. SIMMODE vectors:
   - `ui_run=0` selects daily lane;
   - `ui_run=1` selects hourly lane;
   - any mismatch or missing mode surfaces hard-fails with
     `HS-SIMMODE-E-001`.
11. SIMOUT vectors:
   - WB13 candidate rows emitted from executed lane surfaces publish as valid;
   - projection-only/synthetic substitution hard-fails with
     `HS-SIMOUT-E-001`.
12. SIMCONS vectors:
   - consolidated kernel/policy adoption claim without explicit
     `adopt`/`defer`/`reject` disposition hard-fails governance with
     `HS-SIMCONS-E-001`.

### HPARITY01 Always-Fail Column Lineage Register

HPARITY01 replay evidence from
`/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/semantic`
reports 39/39 hillslopes failing the same 12 WB13 columns. This register
defines canonical lineage owners, disambiguation posture, and guard families
for follow-on closure packages.

| WB13 column | Canonical lineage symbol | Cross-contract authority anchors | Runtime writer surface | Guard families |
|---|---|---|---|---|
| `Dp` | `D -> Dp` (deep percolation loss) | `SC-PERC-001` WB13 Daily Output Coupling Addendum; `SC-WATBAL-001` WB13 schema/guard addendum | `crates/openwepp-runner/src/hillslope/mod.rs` (`require_runtime_surface_scalar_prefer_flux("D")` -> `("Dp", dp)`) | `HKERNEL-WB11-PERC-E-001..003`, `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `Ep` | `Ep -> Ep` (plant transpiration component) | `SC-EVAP-001` WB13 Daily Output Coupling Addendum; `SC-WATBAL-001` WB13 schema/guard addendum | `crates/openwepp-runner/src/hillslope/mod.rs` (`require_runtime_surface_scalar_prefer_flux("Ep")` -> `("Ep", ep)`) | `HKERNEL-WB11-ET-E-001..003`, `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `Es` | `Es -> Es` (soil evaporation component) | `SC-EVAP-001` WB13 Daily Output Coupling Addendum; `SC-WATBAL-001` WB13 schema/guard addendum | `crates/openwepp-runner/src/hillslope/mod.rs` (`require_runtime_surface_scalar_prefer_flux("Es")` -> `("Es", es)`) | `HKERNEL-WB11-ET-E-001..003`, `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `Er` | `Er -> Er` (residue evaporation component) | `SC-EVAP-001` WB13 Daily Output Coupling Addendum; `SC-WATBAL-001` WB13 schema/guard addendum | `crates/openwepp-runner/src/hillslope/mod.rs` (`require_runtime_surface_scalar_prefer_flux("Er")` -> `("Er", er)`) | `HKERNEL-WB11-ET-E-001..003`, `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `ProfileDepth` | `solthk -> ProfileDepth` | `SC-PERC-001` WB13 Daily Output Coupling Addendum; `SC-WATBAL-001` WB13 output invariants | `crates/openwepp-runner/src/hillslope/mod.rs` (`require_runtime_surface_scalar("solthk")` -> `("ProfileDepth", profile_depth_mm)`) | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `ProfilePorosityCap` | `sum(por_i * dg_i)` | `SC-PERC-001` WB13 Daily Output Coupling Addendum; `SC-WATBAL-001` WB13 output invariants + HPARITY02 profile-lineage closure | `crates/openwepp-runner/src/hillslope/mod.rs` consumes `wb13_profile_porosity_cap_mm` (or complete per-layer `theta_s_####` fallback) for `("ProfilePorosityCap", profile_porosity_cap)` publication | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `ProfileFCStore` | `Σ(thetfc_i * dg_i) * 1000 + wb13_profile_fc_tail_mm -> ProfileFCStore` | `SC-PERC-001` WB13 Daily Output Coupling Addendum; `SC-WATBAL-001` WB13 output invariants + HPHYS0216D layer+tail reconciliation | `crates/openwepp-runner/src/hillslope/mod.rs` (layer-authoritative aggregation from `thetfc_####` + `dg_####` plus explicit normalized-tail symbol `wb13_profile_fc_tail_mm` into `("ProfileFCStore", profile_fc_store_mm)`) | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `ProfileWPStore` | `wb13_profile_wp_store_mm -> ProfileWPStore` | `SC-PERC-001` WB13 Daily Output Coupling Addendum; `SC-WATBAL-001` WB13 output invariants + HPHYS0207 depth-authority closure | `crates/openwepp-runner/src/hillslope/mod.rs` (`runtime_surface_symbol_value("wb13_profile_wp_store_mm")` -> `("ProfileWPStore", profile_wp_store_mm)`) | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `RM` | `snow.post_winter_rain_m + snow.routed_melt_m + Irr` | `SC-WATBAL-001` `INV-WATBAL-064/065`; `SC-RUNOFFPART-001` `INV-RUNOFFPART-019/020`; `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-022/023` | `crates/openwepp-runner/src/hillslope/mod.rs` consumes explicit `snow.post_winter_rain_m` and `snow.routed_melt_m` flux surfaces -> `("RM", rm)` | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `Snow-Water` | `snow.runtime_swe -> Snow-Water` | `SC-SNOWFREEZE-001` runtime-SWE publication authority; `SC-WATBAL-001` `INV-WATBAL-026/027` | `crates/openwepp-runner/src/hillslope/mod.rs` (`require_runtime_surface_scalar("snow.runtime_swe")` -> `("Snow-Water", snow_water)`) | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `latqcc` | `q -> latqcc` (lateral contribution) | `SC-SUBHYD-001` WB13 Daily Output Coupling Addendum; `SC-WATBAL-001` WB19 lateral coupling | `crates/openwepp-runner/src/hillslope/mod.rs` (`require_runtime_surface_scalar_prefer_flux("q")` -> `("latqcc", latqcc)`) | `HKERNEL-WB11-LAT-E-001..003`, `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `Total-Soil` | `wb11_soil_water -> Total-Soil` | `SC-SOIL-001` `INV-SOIL-013`; `SC-WATBAL-001` `INV-WATBAL-029`; `SC-SYSTEM-001` `INV-SYSTEM-027` | `crates/openwepp-runner/src/hillslope/mod.rs` (`require_runtime_surface_scalar("wb11_soil_water")` -> `("Total-Soil", total_soil)`) | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |
| `SoilWaterTotal` | `Total-Soil + frozwt -> SoilWaterTotal` | `SC-WATBAL-001` WB13 output invariants; `SC-SYSTEM-001` `INV-SYSTEM-027` | `crates/openwepp-runner/src/hillslope/mod.rs` (`soil_water_total = total_soil + frozwt` -> `("SoilWaterTotal", soil_water_total)`) | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` |

Alias continuity policy for this family is explicit:
1. Canonical publication symbol is `Total-Soil`.
2. Legacy semantic alias `Total-Soil Water` is comparator-only and must map to
   canonical `Total-Soil`.
3. `SoilWaterTotal` remains a distinct aggregate column with closure
   `SoilWaterTotal = Total-Soil + frozwt`.

### HPARITY02 Profile-Capacity Publication Lineage Closure

HPARITY02 closes WB13 profile-capacity publication lineage for
`ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, and `ProfileWPStore`
using baseline-authoritative soil preprocessing and profile aggregation
semantics from `/workdir/wepp-forest_260430_baseline` (`input.for`,
`scon.for`, `watbal.for`, `watbalprint.for`) at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

Required runtime publication symbols for WB13 profile columns:

1. `wb13_profile_depth_mm`
2. `wb13_profile_porosity_cap_mm`
3. Layer aggregation symbols for FC publication:
   `nsl`, `thetfc_####`, `dg_####`.
4. `wb13_profile_fc_tail_mm` (runtime-owned normalized-tail contribution for
   WB13 FC publication under HPHYS0216D).
5. `wb13_profile_wp_store_mm` (runtime-owned normalized-profile storage
   authority for WB13 profile WP publication under HPHYS0209/HPHYS0216)

Deterministic publication rules:

1. `wb13_profile_depth_mm` and `wb13_profile_porosity_cap_mm` remain
   producer-authoritative profile publication symbols when present.
2. `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, and
   `ProfileWPStore` remain non-negative `mm` depth-equivalent publication
   fields under WB13 schema/order authority.
3. `ProfileFCStore` is published from layer-authoritative aggregation plus
   explicit normalized-tail contribution:
   `ProfileFCStore = Σ(thetfc_i * dg_i) * 1000 + wb13_profile_fc_tail_mm`.
4. `wb13_profile_fc_tail_mm` must be finite and non-negative; missing or
   invalid tail symbols hard-fail WB13 publication.
5. `wb13_profile_fc_store_mm` is an optional diagnostic carry surface and is
   not direct publication authority for `ProfileFCStore`.
6. Synthesized placeholder formulas for `ProfilePorosityCap` are prohibited
   (for example `max(ProfileFCStore, ProfileWPStore) + C`).
7. Missing/non-finite/domain-invalid profile-lineage symbols hard-fail WB13
   publication under existing WB13 guard-family continuity.

### HPHYS0202 ProfileFC/ProfileWP Layer-Aggregation Lineage Closure (Historical)

HPHYS0202 amends WB13 profile-storage publication authority so
`ProfileFCStore` and `ProfileWPStore` are simulation-owned layer aggregates
from canonical WB11/WB13 state lineage (`watbal.for`/`watbalprint.for`):

1. `ProfileFCStore = Σ(thetfc_i * dg_i) * 1000` in `mm`.
2. `ProfileWPStore = Σ(thetdr_i * dg_i) * 1000` in `mm`.
3. Required aggregation symbols are per-layer `thetfc_####`, `thetdr_####`,
   and `dg_####` runtime surfaces for `i in [1..nsl]`.
4. Optional adapter seed symbols `wb13_profile_fc_store_mm` and
   `wb13_profile_wp_store_mm` are diagnostic carry surfaces only and must not
   override WB13 publication values. This historical rule is superseded by
   HPHYS0207 depth-authority closure below.
5. Missing/non-finite/domain-invalid layer aggregation symbols hard-fail WB13
   publication under existing guard-family continuity.

### HPHYS0205 Corrected-Layer Authority Closure (Historical)

HPHYS0205 closes the layer-source ambiguity identified in HPHYS0202 by
requiring the authoritative layer symbols consumed by WB13 publication to carry
baseline-corrected moisture lineage, not raw parser theta inputs:

1. Authoritative `thetfc_####`/`thetdr_####` runtime symbols used for WB13
   profile storage publication must be projected from baseline-authoritative
   soil-correction lineage (`scon` family: rock/entrapped-air adjustment and
   moisture-curve domain corrections), when that lineage is available.
2. WB13 `ProfileFCStore`/`ProfileWPStore` publication remains runtime-owned,
   and this historical layer-aggregation authority is superseded by HPHYS0207
   normalized-profile storage authority below.
3. Optional adapter profile-storage diagnostics
   (`wb13_profile_fc_store_mm`, `wb13_profile_wp_store_mm`) remain
   non-authoritative for publication in this historical amendment and must not
   disagree with corrected-layer aggregates when both are present and finite.
4. Missing/non-finite/domain-invalid corrected-layer symbols are typed
   publication-domain violations and hard-fail under existing WB13 guard
   continuity.

### HPHYS0206 Corrected-Layer Normalization and Mapping Closure (Historical)

HPHYS0206 closes residual authoritative-layer mapping ambiguity by requiring
the corrected FC/WP publication lineage to use the same normalized layer set
as profile-capacity lineage and deterministic OFE-layer mapping semantics:

1. Authoritative `thetfc_####`/`thetdr_####` publication-consumer symbols must
   be derived from corrected layers computed on the same baseline-normalized
   profile layer set that governs `wb13_profile_depth_mm` and
   `wb13_profile_porosity_cap_mm`.
2. Mapping from normalized corrected layers to emitted OFE layer symbols
   (`thetfc_####`/`thetdr_####`) must be deterministic and depth-domain
   complete for each emitted layer interval.
3. Profile-storage diagnostics (`wb13_profile_fc_store_mm`,
   `wb13_profile_wp_store_mm`) remain non-authoritative for WB13 publication in
   this historical amendment and must not override layer-authoritative
   publication values.
4. Missing normalized corrected-lineage inputs, incomplete normalized-layer
   coverage, or non-finite/domain-invalid mapped authoritative layer symbols are
   typed fail-closed boundary violations; raw parser-theta fallback for
   authoritative FC/WP publication symbols is prohibited.

### HPHYS0207 FC/WP Depth-Authority and Normalized-Tail Closure

HPHYS0207 closes the normalized-profile versus parser-layer depth mismatch by
aligning WB13 FC/WP publication authority to normalized-profile storage symbols
that share domain authority with profile depth/capacity surfaces:

1. `ProfileFCStore` publication authority is `wb13_profile_fc_store_mm` when
   present and valid.
2. `ProfileWPStore` publication authority is `wb13_profile_wp_store_mm` when
   present and valid.
3. `wb13_profile_fc_store_mm`/`wb13_profile_wp_store_mm` must be runtime-owned
   normalized-profile aggregates from baseline-corrected layer lineage and must
   share depth-domain authority with `wb13_profile_depth_mm` and
   `wb13_profile_porosity_cap_mm`.
4. Residual normalized-tail depth beyond OFE parser-layer publication depth is
   consumed by normalized-profile storage projection authority
   (`wb13_profile_fc_store_mm`/`wb13_profile_wp_store_mm`); normalized-tail
   truncation is forbidden and no fallback may silently republish
   parser-domain aggregates as WB13 profile storage authority.
5. Required WB13 profile ordering remains:
   `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.

### HPHYS0216 ProfileFC Layer-Authority Realignment

HPHYS0216 closes the `ProfileFCStore` structural split by restoring
baseline-authoritative FC publication lineage from `watbal.for` /
`watbalprint.for` while preserving corrected-layer symbol projection and typed
guards:

1. `ProfileFCStore` publication authority is layer aggregation from emitted
   authoritative symbols:
   `ProfileFCStore = Σ(thetfc_i * dg_i) * 1000`, `i in [1..nsl]`.
2. `wb13_profile_fc_store_mm` remains a diagnostic carry surface and is not a
   publication-driving authority symbol for `ProfileFCStore`.
3. `ProfileWPStore` authority remains `wb13_profile_wp_store_mm` under
   HPHYS0209 unless superseded by later contract amendment.
4. Missing/non-finite/domain-invalid layer aggregation symbols (`nsl`,
   `thetfc_####`, `dg_####`) are typed fail-closed WB13 publication-domain
   violations.
5. Required WB13 profile ordering remains:
   `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.

### HPHYS0216D ProfileFC Layer+Tail Authority Reconciliation

HPHYS0216D closes the residual normalized-tail omission identified after
HPHYS0216 by preserving layer-authoritative FC publication while requiring an
explicit runtime tail-contribution symbol.

1. `ProfileFCStore` publication authority is:
   `Σ(thetfc_i * dg_i) * 1000 + wb13_profile_fc_tail_mm`.
2. `wb13_profile_fc_tail_mm` must represent normalized-profile residual depth
   not covered by parser-layer aggregation and must be runtime-owned.
3. `wb13_profile_fc_store_mm` remains a reconciliation/diagnostic profile
   storage surface; it is not direct publication authority, but must reconcile
   with the combined FC publication authority above.
4. Missing/non-finite/negative `wb13_profile_fc_tail_mm` is a typed fail-closed
   WB13 publication-domain violation.
5. Required WB13 profile ordering remains:
   `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.

### HPHYS0203 Physics-Robustness Validation Addendum

1. Contract-derived robustness vectors for WB13 publication must explicitly
   cover targeted hydrology families:
   - profile family (`ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`,
     `ProfileWPStore`),
   - soil-water aggregate family (`Total-Soil`, `SoilWaterTotal`),
   - subsurface-loss family (`latqcc`, `Dp`).
2. Robustness vectors must include all of:
   - conservation-consistent closure checks
     (`SoilWaterTotal = Total-Soil + frozwt`),
   - ordering/monotonic expectations
     (`ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`),
   - unit/domain guards for non-negative depth/flux publication magnitudes,
   - non-finite protections for required runtime publication symbols.
3. Robustness vectors must include deterministic perturbation checks at WB13
   publication assembly so small bounded input perturbations do not violate
   ordering/closure invariants and remain fail-closed on invalid domains.
4. At least one deterministic regression fixture per targeted family must be
   encoded in contract-derived tests and remain passing under canonical
   publication authority.
5. Semantic parity reruns remain diagnostic evidence and are not authority
   overrides for process-correctness closure decisions.

### HPHYS0208 FC-Threshold Coupled Residual Closure Addendum

HPHYS0208 closes the coupled WB11 threshold-lineage residual seam by requiring
baseline-authoritative WB11 seed symbols for per-layer storage initialization.

1. Required WB11 seed symbols for each layer `i` are:
   - `dg_i` via `dg_####`,
   - `thetfc_i` via `thetfc_####`,
   - `thetdr_i` via `thetdr_####`,
   - `por_i` via `por_####`,
   - `cpm_i` via `cpm_####`,
   - profile saturation control `sat`.
2. WB11 seed calculations must follow baseline-authoritative lineage
   (`scon.for` + `watbal.for`) in declared units:
   - `FCi = dg_i * (thetfc_i - thetdr_i)`,
   - `ULi = (por_i - thetdr_i) * dg_i`,
   - `sat = max(sat, thetdr_i / (por_i*cpm_i))` before layer `st(i)` seed,
   - `st(i) = (((sat * por_i) * cpm_i) - thetdr_i) * dg_i`,
   - `soilw(i) = st(i) + thetdr_i * dg_i`.
3. WB11/WB18 seed publication obligations are explicit:
   - `wb18_perc_theta_#### = st(i)`,
   - `wb18_perc_fc_#### = FCi`,
   - `wb18_perc_ul_#### = ULi`,
   - `wb11_soil_water = Σ soilw(i)`,
   - `wb11_drainable_storage = Σ max(st(i) - FCi, 0)`.
4. Missing/non-finite/domain-invalid threshold-lineage symbols
   (`sat`, `por_####`, `cpm_####`, `thetfc_####`, `thetdr_####`, `dg_####`)
   are typed hard-fail WB11 seed states; FC/WP surrogate reconstruction is
   prohibited.

### HPHYS0218 WB19 `drfc`-Equivalent Threshold Closure Addendum

HPHYS0218 closes WB19 threshold-lineage drift by requiring the legacy
drain-threshold relation inside lateral/drainage kernels.

1. For each layer `i`, WB19 must derive:
   `drfc_i = wb18_perc_fc_i + (1-coca_i)*dg_i`.
2. WB19 lateral/drainage saturated-zone checks and top-down/tile withdrawals
   must use `drfc_i` (not `wb18_perc_fc_i` alone) for:
   - saturated-thickness classification,
   - drainable-pool calculations,
   - lateral (`q`) and drainage (`Qdd`) realized withdrawal updates.
3. Required domain for `coca_i` via `coca_####` is `0 < coca_i <= 1`.
4. Missing/non-finite/domain-invalid `coca_####` is a typed hard-fail WB19
   execution state; no fallback to FC-only thresholds is permitted.

### HPHYS0221 WB19 Water-Yield + Saturated-Depth Coupling Addendum

HPHYS0221 closes missing WB19 branch/coupling surfaces from baseline
`watbal.for` for lateral partition and saturated-depth evolution.

1. WB19 lateral execution originally carried provisional `solwpv` selector
   wording, but HPHYS0247 supersedes lateral-active layer selection with
   `SC-SUBHYD-001#INV-SUBHYD-024` baseline `meblfc` authority.
2. WB19 must compute coupled water-yield terms over the active saturated block:
   - `avpora = Σ(por_i * dg_i / fcdep)`
   - `avfca = Σ(thetfc_i * dg_i / fcdep)`
   - `avcoca = Σ(coca_i * dg_i / fcdep)`
   - `watyld = avpora - (avfca + (1-avcoca))`
3. For `solwpv < 2006` and `fcdep > 0`, WB19 must update saturated depth:
   - `fcdep = max(fcdep - (q / watyld), 0)`
   - `unsdep = soldep - fcdep`
4. WB19 lateral phase must publish coupled state surfaces:
   - `wb19_fcdep`
   - `wb19_unsdep`
   - `wb19_watyld`
5. Missing/non-finite/domain-invalid coupling domains (`solwpv`, `por_####`,
   `watyld` when required) are typed hard-fail WB19 states; no silent fallback
   branch is permitted.

### HPHYS0222 WB19 `solwpv` Branch-Authority Correction Addendum

HPHYS0222 resolves the WB19 saturated-depth branch-authority mismatch against
baseline `watbal.for` by tightening `fcdep` mutation scope.

1. WB19 lateral saturated-layer classification is superseded by HPHYS0247
   `SC-SUBHYD-001#INV-SUBHYD-024` baseline `meblfc` authority.
2. WB19 saturated-depth mutation (`fcdep`, `unsdep`) is authorized only for
   `solwpv < 2006`.
3. For `solwpv >= 2006` (including disturbed-soil modes `9001+`), WB19 must
   not apply `fcdep = fcdep - q/watyld`; `fcdep` remains the saturated-depth
   thickness implied by the selected saturated block for that step.
4. External-authority suite `cas_l3_subhyd_solwpv_fcdep_branch_001` is
   periodic/investigation legacy-conformance evidence for this branch law under
   `INV-WATBAL-009` and `SC-SUBHYD-001#INV-SUBHYD-015`; it is non-blocking
   pending independent constitutive authority.

### HPHYS0224 WB19 Realized-Withdrawal Soil-Water Cap Addendum

HPHYS0224 closes a WB19 process-authority gap where soil-water subtraction could
be silently floored after lateral/drainage withdrawal.

1. WB19 lateral (`q`) and drainage (`Qdd`) realized withdrawals are bounded by
   pre-phase `wb11_soil_water`.
2. WB19 soil-water updates must use explicit subtraction without clamp fallback:
   - lateral: `soil_water_after = soil_water_before - q`,
   - drainage: `soil_water_after = soil_water_before - Qdd`.
3. If `q` or `Qdd` exceeds pre-phase `wb11_soil_water`, execution must emit a
   typed hard-fail domain violation (`HKERNEL-WB11-LAT-E-003` or
   `HKERNEL-WB11-DRAIN-E-003`), not silent floor-to-zero behavior.
4. This law is governed by required Level-4 constitutive suite
   `cas_l4_subhyd_withdrawal_soilwater_cap_001` linked to
   `SC-SUBHYD-001#INV-SUBHYD-016`.

### HPHYS0225 WB19 Layer-Pool Available-Cap Authority Addendum

HPHYS0225 removes WB19 legacy available-pool reconciliation and re-anchors
authority on active per-layer state.

1. WB19 lateral/drainage available-pool cap authority is
   `layer_pool = Σ max(theta_i - drfc_i, 0)`.
2. `wb11_drainable_storage` is a compatibility seam symbol and must not expand
   available-pool caps for `q` or `Qdd`.
3. Runtime expressions of the form
   `available_pool = max(layer_pool, legacy_term)` are prohibited.
4. This law is governed by required Level-4 constitutive suite
   `cas_l4_subhyd_layer_pool_withdrawal_cap_001` linked to
   `SC-SUBHYD-001#INV-SUBHYD-017`.

### HPHYS0226 WB19 Lateral Saturated-Thickness Response Addendum

HPHYS0226 starts constitutive re-derivation closure for remaining coupled
residual families by enforcing a behavioral WB19 lateral law.

1. Under fixed geometry/conductivity/forcing and fixed `solwpv` branch, an
   increase in saturated thickness (and corresponding layer-derived available
   pool) must not decrease realized lateral flux.
2. This behavior is evaluated by required paired component vectors under
   `cas_l4_subhyd_lateral_saturated_thickness_response_001`.
3. This law is linked to `SC-SUBHYD-001#INV-SUBHYD-018` and is
   required/hard-fail in release lanes.

### HPHYS0227 WB19 FC/WP + COCA Water-Yield Coupling Addendum

HPHYS0227 closes the FC/WP theta-lineage gap in WB19 coupling and hardens
per-layer FC/WP consistency for `watyld`/`fcdep` mutation.

1. WB19 water-yield coupling must compute:
   - `avpora = Σ(por_i * dg_i / fcdep)`
   - `avfca = Σ(thetfc_i * dg_i / fcdep)`
   - `avcoca = Σ(coca_i * dg_i / fcdep)`
   - `watyld = avpora - (avfca + (1-avcoca))`
2. WB19 lateral execution must enforce per-layer FC/WP consistency:
   - `wb18_perc_fc_i = (thetfc_i - thetdr_i) * dg_i`
   with typed hard-fail posture on mismatch.
3. This law is linked to `SC-SUBHYD-001#INV-SUBHYD-019` and is governed by
   required Level-4 constitutive suite
   `cas_l4_subhyd_watyld_fcwp_consistency_001`.

### HPHYS0234 WB13 Subsurface Flux-Authority Anti-Shadow Addendum

HPHYS0234 closes a WB13 subsurface publication lineage gap where stale
state-surface symbols could shadow same-name WB19 flux outputs.

1. WB13 `latqcc`, `Tile`, and `Qd` coupling evaluations must consume
   flux-authoritative `q`, `Qdd`, and `Qd` symbols when both state and flux
   surfaces are present.
2. State-surface duplicates of `q`, `Qdd`, and `Qd` may remain for seam
   continuity but are non-authoritative for WB13 daily-row subsurface
   publication when same-name flux values exist.
3. WB13 `Qd = latqcc + Tile` coupling validation must be evaluated from the
   same flux-authoritative symbol family in requirement (1).
4. Contract-derived vectors must include stale-state vs flux-conflict probes
   showing flux precedence for subsurface publication symbols.

### HPHYS0235 WB18 Hourly Iterative-Lane Reanchoring Addendum

HPHYS0235 reanchors `ui_run=1` water-balance/percolation authority to
legacy `watbal_hourly.for` execution shape.

1. Hourly lane is not divisor-only attenuation; it is a `24`-substep daily
   loop (`ui_LFtstp=24`) where percolation is re-evaluated on substep-updated
   layer water states.
2. WB18 hourly `Dp` publication must reflect accumulated bottom-layer seepage
   across the hourly substeps.
3. Single-pass daily percolation with only `pei/24` attenuation is
   non-authoritative for `ui_run=1` and cannot close HPHYS `Dp` parity gaps.

### HPHYS0248 WB18 Hourly Restrictive-Bottom Addendum

HPHYS0248 closes the H39 early-season `Dp`/`Pe` lineage defect left after
HPHYS0247 by making the bottom restrictive-layer branch explicit for hourly
percolation.

1. When `ui_run=1`/hourly WB18 executes at the bottom layer, baseline
   `perc.for` sets `meblfc=1`, forcing `fx=1` before bottom seepage.
2. When `ui_run=1`/hourly WB18 executes with `slflag=1`, bottom-layer seepage
   must use pinned baseline `perc.for` thickness-weighted conductivity:
   `sscz = (dg_i + ui_bdrkth) / (dg_i / ssc_i + ui_bdrkth / kslast)`.
3. The resulting bottom-layer `sep` is attenuated by `ui_LFtstp` in `purk`
   before mutating `st`, and `watbal_hourly` accumulates `deepSeep` from the
   remembered bottom `sep`.
4. H39 early-season semantic evidence must report `Dp`/`Pe` residuals before
   and after this branch is active; full H39 closure remains non-promotable
   while WB17/snowmelt residual families remain materially unresolved.

### HPHYS0209 ProfileWP Near-Closed Adjudication Addendum

HPHYS0209 isolates the near-closed `ProfileWPStore` residual lane and codifies
its adjudication posture without changing publication authority.

1. `ProfileWPStore` publication authority remains
   `wb13_profile_wp_store_mm` under HPHYS0207; no surrogate
   reconstruction/reprojection formula is authorized for adjudication closure.
2. WB13 publication must continue direct runtime projection from
   `wb13_profile_wp_store_mm` with existing typed fail-closed guards for
   missing/non-finite/domain-invalid symbols.
3. A residual lane limited to isolated hillslope cases with stable sign and
   magnitude may be classified as expected process-correct diagnostic evidence
   only when:
   - `ProfileDepth` and `ProfilePorosityCap` remain non-regressing, and
   - no new WB13 profile-ordering violations are introduced.
4. Residual spread beyond the isolated lane, profile-ordering regressions, or
   profile-depth/capacity regressions must be treated as unresolved defect
   lineage and retain `HOLD` posture.

### AUTH03 Level-4 Constitutive Gate Bootstrap Addendum

AUTH03 introduces blocking external-authority constitutive suites for FC/WP
and near-FC percolation threshold adjudication independent of legacy parity
signals.

1. Level-4 required suites for this contract family are:
   - `cas_l4_watbal_relax_to_fc_001`
   - `cas_l4_soil_fc_minus33_001`
   - `cas_l4_soil_wp_minus1500_001`
2. Canonical invariant linkage for `cas_l4_watbal_relax_to_fc_001` is:
   - `SC-WATBAL-001#INV-WATBAL-006`
3. `theta <= fc` branches must fail-open to zero percolation flux
   (`pei = 0`, `D >= 0`) and `theta > fc` branches must remain positive-only.
4. Missing/non-finite/domain-invalid constitutive symbols used by WB18
   percolation (`wb18_perc_theta_####`, `wb18_perc_fc_####`,
   `wb18_perc_ul_####`, `wb18_perc_ssc_####`) are typed fail-closed states.
5. AUTH03 suite failures are blocking (`gate_lane=required`,
   `failure_class=hard-fail`) and keep disposition in `HOLD` until resolved.
6. Suite registry/metadata authority is canonicalized in:
   - `docs/specifications/external-authority/registry.yaml`
   - `docs/specifications/external-authority/suites/`

## MOFE04 Multi-OFE WB13/WAT Publication Policy Addendum

1. WB13/H.wat publication policy for hillslope MOFE contexts is explicit and
   canonicalized as single-row daily publication with `OFE = 1`.
2. Published `OFE = 1` under this policy is a canonical row id, not a claim
   that contributor topology cardinality equals one.
3. Publication provenance must carry contributor cardinality and policy
   semantics explicitly:
   - `publication_ofe_policy = "single-row-canonicalized-hillslope-aggregate"`
   - `contributor_ofe_count = slope.ofe_count`
   - `area_policy = "sum-ofe-geometry-area"`
   - `publication_area_m2 = Σ(fwidth_i * slplen_i)` over all contributing OFEs.
4. WB13 `Area` and H.wat `Area` must equal the canonicalized aggregate area
   from all contributing OFE geometries (not primary-OFE-only area) for both
   single-OFE and multi-OFE runs.
5. Missing/non-finite/non-positive contributor geometry terms or invalid
   aggregate publication area are hard-fail publication-domain violations under
   existing WB13 guard-family continuity (`HKERNEL-WB13-HWAT-E-003`).
6. `QOFE = Q` remains required under MOFE04 canonicalized policy and does not
   authorize surrogate per-OFE runoff synthesis.

### MOFE04 Contract-Test Vectors

1. Multi-OFE publication vector: aligned multi-OFE run publishes deterministic
   WB13/H.wat rows with `OFE = 1`, aggregate `Area = Σ(fwidth_i * slplen_i)`,
   and explicit MOFE04 publication provenance fields.
2. Single-OFE publication vector: single-OFE run publishes identical canonical
   policy fields with `contributor_ofe_count = 1` and aggregate area equal to
   single contributor geometry.
3. Missing/invalid contributor geometry vector hard-fails publication assembly
   with WB13 domain guard continuity and does not emit malformed rows.

## ARCH22 Typed Production-Surface Addendum

### Typed Runtime Surface Authority

1. Covered production water-balance interfaces must use typed ARCH22 symbol
   surfaces for boundary-state and boundary-flux access:
   `HillslopeProductionStateSymbol` and `HillslopeProductionFluxSymbol`.
2. Covered production guard/accessor helper signatures must not accept raw
   `&str` symbol identifiers where typed ARCH22 symbols exist.
3. Typed migration must preserve WB11/WB12/WB14/WB15/WB16 typed hard-fail
   posture and message-ID continuity for missing/non-finite/domain failures.

### Contract-Derived Migration Vectors

1. Static migration proof: covered WB11/WB12/WB14/WB15/WB16 production guard
   accessors use typed symbol families, not stringly `&str` parameters.
2. Nominal migration vector: canonical hydrology lane execution preserves
   deterministic state/flux publications under typed symbol resolution.
3. Failure migration vectors: missing/non-finite/domain-invalid symbols retain
   existing typed boundary classes and guard IDs.

## SIMIMPL03 Production Execution Ownership and Intake Guardrail Addendum

### Required Execution Ownership Closure

1. Production watbal boundary publication is execution-owned: required
   hydrology surfaces must originate from accepted scheduler/kernel lane
   execution and carry lane provenance metadata.
2. Projection-first helpers are non-authoritative for required candidate
   surfaces once production execution ownership is claimed.
3. Required publication provenance minimum:
   - lane identity (`daily` or `hourly`);
   - effective mode source (`wepp_ui.mode.ui_run`);
   - execution result (`accepted`/typed failure code).

### Required Mode-Propagation Closure

1. Parser-owned `wepp_ui` effective mode is immutable runtime input to lane
   selection.
2. Daily lane is the only valid lane for `ui_run=0`; hourly lane is the only
   valid lane for `ui_run=1`.
3. Lane selection may not silently collapse to daily on missing/invalid mode
   closure.

### Consolidated Intake Guardrails

1. Consolidated intake from `/workdir/wepp-forest/fpm-src` is selective and
   must be provenance-triaged per kernel/policy family.
2. qcap-style clamp/policy overlays are non-authoritative until explicitly
   triaged and dispositioned under canonical contract governance.
3. Intake guardrails are governance blockers for runtime enablement claims
   until explicit triage evidence exists.

## EROD13 Wave-1 Active Producer-Coupling Addendum

1. When `erod13_core_enabled = 1`, water-balance owned runoff/peak-duration
   coupling surfaces (`Q`, `peakro`, `watdur`, and WB16 branch diagnostics) are
   required erosion-core ingress payloads and must retain finite domain-valid
   values.
2. Producer ownership remains in `SC-WATBAL-001` and `SC-RUNOFFPART-001`;
   erosion-core consumer guard ownership is enforced in `SC-SED-001` through
   `HKERNEL-EROD13-CORE-E-001..003`.
3. Enabled-path missing/non-finite/domain-invalid runoff coupling surfaces must
   hard-fail; no fallback reconstruction of erosion-core forcing symbols is
   allowed.

## EROD14 Wave-2 Active Producer-Coupling Addendum

1. When `erod14_wave2_enabled = 1`, water-balance-owned runoff/runon closure
   exports required by Wave-2 (`erod14_qout`, `erod14_qin`) must remain
   finite, domain-valid, and provenance-consistent with WB12/WB16 closure
   surfaces.
2. Producer ownership remains in `SC-WATBAL-001` and `SC-RUNOFFPART-001`;
   Wave-2 consumer guard ownership is enforced by `SC-SED-001` under
   `HKERNEL-EROD14-WAVE2-E-001..003`.
3. Enabled-path missing/non-finite/domain-invalid runoff coupling exports must
   hard-fail; no fallback reconstruction or silent substitution is allowed.
4. Wave-2 producer-coupling adds to, and does not replace, the existing
   Wave-1 coupling requirements.

## SIMIMPL14 Continuous Runner and WB13 Span/Key Addendum

1. Continuous hillslope execution must iterate all available climate daily
   forcing rows in deterministic order and execute one scheduler/kernel cycle
   per day with carried runtime state between days.
2. WB13/H.wat publication must emit exactly one row per executed day with
   monotonic `sim_day_index` starting at `1`.
3. Published WB13 key tuple must preserve canonical comparator ordering
   semantics `(Y, J, OFE)` where `Y` is simulation-year ordinal
   (`calendar_year - start_year + 1`), not absolute calendar year.
4. Publication provenance must include continuity assertions at minimum:
   executed day count, published row count, first row key, last row key, and
   monotonic-key verdict.
5. Missing continuity assertions, span collapse, or key-domain mismatch are
   typed hard-fail states under `HS-SIMOUT-E-001`.

## SIMIMPL15 Replay Comparator Tooling Alignment Addendum

1. Replay provenance must publish explicit lane-policy mode keyed by candidate
   surface format:
   - `.dat` -> strict lane is required;
   - `.parquet` -> strict-equivalent semantic lane is required with explicit
     strict-skip rationale for raw comparator incompatibility.
2. Replay provenance must classify candidate surface source as one of:
   `native-runtime-dat`, `conversion-derived-dat`, or
   `native-runtime-parquet`.
3. Conversion-derived dat strict comparator evidence remains valid for
   diagnostics, but is non-promotable for final Tier-A closeout claims.
4. Semantic comparator lane must canonicalize investigation-column aliases so
   `Total-Soil` is preserved when candidate parquet uses either
   `Total-Soil` or legacy `Total-Soil Water` field names.
5. Semantic comparator width diagnostics for parquet lanes must publish
   observed row field-count classes; sentinel placeholder widths are invalid.

## SIMIMPL16 Replay Contract-Derived Test-Coverage Closure Addendum

1. Replay promotability governance must fail closed when contract-derived tests
   do not assert span overlap and row-key domain comparability for promoted
   lanes.
2. Key-domain closure tests must explicitly guard simulation-year `Y` semantics
   and reject calendar-year keyed replay promotion claims.
3. Strict-lane governance tests must enforce compensation requirements when raw
   strict comparator execution is skipped for parquet lanes.
4. Conversion-derived dat evidence must include explicit row-consistency checks
   against baseline replay spans before it can be considered promotable.
5. Alias continuity tests must preserve `Total-Soil` investigation lineage
   across accepted parquet alias forms without regressing required diagnostics.

## SIMIMPL18 Rain/Snow Partition and Storage-State Mutation Closure Addendum

1. WB13 day-key publication must treat `RM` as runtime liquid input
   (`rain + melt`) and must not publish direct precipitation passthrough under
   cold all-snow partition branches.
2. `Snow-Water` publication aliases must be runtime-state-derived from
   `snow.runtime_swe`; publishing static sidecar control (`snow.options.ssd`)
   as dynamic snow storage is invalid.
3. Hydout-equivalent storage publication (`Total-Soil`, `frozwt`,
   `Snow-Water`, `SoilWaterTotal`) must originate from mutable runtime state
   surfaces and preserve day-to-day mutation under varying forcing.
4. Baseline/candidate parity closure artifacts must include explicit first-day
   partition diagnostics (`P`, `RM`, `Snow-Water`, `Total-Soil`, `frozwt`,
   `SoilWaterTotal`) plus multi-day storage mutation diagnostics to prevent
   re-introducing static-parameter publication leakage.

## SIMIMPL21 WB11 ET/Soil-Water Baseline Ordering and Lineage Addendum

1. Canonical WB11 authority preserves baseline daily sequencing:
   `purk` percolation call, ET partition/stage-memory update (`evap`/`evappm`),
   drainage + lateral mutations, root-uptake extraction (`swu`), then aggregate
   root-zone recomputation (`watcon`).
2. Canonical ET-root uptake closure requires `swu` execution after drainage and
   lateral mutations when transpiration demand and root depth are both
   positive; moving this extraction earlier is non-authoritative.
3. Aggregate soil-water lineage authority is layer-first:
   `st(i)` updates feed `soilw(i)` and then `watcon`; WB13
   `Total-Soil`/`SoilWaterTotal` publications must trace to this lineage.
4. SIMIMPL22 contract-derived tests must include ordering and lineage vectors
   proving these assertions before SIMIMPL23 production ET migration claims are
   promotable.

## HPHYS0238 WB19 Hourly Iterative Lateral/Drainage Addendum

1. Hourly-lane WB19 execution authority requires iterative substeps in both
   lateral (`run_lateral_transfer`) and drainage (`run_drainage`) kernels.
2. Runner seeding must publish `wb19_lateral_drain_lane_substeps` for active
   lane mode (`1` daily, `24` hourly).
3. Lateral/drainage kernels must recompute state-dependent flux drivers each
   substep from mutated layer state and accumulate realized daily `q`/`Qdd`.
4. Divider-only single-pass substitutions for hourly behavior are not
   authoritative and cannot satisfy hourly closure claims.
5. Contract-derived tests must include daily-vs-hourly lane vectors proving
   non-collapsed behavior under identical forcing/state domains.

## HPHYS0239 WB19->WB12->WB13 Ordering and Flux-Authority Handoff Addendum

1. Canonical promoted hydrology-tail ordering is explicit:
   `PercolationDeepSeepage -> Evapotranspiration -> WB19 subsurface handoff ->
   RunoffReconciliation -> StorageReconciliation`.
   HPHYS0242 refines the hourly-lane WB19 handoff as
   `Drainage -> LateralTransfer` to match baseline `watbal_hourly.for`;
   older lateral-before-drainage wording is not authoritative for hourly lanes.
2. WB12 runoff/storage reconciliation must consume post-WB19 handoff surfaces
   from the same daily execution pass (`Q`, `D`, `Qd`, `ET`) and must not read
   stale pre-WB19 values.
3. WB13 publication symbols `Q`, `Ep`, `Es`, and `Er` are flux-authoritative
   under state/flux symbol conflicts; state duplicates may remain for seam
   continuity but are non-authoritative when same-name flux values exist.
4. Contract-derived vectors must include stale-state/flux-conflict probes for
   `Q`/`Ep`/`Es`/`Er` plus canonical-order checks for the full WB19->WB12 tail.

## HPHYS0240 Hourly Runoff Carryover Addendum

1. WB12/WB14 runoff reconciliation must resolve incoming runoff carryover from
   `wb12_runoff_carryover` when that same-pass flux is present.
2. `wb12_runon_input` remains a compatibility state surface only when the
   same-pass carryover flux is absent; it is non-authoritative under carryover
   flux/state conflicts.
3. The resolved carryover value must be finite, non-negative, used in the `Q`
   and closure-delta equations, and republished as `wb12_runoff_carryover`.
4. Missing/malformed present carryover fluxes are typed hard-fail states; they
   must not be silently defaulted or replaced by stale state.

## HPHYS0241 MOFE Hourly Carry-Array Routing Continuity Addendum

1. MOFE hourly carry-array authority is the baseline `wathour.inc` common block
   with fixed `ui_LFtstp = 24` for `ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`, and
   `ui_LfCrf`.
2. At the first OFE, upstream and current carry arrays initialize to zero. At
   later OFEs, current arrays initialize to zero while upstream arrays retain
   the prior OFE's copied current arrays.
3. Hourly water input to the current OFE uses the explicit array expression
   `xfin = fin/24 + (ui_LfUrf(ii) + ui_SUrunf(ii)) * Aupstream/Acurrent`
   before infiltration/water-balance mutation for hour `ii`.
4. `ui_LfCrf(ii)` is the realized lateral-flow amount after layer-withdrawal
   caps, not the unconstrained potential target. `ui_SCrunf(ii)` is the
   realized top-layer saturation-excess amount from the `st(1) - fzul` branch.
5. After the hourly loop, copy-forward must preserve baseline lineage:
   `ui_SUrunf(ii) = ui_SCrunf(ii)` and
   `ui_LfUrf(ii) = ui_LfCrf(ii)` for `ii = 1..24`.
6. Runtime surfaces for MOFE hourly lanes must publish all 24 scalar entries for
   each array family. Missing entries, non-finite entries, negative entries,
   or cardinality other than 24 are typed hard failures.
7. Daily aggregate carry surfaces such as `wb12_runoff_carryover` may summarize
   the explicit `ui_SUrunf + ui_LfUrf` array payload for WB12/WB13 closure, but
   may not replace or synthesize the array payload in MOFE hourly lanes.

## HPHYS0242 WB14/WB12 Hourly Cadence and Ordering Addendum

1. Baseline hourly water-balance ordering for the WB14/WB12 tail is
   authoritative from `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
   at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`: percolation runs
   hourly, ET runs only after the final hourly percolation pass, drainage runs
   before lateral flow in the hourly tail, and storage/runoff closure observes
   the same-pass mutated states.
2. Positive top-layer saturation excess after the hourly WB19 tail must be
   clipped from `st(1)` into `ui_SCrunf(ii)` before runoff publication; this is
   a production invariant, not a diagnostic-only carry surface.
3. Daily `Q` published by WB14/WB12 must include both partition runoff and
   `surdra = Σui_SCrunf(ii)`, and the closure residual must use the same value
   that is published.
4. WB12 storage reconciliation must consume same-pass `Q`, `ET`, `D`, and `Qd`
   values. Compatibility state surfaces are non-authoritative when same-pass
   fluxes exist.
5. Contract-derived tests must prove scheduler order, `ui_SCrunf` clipping and
   addback, ET/infiltration lineage freshness, and stale-state anti-shadow
   behavior before production edits are promotable.

### HPHYS0246 WB18 Aggregate Soil-Water Writeback Addendum

HPHYS0246 closes the WB18 aggregate writeback half of `INV-WATBAL-029` for the
percolation boundary:

1. WB18 `percolation_deep_seepage` must not publish `Total-Soil` lineage from
   `Σwb18_perc_theta_####` alone.
2. The WB18 producer obligation is to publish `wb11_soil_water` from
   `SC-PERC-001#INV-PERC-013`, which maps baseline `watcon = Σsoilw(i)` to
   runtime symbols.
3. WB13 `Total-Soil` and `SoilWaterTotal` publication must continue to consume
   `wb11_soil_water`; after HPHYS0246, WB13 remains a downstream reflection of
   WB18/WB19 aggregate state rather than a compensating publication layer.

### HPHYS0249 WB17 Aggregate Storage Coupling Addendum

HPHYS0249 closes the WB17 half of `INV-WATBAL-029` for ET-driven aggregate
storage:

1. WB17 must treat `SC-EVAP-001#INV-EVAP-015` layer mutation as the
   authoritative source for `Ep`/`Es` storage effects, with `Es` mutation in
   the ET phase and `Ep`/`swu` mutation after WB19 drainage/lateral execution.
2. `wb11_soil_water` must be recomputed after WB17 soil evaporation and after
   post-WB19 root uptake from
   `wb18_perc_theta_####`, `thetdr_####`, `dg_####`, and optional frozen-depth
   surfaces using baseline `watcon = Σsoilw(i)` semantics.
3. WB13 `Total-Soil` and `SoilWaterTotal` must consume this post-WB17
   aggregate. Publication from pre-ET aggregate state, scalar-only ET
   decrement, or publication-layer compensation is invalid closure evidence.
4. Full-suite continuation metrics for `Ep`, `Es`, `Snow-Water`, `RM`, `Q`,
   `Total-Soil`, and `SoilWaterTotal` must be recorded before promoting H39
   hourly water-balance closure beyond `HOLD`.

### HPHYS0250 WB13 Final-Ep Coupling Addendum

HPHYS0250 closes the WB13 `Ep` publication half of
`SC-EVAP-001#INV-EVAP-016`:

1. Water-balance execution must preserve management-derived PL activation
   surfaces through scheduler dispatch so growth can produce active root-depth
   state before hydrology phases.
2. WB13 `Ep` publication must consume final post-WB19 root-uptake flux, not the
   pre-root-uptake ET-phase seed or any same-name stale state alias.
3. Full-suite continuation metrics must report whether corrected final `Ep`
   lineage reduces `Ep`, `Total-Soil`, and `SoilWaterTotal` residuals before
   any follow-on package reprioritization.

### HPHYS0251 SWU Uptake Magnitude Coupling Addendum

HPHYS0251 closes the WB17/WB13 storage-coupling half of
`SC-EVAP-001#INV-EVAP-017`:

1. WB17 root uptake must consume management-derived raw `pltol` and publish the
   normalized effective tolerance used by the baseline `swu.for` water-stress
   branch.
2. WB17 root uptake must publish layer `UPi_####` and `Ui_####` traces so
   aggregate `UPi`, aggregate `Ui`, final `Ep`, and final `Ws` are auditable
   from layer state.
3. `wb11_soil_water` must be recomputed after actual `Ui_####` has mutated
   `wb18_perc_theta_####`; WB13 `Total-Soil` and `SoilWaterTotal` must consume
   this post-uptake aggregate.
4. H1/H13/H39 diagnostics and the full 39 hillslope suite must be recorded
   against HPHYS0250 continuation metrics before any claim that the uptake
   magnitude residual family has materially improved.

### HPHYS0252 WB19 Storage-Availability Continuation Addendum

HPHYS0252 continues the HPHYS0251 HOLD recommendation by moving upstream from
SWU magnitude to WB19 layer-storage availability before WB17 root uptake.

1. WB19 lateral storage availability must consume
   `SC-SUBHYD-001#INV-SUBHYD-025`: capacity caps and withdrawal floors are
   assembled with `fzdrfc(i) = max(drfc(i)-frzw(i),0)`, while hourly
   conductivity remains governed by unfrozen `drfc(i)` `fffx` weighting.
2. Post-WB19 layer storage (`wb18_perc_theta_####`) and aggregate
   `wb11_soil_water`/`watcon` must remain the authoritative storage input to
   `PlantRootUptake`; WB17 must not consume pre-WB19 shadow state when same-day
   WB19 lateral/drainage writes are present.
3. WB13 `Total-Soil` and `SoilWaterTotal` publication must consume the
   post-WB19/post-WB17 aggregate storage lineage, not compensating publication
   arithmetic.
4. Comparator disposition must report targeted H1/H13/H39 storage-availability
   diagnostics and the full 39-hillslope semantic suite against HPHYS0251
   continuation metrics before any closure claim.
5. If the baseline-authoritative WB19 `fzdrfc` correction is process-correct
   but does not materially improve H39 residuals, disposition remains `HOLD`
   with the next focus selected from observed upstream storage lineage evidence,
   not from heuristic tuning.

### HPHYS0254 WB11 Initial-Storage Projection Addendum

HPHYS0254 continues the HPHYS0253/HPHYS0252 storage residual lineage by moving
to the pre-scheduler WB11 seed grid.

1. WB11 initial storage projection must publish hydrology seed aliases on the
   baseline-normalized corrected layer grid used by profile depth/capacity
   lineage, not the parser-row grid when normalized tail depth is present.
2. `wb11_nsl`, `wb19_dg_####`, and `wb19_solthk_####` for the WB11 runtime hydrology
   surface must span `wb13_profile_depth_mm`; `Σ(wb19_dg_####)*1000` must
   reconcile to `wb13_profile_depth_mm` before seeding `st(i)`/`soilw(i)`.
3. The hydrology threshold/conductivity family (`wb19_thetfc_####`,
   `wb19_thetdr_####`, `wb19_por_####`, `cpm_####`, `wb19_coca_####`,
   `ssc_####`, and WB18 threshold/store aliases) must share that same
   normalized layer cardinality and depth grid.
4. Generic constitutive `thetfc_####`/`thetdr_####` remain governed by
   AUTH03/AUTH05 corrected-parser-layer authority and cannot be repurposed to
   satisfy normalized WB11 hydrology seeding.
5. `wb11_soil_water` must be seeded from layer-authoritative storage
   (`st(i) + thetdr(i)*dg(i)` under unfrozen initial conditions), not by adding a
   scalar tail or publication-side compensation outside the layer state.
6. Missing, non-finite, or incomplete normalized-grid inputs are typed
   fail-closed WB11 seed states and do not authorize parser-depth fallback.

### HPHYS0255 MOFE Storage Projection Addendum

HPHYS0255 clarifies the interaction between MOFE04 aggregate output geometry
and HPHYS0254/WB13 storage lineage.

1. MOFE04 aggregate `Area` publication does not by itself authorize aggregate
   storage synthesis from static per-OFE soil rows.
2. In the current single WB11 hydrology-state architecture, unqualified WB11,
   WB18, WB19, and WB13 storage aliases are simulation-owned runtime-state
   surfaces; they must remain traceable to the canonical WB11 `st(i)`/`soilw(i)`
   to `watcon` lineage before `Total-Soil` and `SoilWaterTotal` publication.
3. OFE-qualified soil symbols (`ofeN_*`) are valid contributor diagnostics and
   parser/runtime provenance. They are not dynamic WB11 hydrology states unless
   a future per-OFE hydrology-state contract and implementation explicitly
   promote them.
4. WB13/H.wat publication provenance for MOFE contexts must declare a storage
   lineage policy. For the current architecture the policy is
   `single-runtime-wb11-state`; downstream consumers must not infer
   area-weighted dynamic storage from aggregate `Area`.
5. A future MOFE dynamic aggregate-storage closure package must amend canonical
   contracts with pinned baseline provenance, define the per-OFE state vector
   and aggregation operator, and add tests that fail if storage is merely
   reconstructed from static soil rows.

### HPHYS0256 WB19 Latqcc Lane-Branch Addendum

HPHYS0256 ties daily `latqcc` publication to WB19 lateral lane provenance.

1. Daily `latqcc` closure claims require `SC-SUBHYD-001#INV-SUBHYD-026`
   daily lateral authority when `wb19_lateral_drain_lane_substeps = 1`.
2. Hourly `latqcc` closure claims retain `SC-SUBHYD-001#INV-SUBHYD-024` and
   `SC-SUBHYD-001#INV-SUBHYD-025` authority when
   `wb19_lateral_drain_lane_substeps = 24`.
3. WB13 publication evidence must preserve lane provenance for `latqcc` so
   residual interpretation does not mix daily and hourly lateral laws.
4. Full `H1..H39` continuation metrics must report whether `latqcc` residual
   movement came from daily lane correction, hourly lane correction, or neither.

### HPHYS0258 WB19 Realized Lateral Publication Addendum

HPHYS0258 makes WB19 hourly cap/withdrawal lineage observable for WB13 closure
claims.

1. Hourly `latqcc`/`Qd` publication must consume realized WB19 lateral
   withdrawal, not uncapped potential or capped-but-unwithdrawn targets.
2. Runtime diagnostics must expose potential, capped target, active-layer
   `tdvv`, active-layer counts, and per-layer withdrawal so `latqcc` residuals
   can be assigned to WB19 cap, publication, or downstream storage lineage.
3. WB13 aggregate storage continuation evidence must reconcile post-WB19 layer
   storage to realized withdrawal before `Total-Soil`/`SoilWaterTotal`
   interpretation.
4. Missing potential/target/realized distinction is insufficient evidence for
   H39 hourly WB19 closure even when daily `q` is finite and non-negative.

### HPHYS0259 WB19 Trace Residual Localization Addendum

HPHYS0259 requires trace-grade WB19 evidence before assigning remaining
H1/H7/H39 `latqcc` residuals to either WB19 internals or downstream water
balance publication/storage.

1. H1/H7/H39 classification reports must consume trace rows carrying WB19
   potential, target, `tdvv`, unrealized residual, per-layer withdrawal, `q`,
   `Qdd`, and `Qd` from the post-lateral-transfer surface.
2. If `q`, per-layer withdrawal, and `Qd` identities reconcile internally,
   WB13 residual interpretation cannot reopen WB19 cap/publication logic
   without new baseline-authoritative evidence.
3. Under closed WB19 identities, continuation focus moves to WB17 `Ep`, WB18
   `Dp`, and final `Total-Soil`/`SoilWaterTotal` reconciliation.

### HPHYS0260 WB17/WB18/Storage Trace Residual Localization Addendum

HPHYS0260 requires trace-grade WB17/WB18/final-storage evidence before
assigning post-HPHYS0259 residual ownership to publication or shadowing.

1. H1/H7/H39 classification reports must consume trace rows carrying WB17
   aggregate/layer `UPi` and `Ui`, final `Ep`, `Etp`, `Ws`, WB18 aggregate
   `D`/`Pe`, per-layer `pei`, post-mutation `st`, `thetdr`, `dg`, optional
   frozen depth, recomputed aggregate `watcon`, and WB13 `Total-Soil` plus
   `SoilWaterTotal`.
2. WB17 identities close when `Ep = ΣUi_####`, aggregate `Ui = ΣUi_####`,
   aggregate `UPi = ΣUPi_####`, `0 <= Ui_#### <= UPi_####`, and
   `Ws = Ep/Etp` when `Etp > 0`.
3. WB18/storage identities close when `D = Pe` for bottom export and traced
   `wb11_soil_water` equals
   `Σ(wb18_perc_theta_i + wb19_thetdr_i*(wb19_dg_i - frozen_i))`.
4. If these identities close and residuals persist, continuation remains
   `HOLD` and should target baseline-authoritative magnitude/initialization
   lineage rather than heuristic storage compensation.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-WATBAL-001 | Per-invariant comparator vectors for `INV-WATBAL-*` families remain uncurated, and this residual automation limitation is explicitly risk-accepted for current governance progression. | Automated per-invariant acceptance remains limited; manual comparator interpretation is required where vectors are absent. | closed | `[DIRECT][Static]` |
| GAP-WATBAL-002 | Companion contracts (`SC-RUNOFFPART-001`, `SC-EVAP-001`, `SC-PERC-001`, `SC-SUBHYD-001`) are authored but retain open implementation-promotability gaps for full WB11 ET/soil-water runtime closure. | Cross-contract ownership is explicit, but promotable runtime closure remains provisional pending SIMIMPL22/SIMIMPL23 execution. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-WATBAL-003 | Wave-0 erosion-lane alias-ownership ambiguity for required runoff/peak-duration boundary symbols is explicitly dispositioned by canonical EROD11 alias ownership registers. | Alias-ownership ambiguity closure is complete for required boundary symbols; production erosion physics remains separately `HOLD`-gated by non-promotable companion/process gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-WATBAL-004 | Chapter-5 validation caveat (stronger near-surface than full-profile agreement) remains and is explicitly retained as a documented limitation with governance risk acceptance. | Deep-profile closure confidence remains lower than near-surface Tier-A signals and requires explicit interpretation in governance decisions; this is accepted as a model-governance limitation. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-WATBAL-005 | WB16 baseline-authoritative `ealpha` producer chain (`frcfac -> rdat(alpha) -> alphay -> eplane`) is now implemented in production runtime surfaces for runtime-projection-complete lanes, with explicit runtime/compatibility provenance policy. | Producer-chain migration closure is complete for scoped runtime lanes; compatibility branch remains explicitly non-promotable and warning-gated when required producer symbols are absent. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-WATBAL-006 | HPHYS0260 adds trace-grade WB17/WB18/final-storage residual classification authority but does not itself change hydrology physics. | Full water-balance parity remains `HOLD` when identities close but H1..H39 semantic residuals persist; follow-on work must target baseline-authoritative magnitude or initialization lineage. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-06-05` | `136` | `Codex` | HPHYS0313 correction: branch-gated the settling-route water-balance lineage and reclassified the material 2013 day 11 hour 11 route from no-snow `driftg` to positive-`hrsnow` hourly snowfall input lineage. |
| `2026-06-05` | `135` | `Codex` | HPHYS0313 amendment: added split-route snowpack settling/carry recursion water-balance authority (`INV-WATBAL-086`) before downstream ownership or compensation. |
| `2026-06-05` | `134` | `Codex` | HPHYS0312 amendment: added prior-year terminal snowpack lineage water-balance authority (`INV-WATBAL-085`) before downstream ownership or compensation. |
| `2026-06-05` | `133` | `Codex` | HPHYS0311 amendment: added snow carry source-line water-balance authority (`INV-WATBAL-084`) before downstream ownership or compensation. |
| `2026-06-05` | `132` | `Codex` | HPHYS0310 amendment: added prior-day snow carry divergence water-balance ledger authority (`INV-WATBAL-083`) before downstream ownership or compensation. |
| `2026-06-05` | `131` | `Codex` | HPHYS0309 amendment: added snow carry/depletion water-balance ledger authority (`INV-WATBAL-082`) requiring prior-day/day-start snow-state evidence before downstream ownership or compensation. |
| `2026-06-05` | `130` | `Codex` | HPHYS0308 amendment: added branch-extra state-ordering evidence (`INV-WATBAL-081`) requiring key-level baseline/openWEPP snow-state and predicate evidence before branch-predicate, snow-producer, or downstream water-balance edits. |
| `2026-06-05` | `129` | `Codex` | HPHYS0307 amendment: added melt-call branch activation lineage (`INV-WATBAL-080`) requiring baseline `winter.for`/`snowd.for` and openWEPP branch predicate provenance before snow-producer or downstream water-balance edits. |
| `2026-06-05` | `128` | `Codex` | HPHYS0306 amendment: added branch-active melt-term observe semantics (`INV-WATBAL-079`) so inactive fixed-baseline `melt.for` hours are not zero-imputed and active-mask divergence is classified before numeric term comparison. |
| `2026-06-05` | `127` | `Codex` | HPHYS0305 amendment: added paired melt-term/state water-balance gate (`INV-WATBAL-078`) and ratified openWEPP trace aliases for rain/snow-depth/depth/density hourly maps needed before any snow or downstream compensation edit. |
| `2026-06-05` | `126` | `Codex` | HPHYS0303 ratification amendment: aligned water-balance negative-melt governance with fixed `wepp_260430` comparator commit `47ac4c32faeea81bb99081f955a14c38b815ef4d` and preserved the original `dac3c950` branch as archived bug context. |
| `2026-06-05` | `125` | `Codex` | HPHYS0302 amendment: added `INV-WATBAL-077`, requiring same-quantity/same-unit comparator-surface proof before assigning water-balance residual ownership or authorizing downstream/snow-producer edits. |
| `2026-06-05` | `124` | `Codex` | HPHYS0301 amendment: added `INV-WATBAL-076`, requiring H39 first-2013 residual rain-on-snow reconciliation against openWEPP released plus post-winter rain before any forcing, snow-producer, or downstream water-balance edit. |
| `2026-06-05` | `123` | `Codex` | HPHYS0300 Claude review disposition: added bounded evidence-gate criteria requiring an implementation-or-blocking-invariant decision once paired term/state evidence isolates a raw/post-raw producer source, with H39 first-2013 forcing kept separately actionable. |
| `2026-06-05` | `122` | `Codex` | HPHYS0300 amendment: added `INV-WATBAL-075`, requiring raw hourly melt/post-raw routed-melt lineage evidence before downstream water-balance focus changes or snow producer edits. |
| `2026-06-05` | `121` | `Codex` | HPHYS0299 amendment: added `INV-WATBAL-074`, suspending HPHYS0298 production-migration authority until corrected paired evidence compares pinned-baseline `hrsnow` depth with openWEPP snowfall-depth traces rather than water-equivalent summaries. |
| `2026-06-05` | `120` | `Codex` | HPHYS0298 Claude review disposition: clarified that all-window `hourly-forcing` residual ownership is a producer-side porting-fidelity defect against `SC-SNOWFREEZE-001#INV-SNOWFREEZE-029` and pinned baseline `winter.for:410-412`, not downstream WB17/WB18/WB19/WB13 compensation authority. |
| `2026-06-05` | `119` | `Codex` | HPHYS0298 amendment: added paired source-partition authority requiring baseline observe identity, same-HEAD full-suite metrics, ordered first-divergent cut-point evidence, source-line provenance, and final verdicts before re-tiering or downstream hydrology compensation. |
| `2026-06-05` | `118` | `Codex` | HPHYS0297 amendment: added water-balance defect-ledger authority requiring observed/reconstructed `RM`, reconstruction residuals to named tolerance, closed `Q`, producer-consumer identity, and explicit verdicts before re-tiering or downstream hydrology focus. |
| `2026-06-05` | `117` | `Codex` | HPHYS0296 review disposition: tightened water-balance acceptance so corrected-negative-melt correlation plus internal closure cannot re-tier residuals without per-window defective-model verdict, reconstruction, independent correctness adjudication, and auditable documented-legacy-defective accounting. |
| `2026-06-05` | `116` | `Codex` | HPHYS0296 amendment: added snow/`RM` semantic acceptance gate requiring corrected-negative-melt explanation or producer-migration hold, with downstream compensation prohibited. |
| `2026-06-05` | `115` | `Codex` | HPHYS0295 amendment: added cumulative row-to-row storage-budget ownership authority requiring ET/percolation/lateral/snow-mask accounting before WB17/WB18/WB19/WB13 production edits. |
| `2026-06-05` | `114` | `Codex` | HPHYS0294 amendment: added post-ingress storage/percolation/lateral attribution authority requiring WB18/WB19 trace-grade magnitude accounting and snow-excluded residual masks before production edits. |
| `2026-06-05` | `110` | `Codex` | HPHYS0291 amendment: added same-day snow publication lifecycle authority from runoff producer fluxes through WB13 and required trace evidence before assigning remaining residual ownership. |
| `2026-06-05` | `109` | `Codex` | HPHYS0290 amendment: required WB13 `RM` to consume explicit `snow.post_winter_rain_m` rather than inferring post-winter rain from raw precipitation, SWE, or snow-active state. |
| `2026-06-04` | `108` | `Codex` | HPHYS0289 amendment: superseded raw-precipitation/SWE-delta WB13 `RM` proxy wording with baseline publication authority `RM = post-winter rain + wmelt + irrigation` and `Snow-Water = runtime snowpack storage`. |
| `2026-06-04` | `107` | `Codex` | HPHYS0288 amendment: added rain-on-snow `RM`/storage-forcing authority requiring residual rain released from snowpack holding capacity to flow through `hrmlt`/`wmelt` and be excluded from direct-rain double counting. |
| `2026-06-04` | `106` | `Codex` | HPHYS0287 amendment: added fail-closed runtime snow-state guard to WB12/WB14 liquid partition and WB13 storage publication closure. |
| `2026-06-04` | `105` | `Codex` | HPHYS0286 amendment: added post-ET/pre-WB19 lower-layer upper-limit redistribution authority so WB13 storage closure consumes baseline retained layer state rather than over-cap lower layers, discard clamps, or publication compensation. |
| `2026-06-04` | `104` | `Codex` | HPHYS0285 amendment: added spring soil-retention closure authority requiring positive local WB12/WB14 same-pass infiltration to mutate WB18 layer/aggregate storage before WB13 `Total-Soil`/`SoilWaterTotal` publication, without active-snow gating and with hourly per-substep `xfin` cadence; MOFE carry/runon storage-ingress promotion remains follow-up scope. |
| `2026-06-04` | `103` | `Codex` | HPHYS0284 amendment: added corrected negative-melt snowpack state-lineage authority so `RM`/`S` routed melt and WB13 `Snow-Water` carry-state are not collapsed to the same net-melt shortcut under mixed positive/negative hourly melt. |
| `2026-06-04` | `102` | `Codex` | HPHYS0283 amendment: linked melt-aware WB12 partition authority to WB18 same-pass layer ingress (`SC-PERC-001#INV-PERC-016`) so storage closure cannot pass by reducing runoff alone. |
| `2026-06-04` | `101` | `Codex` | HPHYS0283 amendment: added baseline-authoritative `wmelt` infiltration/runoff partition authority to water-balance closure and forbade downstream storage/ET/publication compensation for melt-only runoff paths. |
| `2026-06-03` | `100` | `Codex` | HPHYS0272 amendment: linked H1 day-36 WB13/WB17/storage residual ownership to `SC-CLIMATE-001#INV-CLIMATE-013` radiation-unit closure and prohibited downstream compensation for Langley-scale radiation artifacts. |
| `2026-06-03` | `99` | `Codex` | HPHYS0271 amendment: added `INV-WATBAL-057` requiring H1 day-36 `melt.for` term-level and hourly-forcing evidence before assigning `RM`/`Snow-Water`, WB17 `Ep`, or storage residual ownership. |
| `2026-06-03` | `98` | `Codex` | HPHYS0270 amendment: added `INV-WATBAL-056` requiring daily snowpack carry-state evidence before assigning WB13 `RM`/`Snow-Water`, WB17 `Ep`, or storage residual ownership. |
| `2026-06-03` | `97` | `Codex` | HPHYS0269 follow-up amendment: aligned `INV-WATBAL-055` with corrected `/workdir/wepp-forest` negative-melt redistribution authority and rejected pinned-baseline sign/branch bug compatibility as target behavior. |
| `2026-06-03` | `96` | `Codex` | HPHYS0269 amendment: added `INV-WATBAL-055` requiring WB13 `RM`/signed `S` closure to consume baseline-authoritative retained-rain and signed-melt redistribution lineage before returning to WB17 `Ep` or storage residual tuning. |
| `2026-06-03` | `95` | `Codex` | HPHYS0268 amendment: added `INV-WATBAL-054` requiring spring snowpack/SWE/`RM` lineage evidence before returning material H1/H7/H39 `Ep` residual ownership to WB17 or changing production physics. |
| `2026-06-03` | `88` | `Codex` | HPHYS0262 amendment: added `INV-WATBAL-048` requiring WB13 `Ep`/storage residual claims to consume PMET sidecar, crop-coefficient lookup, actual ET-demand seed-branch, and baseline `evappm` provenance evidence. |
| `2026-06-03` | `89` | `Codex` | HPHYS0263 amendment: added `INV-WATBAL-049` requiring WB13 `Ep`/storage residual claims under PMET mode to consume migrated `SC-EVAP-001#INV-EVAP-021` EVAPPM demand evidence before assigning remaining residual ownership. |
| `2026-06-03` | `90` | `Codex` | HPHYS0264 amendment: added `INV-WATBAL-050` requiring PMET-mode WB13 closure evidence to consume branch-aware WB17 `pmet.es_m`/`pmet.ep_m` seam lineage, preserve SWU final `Ep`, and reject Priestley-Taylor repartition of PMET `ep`. |
| `2026-06-03` | `91` | `Codex` | HPHYS0264 review disposition: corrected PMET `Es` domain semantics to reject material negatives and allow only within-tolerance negative roundoff canonicalization. |
| `2026-06-03` | `92` | `Codex` | HPHYS0265 amendment: added `INV-WATBAL-051` requiring first-large longer-season `Ep` divergence context before assigning seasonal `Ep`/storage residual ownership. |
| `2026-06-03` | `93` | `Codex` | HPHYS0266 amendment: added `INV-WATBAL-052` requiring layer storage, WB19 active-zone, and snow/runoff first-divergence context before assigning seasonal `Ep` residual ownership. |
| `2026-06-03` | `94` | `Codex` | HPHYS0267 amendment: added `INV-WATBAL-053` requiring post-lateral/pre-SWU threshold-lineage evidence before residual ownership or production edits. |
| `2026-06-03` | `87` | `Codex` | HPHYS0261 amendment: added `INV-WATBAL-047` requiring WB13 `Ep`/storage residual claims to consume WB17 `Ep` magnitude/initialization trace evidence and legacy `evap`/`swu` call-order provenance. |
| `2026-06-03` | `86` | `Codex` | HPHYS0260 amendment: added `INV-WATBAL-046` requiring trace-grade WB17 layer uptake, WB18 percolation/storage, aggregate `watcon`, and WB13 storage publication evidence before assigning post-WB19 H1/H7/H39 residual ownership. |
| `2026-06-03` | `85` | `Codex` | HPHYS0259 amendment: added `INV-WATBAL-045` requiring trace-grade WB19 identity evidence before assigning residual ownership and shifting continuation focus to Ep/Dp/storage when WB19 identities close. |
| `2026-06-03` | `84` | `Codex` | HPHYS0258 amendment: added `INV-WATBAL-044` tying WB13 `latqcc`/`Qd` closure evidence to realized WB19 lateral withdrawal diagnostics from `SC-SUBHYD-001#INV-SUBHYD-028`. |
| `2026-06-03` | `83` | `Codex` | HPHYS0257 amendment: added hourly WB19 `ui_ssh`/`wb19_lateral_ssh_####` conductivity lineage for modern `ui_anisrt` soils so WB19 `latqcc`/`Qd` closure cannot substitute vertical `ssc`. |
| `2026-06-02` | `82` | `Codex` | HPHYS0256 amendment: added `INV-WATBAL-043` requiring WB13 `latqcc` evidence to preserve WB19 daily/hourly lateral lane provenance and consume `SC-SUBHYD-001#INV-SUBHYD-026` for daily lanes. |
| `2026-06-02` | `81` | `Codex` | HPHYS0255 amendment: added `INV-WATBAL-042` defining MOFE storage-lineage semantics, prohibiting static area-weighted storage synthesis without per-OFE dynamic hydrology-state authority, and requiring explicit storage-lineage provenance under MOFE publication. |
| `2026-06-02` | `80` | `Codex` | HPHYS0254 amendment: added `INV-WATBAL-041` requiring WB11 initial `st(i)`/`soilw(i)` hydrology seed aliases (`wb11_nsl`, `wb19_*`) to use the baseline-normalized layer grid and reconcile `Σwb19_dg` to `wb13_profile_depth_mm` while preserving AUTH03/AUTH05 generic `nsl` and FC/WP symbols. |
| `2026-06-02` | `79` | `Codex` | HPHYS0252 amendment: added `INV-WATBAL-040` tying WB19 frozen-adjusted lateral storage availability (`SC-SUBHYD-001#INV-SUBHYD-025`) to post-WB19 layer storage, WB17 root uptake, and WB13 `Total-Soil`/`SoilWaterTotal` continuation evidence. |
| `2026-06-02` | `78` | `Codex` | HPHYS0251 amendment: added `INV-WATBAL-039` coupling baseline `swu.for` uptake magnitude, crop `pltol`, layer `UPi`/`Ui`, final `Ep`, and post-uptake aggregate storage publication. |
| `2026-06-02` | `77` | `Codex` | HPHYS0250 follow-up amendment: extended WB15 interception publication to canonicalize only within-tolerance negative `I`/liquid values from snow/rain roundoff before writeback while preserving typed failures for material negatives. |
| `2026-06-02` | `76` | `Codex` | HPHYS0250 amendment: added `INV-WATBAL-038` requiring WB13 `Ep` publication to consume final post-WB19 root-uptake flux and preserving PL scheduler activation needed for root-depth production. |
| `2026-06-02` | `75` | `Codex` | HPHYS0249 amendment: added `INV-WATBAL-037` requiring WB17 `Ep`/`Es` layer-storage mutation from `SC-EVAP-001#INV-EVAP-015` before `watcon`/WB13 aggregate storage publication. |
| `2026-06-02` | `74` | `Codex` | HPHYS0248 amendment: added `INV-WATBAL-036` requiring H39 hourly `Dp`/`Pe` evidence to use `SC-PERC-001#INV-PERC-014` baseline hourly restrictive-bottom `ui_bdrkth`/`kslast` conductivity lineage. |
| `2026-06-02` | `73` | `Codex` | HPHYS0247 amendment: added `INV-WATBAL-035` tying H39 hourly closure evidence to runtime winter activation triggers and `SC-SUBHYD-001#INV-SUBHYD-024` WB19 lateral capacity lineage. |
| `2026-06-02` | `72` | `Codex` | HPHYS0246 amendment: added WB18 aggregate soil-water writeback authority requiring `wb11_soil_water`/WB13 `Total-Soil` lineage to follow `SC-PERC-001#INV-PERC-013` baseline `watcon = Σsoilw(i)` semantics instead of `Σtheta`-only percolation writeback. |
| `2026-06-01` | `71` | `Codex` | HPHYS0242 amendment: added `INV-WATBAL-034`, hourly WB14/WB12 cadence authority, surface-saturation `ui_SCrunf` clipping/addback, same-pass runoff/storage lineage, and refined HPHYS0239 ordering language so HPHYS0242 controls hourly WB19 drainage/lateral sequencing. |
| `2026-06-01` | `70` | `Codex` | HPHYS0241 amendment: added `INV-WATBAL-033` and baseline `wathour.inc`/`watbal_hourly.for` authority for 24-slot MOFE hourly carry arrays (`ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`, `ui_LfCrf`), explicit upstream/current copy-forward, aggregate-only substitution prohibition, and fail-closed malformed-array guard posture. |
| `2026-06-01` | `69` | `Codex` | HPHYS0240 amendment: added `INV-WATBAL-032` and addendum codifying same-pass `wb12_runoff_carryover` authority for WB12/WB14 runoff reconciliation, compatibility-only `wb12_runon_input` fallback, and flux publication/guard obligations. |
| `2026-06-01` | `68` | `Codex` | HPHYS0239 amendment: added `INV-WATBAL-031` and addendum codifying canonical WB19->WB12->WB13 hydrology-tail ordering plus flux-authoritative WB13 anti-shadow posture for `Q`/`Ep`/`Es`/`Er`; updated WB13 lineage register writer surfaces accordingly. |
| `2026-06-01` | `67` | `Codex` | HPHYS0238 amendment: added `INV-WATBAL-030` plus WB19 hourly iterative lateral/drainage addendum requiring seeded `wb19_lateral_drain_lane_substeps`, per-substep state recomputation, accumulated daily `q/Qdd`, and prohibition of divisor-only single-pass substitutions. |
| `2026-06-01` | `66` | `Codex` | HPHYS0235 amendment: reanchored `ui_run=1` WB18/WB11 authority to legacy `watbal_hourly` 24-substep iterative percolation semantics, requiring accumulated hourly seepage lineage for `Dp` and prohibiting divisor-only single-pass hourly treatment for closure claims. |
| `2026-06-01` | `65` | `Codex` | HPHYS0234 amendment: required flux-authoritative WB13 subsurface publication lineage for `q`/`Qdd`/`Qd` (anti-shadow posture), updated WB13 invariants and lineage register writer surfaces to `*_prefer_flux`, and added conflict-probe vector obligations. |
| `2026-06-01` | `64` | `Codex` | HPHYS0227 amendment: corrected WB19 `avfca` authority to `thetfc_####` theta lineage, added per-layer FC/WP consistency requirement (`wb18_perc_fc_#### = (thetfc_####-thetdr_####)*dg_####`), and linked required Level-4 suite `cas_l4_subhyd_watyld_fcwp_consistency_001` to `SC-SUBHYD-001#INV-SUBHYD-019`. |
| `2026-06-01` | `63` | `Codex` | HPHYS0226 amendment: added WB19 saturated-thickness lateral-response behavioral authority and linked required Level-4 suite `cas_l4_subhyd_lateral_saturated_thickness_response_001` to `SC-SUBHYD-001#INV-SUBHYD-018`. |
| `2026-06-01` | `62` | `Codex` | HPHYS0225 amendment: added WB19 layer-pool available-cap authority, prohibited legacy max-reconciliation expansion (`max(layer_pool, legacy_term)`), and linked required Level-4 suite `cas_l4_subhyd_layer_pool_withdrawal_cap_001` to `SC-SUBHYD-001#INV-SUBHYD-017`. |
| `2026-06-01` | `61` | `Codex` | HPHYS0224 amendment: added WB19 realized-withdrawal soil-water cap authority (non-clamping subtraction and typed over-withdrawal hard-fail) with required Level-4 suite linkage to `SC-SUBHYD-001#INV-SUBHYD-016`. |
| `2026-05-31` | `60` | `Codex` | AUTH09 taxonomy normalization: introduced Level-3 legacy/sanity tier usage for WB19 branch governance and renamed suite reference to `cas_l3_subhyd_solwpv_fcdep_branch_001`. |
| `2026-05-31` | `58` | `Codex` | HPHYS0222 amendment: corrected WB19 `fcdep/unsdep` mutation authority to `solwpv < 2006` only (no `fcdep` mutation for `solwpv >= 2006`, including `9001+`) and linked external-authority suite `cas_l4_subhyd_solwpv_fcdep_branch_001`. |
| `2026-05-31` | `59` | `Codex` | AUTH08A governance re-tiering: reclassified `cas_l4_subhyd_solwpv_fcdep_branch_001` as periodic/investigation legacy-conformance evidence (non-blocking) pending independent constitutive authority. |
| `2026-05-31` | `57` | `Codex` | AUTH03 amendment: added Level-4 constitutive gate bootstrap authority for FC/WP and relax-to-FC percolation threshold closure, including blocking suite linkage and fail-closed symbol posture. |
| `2026-05-31` | `56` | `Codex` | HPHYS0221 amendment: added WB19 `solwpv` branch semantics and coupled water-yield/saturated-depth authority (`avpora`, `avfca`, `avcoca`, `watyld`, `fcdep`, `unsdep`) with required runtime publications (`wb19_watyld`, `wb19_fcdep`, `wb19_unsdep`) and fail-closed domain posture. |
| `2026-05-31` | `55` | `Codex` | HPHYS0219 amendment: corrected WB19 `drfc` coefficient-family authority from `cpm_####` to baseline-authoritative `coca_####` and retained typed hard-fail domain guards for `coca` surfaces. |
| `2026-05-31` | `54` | `Codex` | HPHYS0218 amendment: required WB19 `drfc`-equivalent threshold lineage (`wb18_perc_fc_#### + (1-coca_####)*dg_####`) for saturated-zone classification and lateral/drainage withdrawals with fail-closed `coca_####` guard posture. |
| `2026-05-31` | `53` | `Codex` | HPHYS0216D amendment: reconciled WB13 `ProfileFCStore` authority to layer aggregation plus explicit normalized-tail contribution (`wb13_profile_fc_tail_mm`), retained `wb13_profile_fc_store_mm` as diagnostic/reconciliation lineage, and added typed fail-closed guard posture for missing/non-finite/negative tail symbols. |
| `2026-05-31` | `52` | `Codex` | HPHYS0216 amendment: realigned `ProfileFCStore` publication authority to baseline-authoritative layer aggregation (`Σ(thetfc_i*dg_i)*1000`), demoted `wb13_profile_fc_store_mm` to diagnostic carry surface, and preserved typed fail-closed guard obligations for missing/non-finite layer symbols. |
| `2026-05-30` | `51` | `Codex` | HPHYS0209 amendment: codified near-closed `ProfileWPStore` adjudication governance, preserving HPHYS0207 `wb13_profile_wp_store_mm` publication authority and allowing isolated stable residuals as diagnostic expected process-correct evidence only under non-regressing profile depth/capacity/order conditions. |
| `2026-05-30` | `50` | `Codex` | HPHYS0208 amendment: required baseline-authoritative WB11 seed threshold lineage (`sat`, `por_####`, `cpm_####`, `thetfc_####`, `thetdr_####`, `dg_####`) for `st(i)`/`FCi`/`ULi` initialization and coupled WB11/WB18 publication continuity to WB13 `Dp`/`latqcc`/`Total-Soil`/`SoilWaterTotal`. |
| `2026-05-30` | `49` | `Codex` | HPHYS0203 amendment: added WB13 physics-robustness validation obligations for profile, soil-water aggregate, and subsurface-loss publication families, including conservation/order/domain/non-finite vectors, deterministic perturbation checks, and per-family deterministic regression fixture requirements. |
| `2026-05-30` | `47` | `Codex` | HPHYS0206 amendment: required authoritative FC/WP layer publication symbols to be mapped deterministically from the same baseline-normalized corrected-layer set used by profile-capacity lineage, with explicit no-raw-fallback typed fail-closed posture. |
| `2026-05-30` | `48` | `Codex` | HPHYS0207 amendment: aligned WB13 FC/WP publication authority to normalized-profile runtime storage symbols (`wb13_profile_fc_store_mm`, `wb13_profile_wp_store_mm`) and added explicit normalized-tail consumption policy authority. |
| `2026-05-29` | `46` | `Codex` | HPHYS0205 amendment: required authoritative WB13 layer symbols (`thetfc_####`/`thetdr_####`) to carry baseline-corrected moisture lineage while retaining layer-authoritative publication and non-authoritative FC/WP adapter diagnostics. |
| `2026-05-29` | `45` | `Codex` | HPHYS0202 amendment: made `ProfileFCStore`/`ProfileWPStore` publication authority explicitly layer-aggregated (`Σ(thetfc_i*dg_i)`, `Σ(thetdr_i*dg_i)`), and restricted `wb13_profile_fc/wp_store_mm` to non-authoritative adapter diagnostics. |
| `2026-05-29` | `44` | `Codex` | HPARITY02 amendment: added profile-capacity publication-lineage closure authority (`wb13_profile_*_mm`) and explicit prohibition of synthesized `ProfilePorosityCap` placeholder formulas. |
| `2026-05-29` | `43` | `Codex` | HPARITY01 amendment: added always-fail WB13 12-column lineage register with canonical symbol ownership, process-contract disambiguation (`Dp` deep-percolation vs climate time-to-peak), runtime writer surfaces, and explicit alias continuity policy for `Total-Soil`/`Total-Soil Water`/`SoilWaterTotal`. |
| `2026-05-29` | `42` | `Codex` | HILLSTAB08 amendment: landed baseline-authoritative WB16 `ealpha` producer-chain runtime migration (`frcfac -> rdat(alpha) -> alphay -> eplane`), added runtime-producer provenance vector (`runtime_provided`), retained explicit compatibility degradation policy (`SIMPIPE-W-003`), and dispositioned `GAP-WATBAL-005` to `closed`. |
| `2026-05-29` | `41` | `Codex` | HILLSTAB07 amendment: added explicit WB16 input-provenance authority for canonical `m=1.5`, baseline `ealpha` producer-chain lineage, compatibility-seed provenance surfaces/warning obligations (`wb16_ealpha_compatibility_seed_used`, `wb16_ealpha_seed_policy`, `SIMPIPE-W-003`), and non-promotable gap row `GAP-WATBAL-005` for full producer migration closure. |
| `2026-05-29` | `40` | `Codex` | HILLSTAB06 amendment: aligned WB16 authority to baseline `appmth` near-zero runoff branch (`Q < 1.0e-8`) and explicit positivity-domain semantics so positive near-zero WB16 intermediates do not fail pre-floor. |
| `2026-05-28` | `39` | `Codex` | HILLSTAB03 WB16 amendment: corrected baseline `appmth.for` branch authority by deriving `tc` from `vstar`, adding explicit `vstar>=1` constant-excess branch (`qpstar=1`), removing non-authoritative `timep` as required WB16 coupling input, and updating WB16 domain/test-vector obligations accordingly. |
| `2026-05-26` | `38` | `Codex` | SIMIMPL36 amendment: added explicit WB12/WB14 near-zero reconciled-runoff canonicalization authority (`TOL-WATBAL-006`) requiring `Q`/`wb12_runoff_reconciled` normalization to zero only within `[-1e-12, 0)` before writeback/publication while preserving typed domain-fail posture for material negatives. |
| `2026-05-25` | `37` | `Codex` | MOFE13 amendment: added baseline-authoritative WB14 `ksatadj` three-regime conductivity selection authority (`9001` exponential recovery, `9002` Saxton-Rawls Brooks-Corey, `9003` burn-severity floor), including required regime symbols and typed active-path guard obligations. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-04 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with authority anchors, invariants, guard map, alias map, obligations, tolerances, and gap register for SCI-04 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: clarified daily-step closure enforcement, added explicit zero-demand `Ws` branch, expanded alias map coverage, and added Chapter-5 validation caveat gap entry. |
| `2026-05-23` | `3` | `Codex` | WB10 amendment: added scheduler hydrology phase-entry routing authority for runoff/storage classes with explicit unsupported-class hard-fail posture and WB10 test-vector obligations. |
| `2026-05-23` | `4` | `Codex` | WB11 amendment: promoted hydrology section from routing-only scaffolding to ET/percolation/lateral/drain production-kernel authority with deterministic WB11 updates, typed WB11 guard codes, and WB11 contract-derived vectors. |
| `2026-05-23` | `5` | `Codex` | WB12 amendment: added runoff/storage reconciliation kernel authority with deterministic closure diagnostics, typed WB12 guard codes, and WB12 contract-derived vectors. |
| `2026-05-23` | `6` | `Codex` | WB13 amendment: added canonical daily water-balance output surface authority (`H5.wat.dat` equivalent) with fixed 25-column schema, deterministic row-order key rules, typed WB13 output guards, and WB13 contract-derived vectors. |
| `2026-05-23` | `7` | `Codex` | INT10 amendment: added coupled watbal lane-entry invariant (`INV-WATBAL-011`), scheduler ordering-precondition guard authority, and INT10 coupled replay test-vector obligations for ordering/state-transfer validation. |
| `2026-05-23` | `8` | `Codex` | PL14 amendment: added replay-candidate emission invariant (`INV-WATBAL-012`) with strict WB13 schema/order + artifact completeness guard authority for Tier-A closeout staging. |
| `2026-05-23` | `9` | `Codex` | WB14 amendment: added computed infiltration + hyetograph coupling authority for runoff reconciliation, replacing required externally seeded infiltration input in acceptance paths and adding typed WB14 runoff guards. |
| `2026-05-23` | `10` | `Codex` | CLIM05 amendment: added signed `S` snow-coupling authority for WB12 storage reconciliation, including required storage-surface inputs, deterministic storage equation update, and typed active-coupling guard vectors. |
| `2026-05-23` | `11` | `Codex` | CLIM06 amendment: added frozen-soil infiltration-capacity coupling authority (`frost.runtime_infcap_frz`) for WB14 runoff reconciliation, bounded frozen-state surface requirements, and typed active-coupling guard vectors. |
| `2026-05-23` | `12` | `Codex` | WB15 amendment: added canopy interception coupling authority from plant runtime surfaces (`cancov`, `lai`, `vdmt`) with Eq. [5.1.2] lineage, explicit runoff/storage closure integration of `I`, and typed hard-fail guard posture for missing/non-finite/domain-invalid canopy symbols. |
| `2026-05-23` | `13` | `Codex` | IRRIG10 amendment: added explicit irrigation depth coupling (`Irr`) into WB12 storage reconciliation equation and typed guard/test-vector obligations for irrigation-triggered runoff/storage closure. |
| `2026-05-23` | `14` | `Codex` | WB16 amendment: added closure-diagnostics peak-runoff authority (`peakro`, `watdur`) with three method branches (`tstar`/`tc`), explicit minimum-flow and max-duration rules, and typed WB16 guard/test-vector posture. |
| `2026-05-23` | `15` | `Codex` | ARCH22 amendment: added typed production-surface authority requiring covered WB11/WB12/WB14/WB15/WB16 interfaces to consume boundary symbols via ARCH22 typed symbol families while preserving existing failure-class/message continuity. |
| `2026-05-23` | `16` | `Codex` | PL14R amendment: added strict replay rerun candidate-surface invariant (`INV-WATBAL-014`) requiring explicit candidate-lane coverage of `H5.wat.dat` and `H5.plot.dat` with no fallback substitution. |
| `2026-05-23` | `17` | `Codex` | PL15R amendment: added schema-aligned replay supersession invariant (`INV-WATBAL-015`) requiring Tier-A WB13 residual classification from canonical 25-column strict replay and keyed day-by-day parity evidence before retaining blockers. |
| `2026-05-23` | `18` | `Codex` | WB17 amendment: updated hydrology contract authority to reflect WB17 equation-driven ET partition execution (`Ep`, `Es`, `Er`, `ET`, `Ws`) with explicit runtime aliases (`Eu -> wb11_et_demand`, `L -> lai`, `Er -> wb17_residue_interception`). |
| `2026-05-23` | `19` | `Codex` | EROD11 amendment: ratified Wave-0 alias ownership for runoff/peak-duration coupling surfaces, added explicit cross-contract ownership register, and downgraded `GAP-WATBAL-003` from non-promotable to promotable-with-risk pending broader internal alias expansion. |
| `2026-05-23` | `20` | `Codex` | WB18 amendment: updated hydrology contract authority to require WB18 per-layer percolation symbols (`wb18_perc_theta/fc/ul/ssc/pei_####`) and WB18 per-layer deterministic routing semantics while preserving WB17 ET and WB11 lateral/drain guard posture. |
| `2026-05-23` | `21` | `Codex` | EROD11 closure amendment: dispositioned alias-ownership ambiguity row `GAP-WATBAL-003` to `closed` for required boundary symbols and made explicit that erosion-physics implementation remains separately governed by non-promotable holds. |
| `2026-05-23` | `22` | `Codex` | EROD11 risk-acceptance amendment: dispositioned `GAP-WATBAL-001` and `GAP-WATBAL-004` from promotable-with-risk to `closed` via explicit governance risk acceptance while preserving non-promotable erosion-physics HOLD posture. |
| `2026-05-23` | `23` | `Codex` | WB19 amendment: updated hydrology authority from WB18+WB11 surrogate lateral/drain execution to WB18+WB19 layer-aware lateral/drainage execution, including explicit WB19 geometry/anisotropy symbol requirements and guard posture continuity on legacy status IDs. |
| `2026-05-23` | `24` | `Codex` | WB20 amendment: added forward-solver lane selector authority (`wb20_forward_solver_lane_enabled`) and lane-scoped closure semantics so parity-lane acceptance is solver-residual-derived and excludes observed closure targets from acceptance-driving inputs. |
| `2026-05-23` | `25` | `Codex` | EROD12 amendment: added cross-domain ownership/guard closure addendum for required erosion-lane hydrology boundary exports while preserving existing non-Wave-0 companion-gap posture (`GAP-WATBAL-002`). |
| `2026-05-24` | `26` | `Codex` | CLI02 amendment: replaced required replay include-surface authority from legacy/bootstrap candidate files (`H5.wat.dat`, `H5.plot.dat`) to simulation-driven partitioned interchange candidate surfaces (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`) and updated WB13 replay vector wording accordingly. |
| `2026-05-24` | `27` | `Codex` | PL14S amendment: added semantic replay diagnostics invariant (`INV-WATBAL-017`), semantic comparator guard/disposition authority, WB13 semantic evidence vectors, and explicit producer obligations for investigation-grade WB13 parity reporting. |
| `2026-05-24` | `28` | `Codex` | SIMIMPL03 amendment: added production execution ownership, mode-propagation closure, simulation-owned WB13 provenance, and selective consolidated-intake guardrail invariants (`INV-WATBAL-018..021`) with typed guards (`HS-SIMPIPE/SIMMODE/SIMOUT/SIMCONS`) and addendum authority. |
| `2026-05-25` | `29` | `Codex` | EROD13 amendment: activated Wave-1 runoff/peak-duration producer-coupling authority for erosion-core ingress under `erod13_core_enabled`, preserving typed hard-fail continuity (`HKERNEL-EROD13-CORE-E-001..003`) for missing/non-finite/domain-invalid coupling payloads. |
| `2026-05-25` | `30` | `Codex` | EROD14 amendment: added Wave-2 runoff producer-coupling continuity for multi-OFE/enrichment ingress surfaces (`erod14_qout`, `erod14_qin`) with typed hard-fail guard continuity (`HKERNEL-EROD14-WAVE2-E-001..003`). |
| `2026-05-25` | `31` | `Codex` | SIMIMPL14 amendment: added continuous runner span/key closure invariant (`INV-WATBAL-022`) requiring full climate-span day progression, carried-state daily lifecycle execution, simulation-year WB13 key mapping, monotonic `sim_day_index`, and manifest continuity assertions for replay-overlap readiness. |
| `2026-05-25` | `32` | `Codex` | SIMIMPL15 amendment: added strict/parquet lane-policy + candidate-source provenance closure invariants (`INV-WATBAL-023`), parquet semantic alias/width diagnostic continuity invariant (`INV-WATBAL-024`), and explicit replay-tooling obligations for non-promotable conversion-derived dat strict evidence classification. |
| `2026-05-25` | `33` | `Codex` | SIMIMPL16 amendment: added replay contract-derived test-coverage closure invariant (`INV-WATBAL-025`) and explicit producer/governance obligations for span/key overlap, strict-lane compensation, alias continuity, and conversion-derived dat row-consistency test enforcement. |
| `2026-05-25` | `34` | `Codex` | SIMIMPL18 amendment: added day-key rain/snow partition and publication-source closure invariant (`INV-WATBAL-026`), storage-state mutation invariant (`INV-WATBAL-027`), producer obligations for runtime-derived `RM`/`Snow-Water` publication, and addendum authority for first-day + multi-day storage diagnostics. |
| `2026-05-25` | `35` | `Codex` | SIMIMPL21 amendment: added baseline WB11 ET/soil-water ordering authority (`INV-WATBAL-028`) and layer-to-aggregate publication-lineage authority (`INV-WATBAL-029`) with explicit legacy provenance anchors and SIMIMPL22/SIMIMPL23 gating obligations. |
| `2026-05-25` | `36` | `Codex` | MOFE04 amendment: added explicit multi-OFE WB13/H.wat canonicalized publication policy authority (`OFE=1` row id semantics), required publication provenance fields (`publication_ofe_policy`, `contributor_ofe_count`, `area_policy`, `publication_area_m2`), and aggregate OFE-geometry area closure obligations. |
