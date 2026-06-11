---
contract_id: SC-SNOWFREEZE-001
title: Snow and Freeze Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 58
producer_scope:
  - Winter precipitation phase partition surfaces (rain vs snow)
  - Snowpack depth/density/water-equivalent state surfaces
  - Melt and freeze-thaw transition surfaces
consumer_scope:
  - Daily water-balance accounting consumers
  - Infiltration/runoff partition consumers affected by frozen-soil state
  - Soil/erosion coupling consumers requiring freeze-thaw context
evidence_level: static
last_reviewed: 2026-06-11
supersedes: []
superseded_by: []
---

# SC-SNOWFREEZE-001 Snow and Freeze Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for snow accumulation/melt and freeze-thaw
process behavior, including downstream coupling boundaries in openWEPP.

## Scientific Scope

In scope:
- Snowpack accumulation, density, and melt boundary behavior. `[DIRECT][Static]`
- Frozen-soil and thaw transition boundary behavior relevant to hydrology. `[DIRECT][Static] + [INFERENCE][Static]`
- Required producer/consumer boundary semantics for winter-process handoff. `[INFERENCE][Static]`
- Hourly winter-process forcing transformations derived from daily climate input. `[DIRECT][Static]`

Out of scope:
- Kernel implementation details. `[INFERENCE][Static]`
- Non-snow/freeze domains except required coupling boundaries. `[INFERENCE][Static]`
- Standalone activation of snow drifting process equations not active in the
  August 1995 WEPP release lineage. `[DIRECT][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-SNOWFREEZE-CH3-INTRO | `references/50201000/chap3.pdf` §3.1 | Winter routine scope, activation conditions, and declared outputs/processes (hourly snow accumulation/melt/frost-thaw). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-HRPRECIP | `chap3.pdf` §3.2 | Hourly precipitation derivation and disaggregation/start-time semantics for winter routine inputs. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-HRTEMP | `chap3.pdf` §3.3, Eq. [3.3.1]-[3.3.3] | Hourly air/surface temperature derivation semantics used by melt/frost routines. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-HRRAD | `chap3.pdf` §3.5, Eq. [3.5.1]-[3.5.7] | Hourly radiation derivation used by snowmelt energy terms. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-MELT | `chap3.pdf` §3.6, Eq. [3.6.1]-[3.6.6] | Melt equation structure and component terms (`amelt`, `bmelt`, `cmelt`, `dmelt`). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-MELT-ASSUMP | `chap3.pdf` §3.6 assumptions list | Melt gating assumptions (`Tmax` thresholds, density threshold, bounded melt, albedo assumption). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-SNOWDENS | `chap3.pdf` §3.7, Eq. [3.7.1]-[3.7.5] | Snow depth/density update rules under snowfall, settling, melt, and mixed melt+snowfall. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-SNOWDENS-LIM | `chap3.pdf` §3.7 terminal paragraph | Explicit upper density limit (`522 kg m^-3`). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-FROST | `chap3.pdf` §3.8, Eq. [3.8.1]-[3.8.4] | Frost/thaw heat-flow relations, layered thermal conductivity, and hourly bookkeeping outputs. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-DRIFT-INACTIVE | `chap3.pdf` §3.9 intro paragraph | Snow drifting equations are described but not currently active in the August 1995 WEPP release. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH4-COUPLING | `references/50201000/chap4.pdf` §4.1 | Infiltration/runoff components consume rainfall-excess timing/intensity and infiltrated water surfaces. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH5-COUPLING | `references/50201000/chap5.pdf` §5.1, Eq. [5.1.1] | Daily water balance includes snow-water term and treats melted snow as rainfall for runoff/percolation estimation. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT | Archived original `/workdir/wepp-forest_260430_baseline/src/winter.for` lines 420-464 and `/workdir/wepp-forest_260430_baseline/src/melt.for` lines 275-301 at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Archived signed-hourly-melt source and original bug-compatible negative-melt context. The daily negative-melt sign/scale branch is no longer the active comparator after ADR-0016/HPHYS0303; active fixed comparator authority is `REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX`. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX | Fixed `wepp_260430` comparator branch `wepp_260430_negmeltfix_comparator`, tag `wepp_260430_negmeltfix_comparator_47ac4c32faee`, commit `47ac4c32faeea81bb99081f955a14c38b815ef4d`, `src/winter.for` lines 434-453; patch provenance `/workdir/wepp-forest` commit `03fee4558456535138592630b5dedc4d81ce8d06` and `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-comparator-source-delta.patch` | Active comparator and target daily negative-melt authority: compare net daily melt with `pstvML + ngtvML <= 0.0`; when net melt is positive, reduce positive hourly melt by scaling with `1 + ngtvML/pstvML` for routed melt and apply the companion `snodpt = snodpt + ngtvML*1000/densgt` carried-depth adjustment. This supersedes the archived original sign/branch bug. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-SNOWD-RAINSTORE | `/workdir/wepp-forest_260430_baseline/src/snowd.for` lines 240-279 | Pinned baseline rain-on-snow holding-capacity branch consumes hourly rain into snowpack density until `350 kg m^-3`, leaving only residual rain as liquid runoff/infiltration forcing. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE | `/workdir/wepp-forest_260430_baseline/src/winter.for` lines 456-459 | Pinned baseline daily winter post-processing adds positive residual `hrrain(hour)` remaining after `snowd.for` holding-capacity accounting into `hrmlt(hour,iplane)` before `totmel`/`wmelt(iplane)` publication. Residual rain-on-snow therefore follows the `hrmlt`/`wmelt` routed-melt lineage rather than an independent direct-rain-only path. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-WMELT-INFIL | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` lines 342-345 and `/workdir/wepp-forest_260430_baseline/src/grna.for` lines 267-269, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative meltwater partition: daily redistributed `wmelt(iplane)` is included in `fin` water available for infiltration and as Green-Ampt event forcing (`smrate = wmelt(iplane) / dur`) before residual runoff is finalized. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-WB13-RM-SNOW | `/workdir/wepp-forest_260430_baseline/src/contin.for:847-880`, `/workdir/wepp-forest_260430_baseline/src/watbalprint.for:84-106`, and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1082-1142`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | WB13 snow-related publication consumes post-winter `rain(iplane)`, daily `wmelt(iplane)`, and snowpack storage `snodpy(iplane)*densg(iplane)` rather than reconstructing `RM` from raw precipitation and SWE delta. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-WNTTIM-MIN | `/workdir/wepp-forest_260430_baseline/src/winter.for:206-235`, `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline precipitation-phase start-time authority for snow/freeze forcing: finite `wnttim < 1.0` is normalized to `1.0` before `stmtim` active membership and rain/snow branch selection. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative snow depth/water and bounded densities are required for physical validity. | `[INFERENCE][Static]` |

## Variables and Units

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `Dsold` | `m` | Snow depth before hourly update. | winter snow routine | snow density/melt update logic |
| `Dsnew` | `m` | Snow depth after hourly update. | winter snow routine | melt routing, frost conductivity terms |
| `Dsavail` | `m` | Available pre-hour snow depth state used by Eq. [3.6.1] melt upper-bound branch (legacy text refers to preceding-hour snow depth state). | winter snow routine | melt bound branch semantics |
| `ρsold` | `kg m^-3` | Snow density before update. | winter snow routine | density transition logic |
| `ρsnew` | `kg m^-3` | Snow density after update. | winter snow routine | melt gating and density cap checks |
| `hrsnow` | `m` | Hourly snowfall depth increment. | hourly precip partition | snowpack accumulation update |
| `faldr` | `m` | Falling drift contribution used by legacy drift equations. | legacy drift formulation (inactive in target lineage) | governance/provenance only |
| `grdri` | `m` | Ground drift contribution used by legacy drift equations. | legacy drift formulation (inactive in target lineage) | governance/provenance only |
| `hrmelt` | `m` | Hourly melt water from snowpack. | melt routine | DISAG/infiltration-runoff coupling |
| `hrrain` | `m` | Hourly rainfall amount. | hourly precip partition | melt term and runoff/infiltration forcing |
| `hrrain_store` | `m` | Hourly rain retained in sub-`350 kg m^-3` snowpack holding capacity. | snow density update | daily `S`, runtime SWE, liquid-forcing reduction |
| `hrrain_release` | `m` | Residual positive rain-on-snow left in `hrrain` after holding-capacity accounting and added into `hrmlt`/`wmelt` during daily winter post-processing. | snow density update + winter post-processing | routed snowmelt event forcing (`wmelt -> fin/smrate`) |
| `wmelt` | `m` | Daily routed snowmelt liquid after hourly melt redistribution and residual rain-on-snow release. | winter routine | runoff/infiltration forcing and WB13 `RM` publication |
| `hrmelt_raw` | `m` | Signed hourly energy-balance melt before daily negative-melt redistribution. | melt routine | daily redistribution and diagnostics |
| `Thr` | `degC` | Hourly air temperature. | hourly temperature routine | melt/frost branch logic |
| `Thra` | `degC` | Hourly adjusted surface temperature. | surface energy balance routine | frost routine driver |
| `Tmax` | `degC` | Daily maximum air temperature. | climate forcing | rain/snow partition and melt gating |
| `Tmin` | `degC` | Daily minimum air temperature. | climate forcing | rain/snow partition and hourly temperature |
| `hrrad` | `MJ m^-2` | Hourly radiation on sloping surface. | SUNMAP routine | melt radiation term |
| `cancov` | `fraction` | Canopy cover fraction (`0..1`). | plant/canopy state | melt attenuation term |
| `clouds` | `fraction` | Cloud-cover fraction (`0..1`). | hourly radiation/cloud routine | melt and surface-temperature terms |
| `Qsrf` | `W m^-2` | Heat flux through snow-residue-frozen-soil layered system. | frost routine | freeze/thaw depth update bookkeeping |
| `Quf` | `W m^-2` | Heat flow from unfrozen soil below freezing front. | frost routine | freeze/thaw depth update bookkeeping |
| `Ksrf` | `W m^-1 degC^-1` | Harmonic-mean layered thermal conductivity for snow-residue-frozen system. | frost routine | heat-flux computation |
| `Snowd` | `m` | Snow layer depth in layered frost conductivity equation. | snow routine | frost layered conductivity equation |
| `Resd` | `m` | Residue thickness in layered conductivity equation. | residue/management surfaces | frost layered conductivity equation |
| `Tilld` | `m` | Frozen tilled-layer depth in conductivity equation. | soil/frost routine | frost layered conductivity equation |
| `Utilld` | `m` | Frozen untilled-layer depth in conductivity equation. | soil/frost routine | frost layered conductivity equation |
| `Dfrost` | `m` | Frost depth output from hourly frost bookkeeping. | frost routine | soil-state and winter coupling consumers |
| `Dthaw` | `m` | Thaw depth output from hourly frost bookkeeping. | frost routine | soil-state and winter coupling consumers |
| `S` | `m` | Daily snow-water storage term in Eq. [5.1.1] (`+` melt, `-` accumulation). | winter routine | daily water-balance closure |
| `Ws_frz` | `m` | Water accumulated in frozen soil (hourly bookkeeping output). | frost routine | infiltration-capacity adjustment / reporting |
| `Nft` | `count` | Number of freeze-thaw cycles over winter bookkeeping. | frost routine | soil-state/infiltration-capacity coupling |
| `InfCap_frz` | `m s^-1` | Infiltration capacity of tilled layer/top `0.20 m` (untilled case) under frost routine output; non-SI internal units must be converted at boundary publish. | frost routine | infiltration/runoff component |
| `Snow-Water` | `mm` | WB13/hydout snow-water storage publication surface converted from runtime SWE at the output boundary. | winter runtime state publication | WB13/hillslope WAT output |
| `winter_rad_hourly` | `MJ m^-2 h^-1` | Registry-owned hourly winter radiation forcing surface. | hourly radiation adapter | melt-forcing diagnostics |
| `winter_air_temp_hourly` | `degC` | Registry-owned hourly winter air-temperature forcing surface. | hourly temperature adapter | melt/frost branch diagnostics |
| `winter_dewpoint_hourly` | `degC` | Registry-owned hourly winter dewpoint forcing surface. | hourly climate adapter | melt-term diagnostics |
| `winter_wind_hourly` | `m s^-1` | Registry-owned hourly winter wind-speed forcing surface. | hourly climate adapter | melt-term diagnostics |
| `winter_cloud_fraction_hourly` | `dimensionless` | Registry-owned hourly winter cloud-fraction forcing surface. | hourly climate adapter | melt/surface-temperature diagnostics |
| `snow_runtime_swe` | `m` | Runtime snow-water-equivalent state surface. | winter snow routine | snow coupling and WB13 publication |
| `snow_routed_melt` | `m` | Daily routed meltwater surface after winter post-processing. | winter routine | WB12/WB13 liquid-forcing consumers |
| `snow_post_winter_rain` | `m` | Direct-rain depth remaining after winter rain retention/release. | winter routine | WB13 `RM` and liquid-forcing consumers |
| `snow_runtime_depth` | `m` | Runtime snow-depth state surface. | winter snow routine | snow/frost diagnostics |
| `snow_runtime_density` | `kg m^-3` | Runtime snow-density state surface. | winter snow routine | snow/frost diagnostics |
| `snow_runtime_settle_day_count` | `count` | Runtime snow settle-day counter. | winter snow routine | snow carry-state diagnostics |
| `snow_hourly_rain` | `m` | Hourly rainfall forcing before snowpack retention. | hourly precipitation partition | snow/rain-retention diagnostics |
| `snow_hourly_rain_retained` | `m` | Hourly rain retained in snowpack holding capacity. | snow density update | snowpack liquid-storage diagnostics |
| `snow_hourly_snowfall` | `m` | Hourly snowfall depth forcing. | hourly precipitation partition | snowpack accumulation diagnostics |
| `snow_hourly_depth` | `m` | Hourly snow-depth before/available/after state family. | winter snow routine | melt-bound and snow-state diagnostics |
| `snow_hourly_density` | `kg m^-3` | Hourly snow-density before/after state family. | winter snow routine | density gate diagnostics |
| `snow_hourly_melt` | `m` | Hourly post-redistribution meltwater surface. | winter routine | melt/routing diagnostics |
| `snow_hourly_melt_raw` | `m` | Signed hourly raw melt before daily redistribution. | melt routine | negative-melt diagnostics |
| `snow_hourly_melt_branch_active` | `dimensionless` | Hourly melt-branch active flag. | melt routine | melt-forcing diagnostics |
| `snow_hourly_melt_terms` | `in` | Hourly `amelt`/`bmelt`/`cmelt`/`dmelt` term family before metric conversion. | melt routine | term-level melt diagnostics |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SNOWFREEZE-001 | Melt bound and non-negativity branch semantics: post-branch exported melt satisfies `0 <= hrmelt <= Dsavail`, where `Dsavail` is the pre-hour available snow-depth state used by Eq. [3.6.1] branch logic. | hard-fail | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-MELT-ASSUMP, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-002 | Snow-density melt gate: liquid melt export to infiltration/runoff is not allowed until post-update snow density reaches `ρsnew >= 350 kg m^-3`; below this threshold melt remains in-pack and increases density. | hard-fail | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-SNOWDENS | `[DIRECT][Static]` |
| INV-SNOWFREEZE-003 | Snow depth-density domain bounds: `Dsold >= 0`, `Dsnew >= 0`, `ρsold >= 0`, `ρsnew >= 0`, and `ρsnew <= 522 kg m^-3`; when `Dsnew = 0`, `ρsnew = 0`. | hard-fail | REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH3-SNOWDENS-LIM, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-004 | Active snow-update branch consistency: fresh snowfall contribution uses `100 kg m^-3` density and active depth/density updates follow Eq. [3.7.1]-[3.7.5] for settling, snowfall, melt, and melt+snowfall cases; drift-term equations remain governance-only while drift is inactive. | hard-fail | REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH3-DRIFT-INACTIVE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-005 | Rain/snow partition consistency: precipitation phase partition follows daily temperature logic (`Tmax < 0` => all snow; `Tmin > 0` => all rain; mixed day uses hourly occurrence/diurnal temperature function). | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-HRPRECIP, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static]` |
| INV-SNOWFREEZE-006 | Frost heat-flow formulation consistency: frost/thaw bookkeeping uses explicit layered heat-flow relations (`Qsrf`, `Quf`) and harmonic-mean layered thermal conductivity per Eq. [3.8.1]-[3.8.4]. Executable frost-depth progression must derive from hourly signed heat flow, latent-heat increments, and the active fine-layer frozen-depth/frozen-water state (`wb18_perc_frozen_depth_####`, `wb18_perc_frzw_####`). Freeze energy advances the front only by freezing layer water into the same store that `frwatc` later publishes; thaw energy retreats the active front by melting that same layer ice back into liquid storage. The surface heat path must be resisted by the current frozen-layer thickness (`Σ dz/k` through snow, residue, tilled frozen soil, and untilled frozen soil), and the lower heat path remains a separate `Quf` term. Depth must be bounded by the physical soil profile and by the layer/fine-layer capacity exposed by that state; it must not use the retired `0.20 m * clamp(mean-temperature / 6 degC)` freeze-index proxy, a `0.20 m` model cap, post-hoc scalar depth projection into layer stores, or a scalar `frdp * theta` frozen-water surrogate. | hard-fail | REF-SNOWFREEZE-CH3-FROST | `[DIRECT][Static]` |
| INV-SNOWFREEZE-007 | Winter coupling payload completeness: hourly winter outputs required for downstream consumers are emitted with valid units/domains, including `hrmelt`, `Dfrost`, `Dthaw`, `Nft`, `Ws_frz`, and `InfCap_frz`, and daily snow-water term `S` is consistently reflected in water balance semantics. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-FROST, REF-SNOWFREEZE-CH4-COUPLING, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-008 | Snow drifting governance invariant: process claims requiring active drift transport equations are non-promotable until authority confirms an active drift implementation path for the target lineage. | governance-fail | REF-SNOWFREEZE-CH3-DRIFT-INACTIVE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-009 | Winter-routine activation branch is explicit: winter hourly processing is invoked when at least one trigger condition is true (existing snowpack, existing soil frost layer, or average daily temperature below `0 degC`), with no silent bypass. Activation depends on runtime state/forcing triggers, not snow-sidecar or frost-sidecar presence alone; parsed default snow/frost controls are valid controls when the missing-file branch has explicitly set defaults. For standard `ksflag` frost, `frost.options.frost_file_present` is provenance only and must not suppress the frozen-soil routine when `frost.options.wintRed=1` and thermal/runtime frost triggers are active. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, INV-SNOWFREEZE-012, INV-SNOWFREEZE-013 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-010 | CLIM05 parsed snow-control coupling invariant: when parsed `snow.options.*` controls are projected to runtime, coupling must enforce finite/valid control domains (`newsnw > 0`, `ssd > 0`, `newsnw <= ssd`), publish signed `S = melt - accumulation`, and maintain non-negative `snow.runtime_swe` without silent fallback/defaulting. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH5-COUPLING, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-011 | SIMIMPL18 day-key partition/publication closure: for active snow coupling and precipitation days where `Tmax <= rst`, liquid runoff-coupling input from direct rainfall/melt is zero for that day key (`RM = 0`), snow storage update remains explicit (`snow.runtime_swe(new) = snow.runtime_swe(old) + accumulation - melt`), and downstream published `Snow-Water`/hydout-equivalent snow storage values derive from runtime SWE state rather than static sidecar control `snow.options.ssd`. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-HRPRECIP, REF-SNOWFREEZE-CH5-COUPLING, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-012 | Frost routine-chain dispatch and handoff closure: active winter-hourly frost triggers dispatch `winter -> frostN`, `frostN` performs water-state handoff with `frwatc(1)` once at active-day hour-1 ingress (`frostn.for:335-337`, guarded by `hour.eq.1`) and `frwatc(0)` at day-end/thaw-complete exit, and freeze-active branches execute `frzng -> frznw` lineage without silent bypass. Implementations must not interpret `frwatc(1)` as an every-hour entry handoff because that would reapply the daily `st - yst` water-balance delta. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-FROST | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-013 | Frozen-soil conductivity authority closure: when frost is present, saturated-conductivity coupling follows `frsoil` fine-layer aggregation with `getFreezeCond` land-use-dependent `kfactor` selection and remains explicitly bounded/typed at the runtime seam (`frost.runtime_infcap_frz`). | hard-fail | REF-SNOWFREEZE-CH3-FROST, REF-SNOWFREEZE-CH4-COUPLING, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-014 | HPHYS0268 spring snowpack lineage closure: material H1/H7/H39 seasonal `Ep` divergence claims must expose baseline-authoritative `winter -> snowd -> melt` lineage for runtime SWE/depth/density/settle carry state, hourly rain/snow/melt sums, signed `S`, WB13 `RM`, and WB13 `Snow-Water` before returning residual ownership to WB17 `Ep`. Active snowpack execution is governed by runtime snow/frost/thermal triggers and parsed/default snow controls; `snow.options.snow_file_present` may only select parsed-vs-default control provenance and must not gate whether snow processing runs. Non-agricultural HPHYS parity keeps frost disabled while snow remains active. | governance-hold | INV-SNOWFREEZE-009, INV-SNOWFREEZE-010, INV-SNOWFREEZE-011, REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-015 | HPHYS0269/HPHYS0303 baseline winter melt/snowpack invariant: openWEPP snowpack migration must preserve fixed `wepp_260430` daily negative-melt redistribution authority while preserving baseline `snowd.for`/`melt.for` signed hourly melt and rain-on-snow holding-capacity lineage. `melt.for` may emit negative hourly `hrmelt_raw`; only positive raw melt is bounded to available snow during the hourly melt branch. Corrected `winter.for` daily post-processing compares net daily melt (`pstvML + ngtvML`) and, when positive, scales positive hourly melt by `1 + ngtvML/pstvML` before daily routed melt is summed; the archived original `pstvML <= ngtvML` and `1 - ngtvML/pstvML` branch is rejected as bug-compatible archaeology, not active comparator behavior or target physics. `snowd.for` rain-on-snow storage consumes hourly rain into snowpack density while `ρsnew < 350 kg m^-3`; retained rain increases runtime SWE and contributes negative daily `S` just like snowfall accumulation. Residual rain-on-snow that remains in `hrrain` after holding-capacity accounting must be handed to daily `winter.for` post-processing and added into `hrmlt`/`wmelt` before downstream liquid forcing. | hard-fail | REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-LEGACY-SNOWD-RAINSTORE, REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-016 | HPHYS0270 daily snowpack carry-state invariant: H1/H7/H39 spring snowpack residual claims must expose same-day pre-update and post-update runtime SWE, snow depth, snow density, and settle-day-count state, plus their daily deltas, before assigning residual ownership to WB17 `Ep`, aggregate storage, WB13 publication, or a new snowpack production defect. Final-hour state alone is insufficient for closure because `winter -> snowd -> melt` mutates carry state across the whole day and WB13 `RM`/`Snow-Water` publication consumes the day-begin SWE lineage. | governance-hold | INV-SNOWFREEZE-014, INV-SNOWFREEZE-015, REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH5-COUPLING, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-017 | HPHYS0271/HPHYS0272 day-36 melt-forcing lineage invariant: residual claims for the H1 sim-day 36 spurious early-February melt event must expose `melt.for` term-level hourly evidence (`amelt`, `bmelt`, `cmelt`, `dmelt`, signed `wmelt`) and the hourly forcing/branch inputs that produce them, including air temperature, dewpoint/temperature-for-rain term, radiation, cloud fraction, wind, rain, snowfall, canopy cover, wind adjustment, warm-branch activation, and pre/post snowpack state. Radiation-driven melt claims must consume `SC-CLIMATE-001#INV-CLIMATE-013` so `winter.hourly.rad_mj_m2_####` is proven to be `MJ m^-2 h^-1`, not raw Langley-scale magnitude. Evidence that treats day-36 as broad accumulation, WB17 `Ep`, WB13 publication, aggregate storage, negative-melt redistribution, or heuristic radiation clipping is invalid unless term-level melt and climate radiation-unit evidence proves that ownership. | governance-hold | INV-SNOWFREEZE-015, INV-SNOWFREEZE-016, SC-CLIMATE-001#INV-CLIMATE-013, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-018 | HPHYS0283 meltwater infiltration partition invariant: after `winter.for` daily signed-melt redistribution and `snowd.for` rain-retention accounting, routed meltwater (`wmelt`) must be part of the WB12 event liquid supply for both infiltration and runoff partition and must enter WB18 layer storage before percolation/aggregate `watcon` recomputation. The consumer may not add melt only to runoff closure while excluding it from infiltration forcing or layer storage; such exclusion is a hard partition defect and cannot be compensated by WB17 `Ep`, WB13 publication, or aggregate-storage edits. | hard-fail | REF-SNOWFREEZE-LEGACY-WMELT-INFIL, REF-SNOWFREEZE-CH4-COUPLING, REF-SNOWFREEZE-CH5-COUPLING, INV-SNOWFREEZE-015, SC-PERC-001#INV-PERC-016 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-019 | HPHYS0284/HPHYS0285/HPHYS0303/SNOWSCI-S1 corrected negative-melt carry-state invariant: daily snowpack state must distinguish raw signed hourly melt diagnostics from the authoritative runtime storage store. For Stage-1 conservation, runtime snow storage is single-sourced from the post-hourly snow depth/density state (`SWE = Dsnew * ρsnew / 1000`) after the snow-density/melt equations have run; `snow.runtime_swe` is a derived publication/carry value, not an independently debited ledger. When positive and negative hourly `hrmelt` coexist, negative raw melt may affect diagnostic signed-melt lineage but must not create a second SWE debit after the depth/density store has already recorded pack loss. The snowpack routed-melt scalar used for signed `S`, WB12 liquid forcing, and WB13 `RM` must equal the storage loss implied by the authoritative store (`old SWE + snowfall water equivalent + retained rain - new SWE`) within `TOL-SNOWFREEZE-006`, with positive raw melt hours preserving the event timing shape. If the depth/density store exhausts the available pack, runtime SWE/depth/density publish zero by construction and routed snowpack melt is capped to the available water removed from that store. Publishing negative snow storage, silently clamping an independently negative SWE ledger, or allowing `S`/`RM` to diverge from the authoritative storage loss is invalid. This Stage-1 conservation rule supersedes the separate openWEPP SWE-debit interpretation while preserving the protected snow physics-magnitude equations. | hard-fail | REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, INV-SNOWFREEZE-015, INV-SNOWFREEZE-016, REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-020 | HPHYS0287 runtime snow-state fail-closed invariant: projected `snow.runtime_swe`, `snow.runtime_depth_m`, `snow.runtime_density_kg_m3`, and `snow.runtime_settle_day_count` are domain-bearing state surfaces, not optional hints. When any snow option/control/runtime state is projected, the runtime snow-state vector is complete-required; missing vector members must hard-fail instead of defaulting to zero. Material negative or non-finite values must hard-fail before active/inactive snow-coupling branch selection and before WB12/WB14 runoff reconciliation. A branch that labels negative SWE/depth/density as inactive stale state and canonicalizes it to zero is invalid. Bounded within-tolerance zero canonicalization remains allowed only after an explicit non-negative-domain guard has accepted the value within numerical tolerance or after corrected pack-exhaustion handling under `INV-SNOWFREEZE-019`. If no snow option/control/runtime projection is present at all, the hydrology kernel may treat the request as explicit no-projection/no-snow compatibility mode and proceed without synthesizing hidden snow state. | hard-fail | INV-SNOWFREEZE-003, INV-SNOWFREEZE-009, INV-SNOWFREEZE-010, INV-SNOWFREEZE-019, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-021 | HPHYS0288 residual rain-on-snow routed-melt invariant: when `snowd.for` retains only part of a positive rain-on-snow hour because snowpack density reaches `350 kg m^-3`, the residual positive `hrrain` is not a direct-rain-only liquid term. It must be added to `hrmlt(hour,iplane)` during `winter.for` daily post-processing, included in `wmelt(iplane)`, and then consumed by WB12/WB18 through the baseline `wmelt -> fin/smrate` infiltration/runoff lineage. Retained rain still increases runtime SWE and contributes negative daily `S`; released residual rain-on-snow contributes to routed melt forcing and must not be double counted as both residual direct rain and routed melt. | hard-fail | REF-SNOWFREEZE-LEGACY-SNOWD-RAINSTORE, REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE, REF-SNOWFREEZE-LEGACY-WMELT-INFIL, INV-SNOWFREEZE-015, INV-SNOWFREEZE-018, SC-RUNOFFPART-001#INV-RUNOFFPART-018 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-022 | HPHYS0289 WB13 snow publication invariant: the winter producer must expose daily routed `wmelt` and runtime snowpack storage state so WB13 can publish baseline `RM = post-winter rain + wmelt + irrigation` and `Snow-Water = snodpy * densg`. `wmelt` must match the routed snowmelt liquid already used by WB12/WB18 forcing, while `Snow-Water` must remain runtime snowpack storage and not a raw precipitation/SWE-delta reconstruction. | hard-fail | REF-SNOWFREEZE-LEGACY-WB13-RM-SNOW, REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE, REF-SNOWFREEZE-LEGACY-WMELT-INFIL, INV-SNOWFREEZE-021, SC-RUNOFFPART-001#INV-RUNOFFPART-019, SC-WATBAL-001#INV-WATBAL-064 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-023 | HPHYS0290 post-winter rain publication invariant: the snow/winter producer must expose post-winter `rain(iplane)` as `snow.post_winter_rain_m` after winter processing has cleared raw rain, retained rain in snowpack, promoted residual rain-on-snow into `wmelt`, and restored only the warm-rain/no-snow branch. The surface is finite/non-negative and represents the remaining direct-rain liquid term, not routed `wmelt`, retained rain, snowfall, raw precipitation, or a SWE-delta reconstruction. WB13 `RM` must consume this explicit surface alongside `snow.routed_melt_m`. | hard-fail | REF-SNOWFREEZE-LEGACY-WB13-RM-SNOW, REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE, REF-SNOWFREEZE-LEGACY-SNOWD-RAINSTORE, INV-SNOWFREEZE-021, INV-SNOWFREEZE-022, SC-RUNOFFPART-001#INV-RUNOFFPART-020, SC-WATBAL-001#INV-WATBAL-065 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-024 | HPHYS0291 same-day snow publication lifecycle invariant: daily snow publication fluxes (`snow.post_winter_rain_m`, `snow.routed_melt_m`) are producer-owned same-day flux surfaces. The scheduler/runoff reconciliation lifecycle must publish them before WB13 publication for every daily execution, including zero-valued dry/no-snow days, and WB13 consumers must fail closed when a required same-day flux is absent rather than accepting state defaults, stale state, raw precipitation reconstruction, or downstream canonicalization. | hard-fail | INV-SNOWFREEZE-022, INV-SNOWFREEZE-023, SC-RUNOFFPART-001#INV-RUNOFFPART-021, SC-WATBAL-001#INV-WATBAL-066, REF-SNOWFREEZE-LEGACY-WB13-RM-SNOW | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-025 | HPHYS0292 spring snowmelt producer-partition localization invariant: after the HPHYS0291 same-day publication lifecycle is satisfied, H1/H7/H39 spring `Snow-Water`/`RM`/`Q`/storage residual ownership must be assigned from producer-side evidence that separates daily melt magnitude/timing, retained/released rain-on-snow, routed `wmelt`, and WB12 infiltration capacity. Routed `wmelt` remains the producer-owned snow-liquid forcing used by WB12 before residual `Q`; a diagnostic that compares WB13 `RM` only, infers melt from SWE deltas, or treats large spring `Q` as a storage/ET defect without WB12 capacity evidence is invalid. | governance-hold | INV-SNOWFREEZE-018, INV-SNOWFREEZE-021, INV-SNOWFREEZE-024, REF-SNOWFREEZE-LEGACY-WMELT-INFIL, REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE, SC-RUNOFFPART-001#INV-RUNOFFPART-022, SC-WATBAL-001#INV-WATBAL-067 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-026 | HPHYS0293 winter melt magnitude/timing and snowpack depletion invariant: after HPHYS0292 closes WB14 routed-melt capacity and `Q` parity, H1/H7/H39 spring `Snow-Water`/`RM` residual ownership must be classified from producer evidence that distinguishes raw hourly melt magnitude, corrected daily negative-melt redistribution, retained/released rain-on-snow, runtime SWE/depth/density before/after, and WB13 publication. A residual caused by fixed-comparator negative-melt carried-state authority is not a defect to tune away against the archived original comparator; a residual caused by missing term-level evidence, silent melt clamping, mis-unit radiation/rain, or flux/state publication mismatch remains a `HOLD`. | governance-hold | INV-SNOWFREEZE-015, INV-SNOWFREEZE-017, INV-SNOWFREEZE-019, INV-SNOWFREEZE-024, INV-SNOWFREEZE-025, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-CH3-MELT, SC-RUNOFFPART-001#INV-RUNOFFPART-023, SC-WATBAL-001#INV-WATBAL-068 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-027 | HPHYS0296 snow/`RM` producer acceptance invariant: after HPHYS0295 assigns cumulative storage residual dominance to snow/`RM`, material negative raw hourly melt, corrected state-loss/pack-exhaustion lineage, internal snow-state closure, `RM` publication identity, and non-negative runtime snowpack domains are necessary diagnostic evidence but are not sufficient acceptance authority. Before any divergent window leaves the failing set, it must carry a per-window defective-model verdict with (A) mechanistic root cause identified by `file:line` in both openWEPP and `/workdir/wepp-forest_260430_baseline`, (B) reconstruction controlled experiment reproducing the comparator value or delta to named tolerance by injecting the identified `/workdir/wepp-forest_260430_baseline` path into openWEPP and/or applying the corrected path to baseline inputs, (C) independent correctness adjudication using mass/energy conservation, documented WEPP reference equations, corrected-fix derivation/provenance, or external data, and (D) a peer verdict taxonomy that includes `HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, and `UNRESOLVED` per `INV-SNOWFREEZE-039`. `LEGACY-DEFECTIVE` windows must be re-tiered as documented-legacy-defective with reconstruction/correctness evidence linked, never silently deleted; `HARNESS-SURFACE-MISMATCH`, `OPENWEPP-DEFECTIVE`, and `UNRESOLVED` windows remain failing/owned `HOLD` unless explicitly corrected or superseded. Downstream WB17/WB18/WB19/WB13 compensation is invalid in all branches. | governance-hold | INV-SNOWFREEZE-015, INV-SNOWFREEZE-019, INV-SNOWFREEZE-024, INV-SNOWFREEZE-026, INV-SNOWFREEZE-039, SC-RUNOFFPART-001#INV-RUNOFFPART-024, SC-WATBAL-001#INV-WATBAL-071 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-028 | HPHYS0297 snow/`RM` defect-ledger reconstruction invariant: H1/H7/H39 snow/`RM` residuals may be re-tiered only by a row/window ledger that reconstructs the `/workdir/wepp-forest_260430_baseline/src/winter.for:434-448` negative-melt branch (`1 - ngtvML/pstvML`) against openWEPP trace surfaces and compares it with the corrected openWEPP `redistribute_daily_signed_snowmelt` lineage in `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4231-4276`. The ledger must record observed candidate/baseline `RM`, reconstructed baseline-branch `RM`, reconstruction residual to a named tolerance, conservation/correctness rationale, and one verdict per window using the ADR0017 peer taxonomy: `HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, or `UNRESOLVED`. Material negative raw melt without reconstruction closure remains `UNRESOLVED`; unit/surface mismatches remain `HARNESS-SURFACE-MISMATCH`; spring-2016 windows with immaterial negative melt remain producer magnitude/timing owned `HOLD`. | governance-hold | INV-SNOWFREEZE-027, INV-SNOWFREEZE-039, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, SC-RUNOFFPART-001#INV-RUNOFFPART-025, SC-WATBAL-001#INV-WATBAL-072 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-029 | HPHYS0298 paired snow/`RM` lineage partition invariant: each H1/H7/H39 target window (`H1 2013 112-127`, `H1 2014 120-146`, `H1 2016 104-111`, `H7 2013 112-127`, `H7 2014 120-146`, `H7 2016 104-111`, `H39 2013 97-112`, `H39 2014 120-146`, `H39 2016 104-111`) must be classified by paired `/workdir/wepp-forest_260430_baseline` and openWEPP lineage observations before any residual is re-tiered or downstream focus changes. A valid partition ledger must prove baseline observe identity, use the canonical cut-point order `winter-gate -> hourly-forcing -> raw-hourly-melt -> negative-melt-correction -> post-winter-wmelt-rain-flags -> runoff-driver-input -> WB13-RM-Q-identity -> WB17-WB18-WB19-storage-consumers`, publish canonical symbols (`snodpy`, `frdp`, `rain`, `wmelt`, `hrmlt`, `hrrain`, `hrsnow`, `pstvML`, `ngtvML`, `pstvhr`, `snodpt`, `densgt`) with units and source-line provenance, and assign exactly one per-window verdict using the ADR0017 peer taxonomy: `HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, or `UNRESOLVED`. Instrumented baseline traces are diagnostic evidence only and cannot be used when observe-on/off output identity fails. Corrected openWEPP negative-melt safety remains target authority unless a stronger contract-authoritative openWEPP defect is proven under `INV-SNOWFREEZE-039`; no WB17/WB18/WB19/WB13 compensation is authorized by a closed downstream identity. | governance-hold | INV-SNOWFREEZE-028, INV-SNOWFREEZE-027, INV-SNOWFREEZE-039, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, SC-RUNOFFPART-001#INV-RUNOFFPART-026, SC-WATBAL-001#INV-WATBAL-073 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-030 | HPHYS0299 hourly snowfall unit/provenance correction invariant: HPHYS0298 `hourly-forcing` verdicts are suspended until corrected evidence compares canonical pinned-baseline `stmtim.for` `hrsnow` snowfall-depth increments against openWEPP snowfall-depth traces. `winter.for:410-412` remains an HPHYS0298 observe cut-point, not the partition equation; authoritative phase partition is `winter.for:296-300` calling `stmtim.for:43-95`. The openWEPP alias for `hrsnow` parity is `snow.hourly.snowfall_m_####` and runner summary `snow_hourly_snowfall_depth_sum_m`; `snow_hourly_snowfall_water_equiv_sum_m` is a derived density-weighted summary and cannot be used as canonical `hrsnow` parity evidence. Production hourly partition migration, window re-tiering, or downstream WB17/WB18/WB19/WB13 compensation remains invalid when based on the old depth-vs-water-equivalent comparison. | governance-hold | INV-SNOWFREEZE-029, SC-CLIMATE-001#INV-CLIMATE-014, SC-WATBAL-001#INV-WATBAL-074, `/workdir/wepp-forest_260430_baseline/src/winter.for:296-300`, `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95` | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-031 | HPHYS0300 raw hourly melt/post-raw routing lineage invariant: after HPHYS0299 corrected canonical `hrsnow` depth mapping, any H1/H7/H39 raw-hourly-melt or post-raw routed-melt correction must be grounded in term/state evidence that distinguishes `melt.for` raw signed `hrmlt` production from `winter.for` daily post-processing. Valid evidence must include corrected HPHYS0299 depth-vs-depth forcing status, baseline observe identity, raw `hrmlt` deltas, post-raw `wmelt`/routed-melt deltas, positive/negative raw melt totals, released/retained rain-on-snow, and, before production code edits, melt-term and state-input lineage for `amelt`, `bmelt`, `cmelt`, `dmelt`, `hrrain`, `hrtemp`, `tdpt`, `hrad`, `cloudC`, `vwind`, `snodpt`, and `densgt`. The H7 first-2013 post-raw row cannot be accepted as pinned-baseline negative-melt legacy-defective because HPHYS0299 measured `baseline_negative_raw_melt_sum_mm = 0.0`; it remains an openWEPP post-raw/routing hold until term/state evidence proves the source. H39 first-2013 remains a corrected-depth hourly-forcing seam and must not be folded into raw-melt closure. Aggregate `RM`, `Snow-Water`, or storage improvement alone is not production authority. | governance-hold | INV-SNOWFREEZE-030, INV-SNOWFREEZE-029, INV-SNOWFREEZE-026, INV-SNOWFREEZE-019, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, SC-WATBAL-001#INV-WATBAL-075 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-032 | HPHYS0301 H39 rain-release lineage invariant: the H39 first-2013 HPHYS0300 raw-rain forcing comparison is not production forcing authority unless it compares like-for-like raw `stmtim.for` outputs. Baseline residual rain-on-snow evidence after `snowd.for`/`winter.for` mutation must be reconciled against openWEPP `snow_hourly_rain_released_sum_m + snow_post_winter_rain_m`, not raw `snow_hourly_rain_sum_m`. If that reconciliation collapses the raw-rain aggregate delta and no source-line raw `stmtim` producer defect is proven, production forcing edits are prohibited and the row moves to rain-retention/post-raw melt lineage `HOLD` pending paired `melt.for`/`snowd.for` term/state evidence. Instrumented observe tags whose call sites are absent from `/workdir/wepp-forest_260430_baseline/src` are evidence artifacts only, not source-line equation authority. | governance-hold | INV-SNOWFREEZE-031, INV-SNOWFREEZE-030, REF-SNOWFREEZE-LEGACY-SNOWD-RAINSTORE, REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE, SC-WATBAL-001#INV-WATBAL-076 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-033 | HPHYS0302 comparator-surface audit invariant: before any H1/H7/H39 snow/melt producer-defect conclusion, the package must prove each baseline/openWEPP comparator is the same physical quantity in the same units. WB13/WAT `RM` and `Snow-Water` may be accepted only as daily publication/output surfaces; raw `hrmlt` and post-raw `wmelt` aggregate surfaces may localize cut-points but are not term-level melt producer authority. Production edits to melt terms require paired baseline and openWEPP term/state surfaces for `amelt`, `bmelt`, `cmelt`, `dmelt`, `hrrain`, `hrtemp`, `tdpt`, `hrad`, `cloudC`, `vwind`, `snodpt`, and `densgt`. Observe tags or ledgers without source call sites are evidence artifacts and must not be promoted to equation authority. | governance-hold | INV-SNOWFREEZE-032, INV-SNOWFREEZE-031, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-LEGACY-SNOWD-RAINSTORE, SC-WATBAL-001#INV-WATBAL-077 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-034 | HPHYS0309 snow carry/depletion lineage invariant: after HPHYS0308 classifies branch-extra melt-call keys as snow-state carry/depletion holds, closure evidence must compare fixed-comparator prior-day/hour carry state against openWEPP daily runtime and hourly snow state before any branch-predicate, melt-term, or downstream water-balance edit. Required evidence includes fixed-comparator post-hour `snodpt`/`densgt` from `snowd.for` observe identity, prior-day hour-24 carry depth, baseline first same-day zero-depth hour, openWEPP `snow_runtime_depth_before_m`, `snow_runtime_swe_before_m`, `snow_hourly_depth_before_m`, `snow_hourly_depth_after_m`, openWEPP first same-day zero-depth hour, depletion lead hours, and explicit classification of pre-day carry deficit versus same-day depletion. Rows where openWEPP starts the day with materially less snow depth than the fixed comparator or starts snow-free while the fixed comparator carries snow are prior carry-state holds, not branch-predicate or same-hour melt-term production authority. Downstream WB13/WB17/WB18/WB19/WB12 compensation remains invalid. | governance-hold | INV-SNOWFREEZE-031, INV-SNOWFREEZE-033, SC-WATBAL-001#INV-WATBAL-081, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, REF-SNOWFREEZE-CH3-SNOWDENS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-035 | HPHYS0310 prior-day snow carry divergence invariant: after HPHYS0309 routes baseline-extra melt-call keys to prior carry-state holds, closure evidence must reconstruct the first material paired snowpack divergence that precedes each affected H1/H7/H39 hillslope/window/year group. The ledger must compare fixed-comparator `snodpt`/`densgt` from `snowd.for` observe identity against openWEPP daily runtime and hourly snow state from the same trace lane, scan backward/forward through the preceding snow episode, record first divergent day/hour, day-start and day-end depth/SWE/density deltas, raw and routed melt sums, retained/released rain proxies, snowfall depth, corrected negative-melt state-loss indicators, and classify the candidate source lane. Rows where the earliest divergence is already present at initial carry-state projection, accumulation/settling onset, corrected negative-melt state-loss, retained-liquid handling, raw/routed melt magnitude, or incomplete paired evidence remain governance `HOLD`; branch-predicate, same-hour melt-term, WB13 publication, WB17 ET, WB18 storage, WB19 lateral/percolation, and WB12 runoff edits remain invalid until this carry-state source is proven by baseline/source lines. | governance-hold | INV-SNOWFREEZE-034, INV-SNOWFREEZE-031, INV-SNOWFREEZE-019, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, REF-SNOWFREEZE-CH3-SNOWDENS, SC-WATBAL-001#INV-WATBAL-083 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-036 | HPHYS0311 snow carry source-line parity invariant: after HPHYS0310 localizes affected H1/H7/H39 rows to carry-state divergence, closure evidence must compare source-line carry semantics before authorizing producer or downstream edits. The ledger must cite fixed-comparator `winter.for:193` day-start `snodpt=snodpy`, `snowd.for:50-53` hourly initialization from carried `snodpy`/`densg`, `snowd.for:122-139` density-settling equations, `snowd.for:303-312` post-hour carry writes, `infile.for:1361,1466` and `inidat.for:383` initial `snodpy`/`densg` provenance, plus openWEPP runtime seed/carry/update aliases. Day-1 rows must compare prior-year terminal fixed-comparator `snodpt`/`densgt` to openWEPP prior-year terminal runtime depth/density and prove whether the day-1 delta is inherited prior-year terminal state versus a year-boundary projection defect. Density/settling rows must compare previous-hour and current-hour paired states and declare `HOLD` when fixed-observe precision is insufficient to prove an equation defect. Branch-predicate, same-hour melt-term, WB13 publication, WB17 ET, WB18 storage, WB19 lateral/percolation, and WB12 runoff compensation remain invalid unless the ledger proves a source-line-owned openWEPP carry-state defect. | governance-hold | INV-SNOWFREEZE-035, INV-SNOWFREEZE-034, SC-INFILE-MANAGEMENT-001, SC-WATBAL-001#INV-WATBAL-084 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-037 | HPHYS0312 prior-year terminal snowpack lineage invariant: after HPHYS0311 proves six day-1 deltas are inherited from the prior-year terminal snowpack state, closure evidence must scan the complete prior calendar year for each inherited H1/H7/H39 group and locate the first material paired fixed-comparator/openWEPP snowpack divergence that produces the terminal delta. The ledger must compare fixed-comparator `snodpt`/`densgt` from `H305_S_OUT` observe identity against openWEPP hourly depth/density traces, use material tolerances no looser than `0.0005 m` depth and `0.5 kg m^-3` density, record the last within-tolerance paired state before the first material divergence when one exists, cite `snowd.for` source-line lanes for settle-day count, cold settling, snowfall mixing, warm melt/density, rain retention/release, and post-hour carry writeback, and cite openWEPP homologous runtime lanes. Rows whose first material divergence is already present at prior-year day-1 hour-1 remain `year-start-inherited-state-hold`; rows whose first material divergence occurs during cold no-snowfall/no-melt settling remain `settling-depth-update-hold` until full-precision baseline `wdayct` and equation reconstruction are available. No producer edit, branch-predicate edit, WB13 publication edit, WB17/WB18/WB19 storage edit, or WB12 runoff compensation is valid until this lineage proves an openWEPP-owned source-line defect. | governance-hold | INV-SNOWFREEZE-036, INV-SNOWFREEZE-035, SC-WATBAL-001#INV-WATBAL-085 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-038 | HPHYS0313 split-route snowpack settling/carry recursion invariant: after HPHYS0312 splits inherited terminal deltas into three `settling-depth-update-hold` rows and three `year-start-inherited-state-hold` rows, continuation must resolve both routes with source-line evidence before production edits. For settling rows, evidence must instrument or otherwise prove full-precision pinned-baseline `wdayct`, pre-settling `densgy`, computed `setf`, post-settling `densgt`, `snodpt` before/after `snowd.for:122-139`, actual M3 branch selection from `hrsnow`, branch final depth, and the branch input terms at the first material 2013 day 11 hour 11 divergence. If `hrsnow > 0`, evidence must cite and reconstruct the snowing branch at `snowd.for:166-172`, compare baseline `hrsnow` to homologous openWEPP hourly snowfall, and must not classify the post-settling final-depth increment as no-snow `driftg`. If `hrsnow <= 0`, evidence may classify the no-snow drift lane at `snowd.for:145-146` only with branch-gated proof. For year-start rows, evidence must recursively scan the 2014 terminal carry-state chain that feeds 2015 day 1 hour 1 and classify the first material paired divergence or prove that still earlier carry state is inherited. Instrumented observe tags remain diagnostic evidence and must cite canonical `/workdir/wepp-forest_260430_baseline` source lines separately. Branch-predicate, melt-term, WB13 publication, WB17/WB18/WB19 storage, and WB12 runoff compensation remain invalid unless this split-route evidence proves an openWEPP-owned source-line defect. | governance-hold | INV-SNOWFREEZE-037, INV-SNOWFREEZE-036, SC-WATBAL-001#INV-WATBAL-086 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-039 | ADR0017 snow/`RM` comparator verdict invariant: after HPHYS0298-HPHYS0313, H1/H7/H39 snow/`RM` comparator and ledger continuations must treat comparator agreement as an investigation flag, not a target. `OPENWEPP-DEFECTIVE` requires like-for-like unit proof, lineage-stage proof, and independent correctness authority; depth-vs-SWE, raw-vs-released, branch-misclassified, absent-key/default, or other surface mismatches must be classified as `HARNESS-SURFACE-MISMATCH` or `UNRESOLVED`. Criterion C may not be waived. `HOLD` rows must name an owner and follow-on gate, and invalidated prior verdicts must be superseded or retracted in-package before downstream WB13/WB17/WB18/WB19/WB12 compensation or snow-producer edits are authorized. | governance-hold | ADR-0017, INV-SNOWFREEZE-030, INV-SNOWFREEZE-038, SC-WATBAL-001#INV-WATBAL-087 | `[INFERENCE][Static]` |
| INV-SNOWFREEZE-040 | HPHYS0314 ADR0017 snow/`RM` route-ledger reclassification invariant: before HPHYS0315, HPHYS0316, or any snow-producer/downstream water-balance edit proceeds, all HPHYS0298-HPHYS0313 H1/H7/H39 snow/`RM` rows must be reclassified under ADR0017 in a single route ledger. The ledger must preserve the HPHYS0313 split (`3` `hourly-snowfall-input-lineage-hold` rows representing `24` carried rows, `3` `recursive-year-start-inherited-state-hold` rows representing `33` carried rows, `57` total), record source package/route, owner, follow-on gate, ADR0017 verdict, and supersession of stale HPHYS0298 `OPENWEPP-DEFECTIVE` labels. Rows lacking same-unit/same-lineage and independent correctness authority remain `UNRESOLVED` or owned `HOLD`; `HARNESS-SURFACE-MISMATCH` remains reserved for proven surface/lineage mismatches. Production edits and downstream compensation remain invalid from reclassification alone. | governance-hold | INV-SNOWFREEZE-039, INV-SNOWFREEZE-038, SC-WATBAL-001#INV-WATBAL-088 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-041 | HPHYS0315 hourly snowfall input-lineage invariant: H1/H7/H39 spring-2014 settling-route continuation must compare pinned-baseline `stmtim.for` `hrsnow` snowfall-depth increments against openWEPP `snow.hourly.snowfall_m_####` at the same year, Julian day, hour, and trace lane before assigning producer ownership. A production edit requires paired source-line evidence over the fixed-baseline `winter.for -> stmtim.for -> snowd.for` path and the openWEPP `SIMIMPL28 -> SIMIMPL29` path for `rain`, `stmdur`, `wntdur`, `wnttim`, `hrtemp`, `rst`, `hrsnow`, `hrrain`, active-hour interval, and rain/snow branch choice. The known HPHYS0313 rows where baseline `hrsnow = 0.0007454545120708644 m` at 2013 day 11 hour 11 while openWEPP homologous snowfall depth is `0.0 m` remain `UNRESOLVED` and owned `HOLD` when paired input-surface closure is absent. Existing source-code resemblance alone is insufficient proof. Snow-drift, WB13/WB17/WB18/WB19/WB12, melt-term, or branch-predicate compensation remains invalid from this classification alone. | governance-hold | INV-SNOWFREEZE-040, INV-SNOWFREEZE-038, INV-SNOWFREEZE-030, SC-CLIMATE-001#INV-CLIMATE-014, SC-WATBAL-001#INV-WATBAL-089 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-042 | HPHYS0316 2013 terminal carry-recursion invariant: H1/H7/H39 spring-2016 rows routed by HPHYS0313 to `recursive-year-start-inherited-state-hold` must recurse through the 2013 terminal snowpack state feeding 2014 day 1 hour 1 before any producer or downstream edit. The recursion ledger must prove continuity from 2014 day 1 hour 1 back to the matching 2013 terminal state (`H1` depth delta about `0.013144251 m`, `H7` about `0.015279466 m`, `H39` about `0.014797909 m`), then connect the inherited terminal delta to the first material 2013 day 11 hour 11 branch-gated positive-`hrsnow` divergence already localized by HPHYS0313. The `33` carried spring-2016 rows therefore remain `UNRESOLVED`/owned `HOLD` under the same 2013 hourly snowfall input-surface parity blocker unless paired input-surface evidence proves a different source-owned lane. Production edits, snow-drift migration, branch-predicate/melt-term edits, and WB13/WB17/WB18/WB19/WB12 compensation remain invalid from inherited terminal carry alone. | governance-hold | INV-SNOWFREEZE-041, INV-SNOWFREEZE-040, INV-SNOWFREEZE-038, INV-SNOWFREEZE-037, SC-WATBAL-001#INV-WATBAL-090 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-043 | HPHYS0317 paired hourly snowfall input-surface closure invariant: the `24` HPHYS0315 spring-2014 rows and `33` HPHYS0316 spring-2016 inherited rows form one `57`-row route through the 2013 day 11 hour 11 positive-`hrsnow` key. Snow/freeze producer ownership requires a paired input-surface ledger that proves the fixed-baseline and openWEPP `stmtim` controlling values (`rain`, `stmdur`, rounded `wntdur`, adjusted `wnttim`, `hrtemp`, `rst`, `hrsnow`, `hrrain`, active interval membership, and branch choice) at that same key. If the ledger cannot publish those values on both sides, the route remains ADR0017 `UNRESOLVED` under `paired-input-surface-instrumentation-hold`; inherited carry, source-code resemblance, aggregate snowfall totals, or observed snowpack deltas cannot authorize production snow producer, drift, melt-term, branch-predicate, WB13, WB17, WB18, WB19, or WB12 edits. | governance-hold | INV-SNOWFREEZE-042, INV-SNOWFREEZE-041, INV-SNOWFREEZE-040, SC-CLIMATE-001#INV-CLIMATE-015, SC-WATBAL-001#INV-WATBAL-091 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-044 | HPHYS0318 `stmtim` control-surface trace invariant: the combined `57` carried rows remain one ADR0017 `UNRESOLVED` route unless paired fixed-baseline/openWEPP `stmtim` control surfaces are present at the 2013 day 11 hour 11 key. OpenWEPP must publish trace maps for `rain`, `stmdur`, rounded `wntdur`, adjusted `wnttim`, `hrtemp`, `rst`, `hrrain`, `hrsnow`, active interval membership, rain branch, and snow branch via `snow.hourly.stmtim.*_####`; those maps are observability for the existing SIMIMPL28 partition result, not a snow-producer correction. If fixed-baseline paired observe values remain unavailable after OpenWEPP instrumentation, the route stays `paired-fixed-baseline-stmtim-observe-hold`, and snow producer, drift, melt-term, branch-predicate, WB13, WB17, WB18, WB19, or WB12 edits remain invalid. | governance-hold | INV-SNOWFREEZE-043, SC-CLIMATE-001#INV-CLIMATE-016, SC-WATBAL-001#INV-WATBAL-092, `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95` | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-045 | HPHYS0319 fixed-baseline `stmtim` observe recovery invariant: snow/freeze continuation must recover fixed-baseline H1/H7/H39 2013 day 11 hour 11 `stmtim` observe values for `rain`, `stmdur`, rounded `wntdur`, adjusted `wnttim`, `hrtemp`, `rst`, `hrrain`, `hrsnow`, active interval membership, rain branch, and snow branch from the pinned baseline commit before assigning snow producer ownership. The paired ledger must compare those fixed-baseline values to regenerated OpenWEPP `snow.hourly.stmtim.*_0011` diagnostics, preserve the combined `57` carried rows, and distinguish absent/extra active interval, rain-vs-snow branch, and value-magnitude deltas. Temporary observe-only Fortran instrumentation is diagnostic evidence only and must be patch-recorded against source lines; it does not replace canonical `SC-*` authority or prove equations by itself. If the ledger does not establish same-unit same-lineage source-line-owned OpenWEPP defect authority plus independent correctness authority, the route remains ADR0017 `UNRESOLVED`/`HOLD`, and snow producer, drift, melt-term, branch-predicate, WB13, WB17, WB18, WB19, or WB12 edits remain invalid. | governance-hold | INV-SNOWFREEZE-044, SC-CLIMATE-001#INV-CLIMATE-017, SC-WATBAL-001#INV-WATBAL-093, `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95`, `/workdir/wepp-forest_260430_baseline/src/winter.for:292-300`, `/workdir/wepp-forest_260430_baseline/src/wepp_observe.for` | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-046 | HPHYS0320 `stmtim` start-time snow/freeze closure invariant: snow/freeze producer ownership for the combined `57` carried rows is source-line proven when OpenWEPP SIMIMPL28 normalizes finite `wnttim < 1.0` to `1.0` before `stmtim` active interval and branch selection, matching pinned-baseline `winter.for:206-235`. For H1/H7/H39 2013 day 11 hour 11 this closes the active-interval and snow-branch divergence (`wntdur = 11`, `wnttim = 1`, active interval `1`, snow branch `1`, `hrsnow ~= 0.00074545 m`) when paired rerun evidence matches HPHYS0319 fixed-baseline observe values. Snow drift, melt-term, WB13, WB17, WB18, WB19, or WB12 compensation remains invalid for this route unless residual paired evidence proves a different source lane. | hard-fail | REF-SNOWFREEZE-LEGACY-WNTTIM-MIN, SC-CLIMATE-001#INV-CLIMATE-018, SC-WATBAL-001#INV-WATBAL-094 | `[DIRECT][Static] + [INFERENCE][Static]` |

### HPHYS0298 Porting-Fidelity Authority

For HPHYS0298 `hourly-forcing` verdicts, `OPENWEPP-DEFECTIVE` means a
porting-fidelity defect against an un-impeached baseline precipitation-phase
partition routine, not a generic "differs from baseline" claim. The paired
ledger identified `hrsnow` and, for H39 first-2013, `hrrain`/`hrsnow` as the
first divergent symbols upstream of the corrected negative-melt defect family.
HPHYS0299 supersedes the direct migration inference until corrected
depth-vs-depth evidence is published: the authoritative pinned-baseline
partition path is `/workdir/wepp-forest_260430_baseline/src/winter.for:296-300`
calling `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95`, while
`/workdir/wepp-forest_260430_baseline/src/winter.for:410-412` is an observe
cut-point used by HPHYS0298 diagnostics. Paired instrumented baseline
observation using
`/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill` plus
observe-off/observe-on identity is an available comparator capability for this
lineage and supersedes any stale assertion that no reference binary exists for
this diagnostic class.

### HPHYS0299 Unit/Provenance Correction Authority

Canonical `hrsnow` is a snowfall-depth symbol. HPHYS0299 diagnostics must map
it to openWEPP `snow.hourly.snowfall_m_####` and summary
`snow_hourly_snowfall_depth_sum_m`. The derived
`snow_hourly_snowfall_water_equiv_sum_m` surface remains valid for SWE
diagnostics, but it is not canonical `hrsnow` parity evidence. Any production
hourly precipitation-partition migration based on the old depth-vs-water-
equivalent comparison remains `HOLD` until corrected paired evidence proves a
remaining openWEPP producer defect.

### HPHYS0300 Raw/Post-Raw Melt Lineage Authority

HPHYS0300 continuation starts from the corrected HPHYS0299 ledger, not the
superseded HPHYS0298 water-equivalent mismatch. The raw melt target is
`melt.for` production of signed hourly `hrmlt`; the post-raw target is
`winter.for` daily redistribution plus residual rain-on-snow promotion into
`wmelt`. A production correction is invalid unless the package can identify
the first divergent term/state source with source-line provenance and preserve
the fixed `wepp_260430` comparator negative-melt branch. Rows with only
aggregate `hrmlt`/`wmelt` deltas remain diagnostic `HOLD` and require
additional paired instrumentation rather than downstream compensation.

Sufficiency and forcing-function closure: paired term/state evidence is
sufficient to leave `HOLD` when it identifies the first divergent source as one
named baseline/openWEPP term or state input (`amelt`, `bmelt`, `cmelt`,
`dmelt`, `hrrain`, `hrtemp`, `tdpt`, `hrad`, `cloudC`, `vwind`, `snodpt`,
or `densgt`) with source-line provenance, unit reconciliation, and unchanged
upstream forcing status for that row/window. Once that criterion is satisfied,
the owning work-package must implement the baseline-authoritative correction or
record a concrete blocking invariant; it must not request a further
diagnostic-only package for the same isolated source. H39 first-2013 remains an
earned corrected-depth hourly-forcing correction lane and does not wait on
raw-melt term instrumentation when its forcing root cause is independently
localized.

### HPHYS0301 H39 Rain-Release Reclassification Authority

HPHYS0301 reconciles the H39 first-2013 forcing lane by comparing baseline
residual rain-on-snow evidence with openWEPP released plus post-winter rain
surfaces. The HPHYS0300 raw-rain aggregate delta is not sufficient production
authority when baseline evidence is post-`snowd.for` residual `hrrain` but
openWEPP evidence is raw `snow_hourly_rain_sum_m`. A valid HPHYS0301 package
must either identify a source-line raw forcing defect in `stmtim.for`-equivalent
openWEPP code or record `HOLD` with the H39 row reclassified to
rain-retention/post-raw melt lineage. The reclassified row still requires
paired `melt.for`/`snowd.for` term/state evidence before any snow producer edit.

### HPHYS0302 Comparator-Surface Audit Authority

HPHYS0302 is a surface-validity gate, not a production-physics package. A
residual may advance only when the baseline and openWEPP evidence surfaces name
the same physical quantity in the same units. `RM` and `Snow-Water` comparisons
are valid daily WB13/WAT publication-surface checks; raw `hrmlt` and post-raw
`wmelt` comparisons are valid aggregate cut-point checks. None of those
aggregate/output surfaces proves a term-level `melt.for` producer defect. A
term-level correction requires paired baseline/openWEPP surfaces for `amelt`,
`bmelt`, `cmelt`, `dmelt`, and their state/forcing inputs before code edits.

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-SNOWFREEZE-001` | runtime | Melt branch validator and exporter (`hrmelt` bounded to `[0, Dsavail]`) | Explicit branch applies authoritative bounds; typed hard error if post-branch export remains out-of-domain | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-002` | runtime | Melt-density gate before routing `hrmelt` to DISAG/infiltration | Typed hard error if liquid melt is exported while `ρsnew < 350 kg m^-3` | Tier-A gate | `[DIRECT][Static]` |
| `INV-SNOWFREEZE-003` | runtime | Snow state domain validator after each hourly update | Typed hard error on negative depths/densities, violated zero-depth/zero-density rule, or density cap breach | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-004` | runtime | Branch-specific snow-density/depth equation checks for active snowfall/settling/melt/mixed branches | Typed hard error on inconsistent active-branch closure | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-005` | runtime | Daily/hourly precipitation phase-partition branch validator | Typed hard error on partition logic mismatch | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-SNOWFREEZE-006` | runtime | Frost routine heat-flow equation and layered conductivity checks | Typed hard error on invalid heat-flow domain or layered conductivity setup | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-SNOWFREEZE-007` | runtime | Winter payload boundary validator (hourly + daily coupling fields) | Typed hard error on missing/invalid required payload fields | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-008` | governance | Review/disposition/verification promotion check | Promotion `HOLD` when drift-active claims appear without authority-backed implementation status update | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-009` | runtime | Winter-routine trigger-condition branch gate | Typed hard error on silent skip when trigger condition is true | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-010` | runtime | CLIM05 snow-control adapter + hydrology snow-coupling branch | Typed hard error on missing/non-finite/domain-invalid `snow.options.*` controls or invalid `S`/`snow.runtime_swe` closure | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-011` | runtime | CLIM05 partition branch + WB13/hydout publication mapper | Typed hard error on cold-day liquid-partition mismatch (`RM > 0` when `Tmax <= rst`) or publication of `Snow-Water` from static `snow.options.ssd` instead of runtime SWE state | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-012` | runtime | Winter/frost dispatch gate and routine-chain sequencing validator (`winter`, `frostN`, `frwatc`, `frzng`, `frznw`) | Typed hard error on active-branch dispatch mismatch, missing handoff call sequencing, or silent bypass | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-013` | runtime | Frozen-soil conductivity coupling validator (`frsoil` + `getFreezeCond`) | Typed hard error on missing land-use coefficient selection, invalid frozen-zone conductivity aggregation, or non-finite seam export | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-014` | governance | H1/H7/H39 material spring snowpack classifier spanning runtime snow carry state, hourly rain/snow/melt totals, signed `S`, WB13 `RM`, and WB13 `Snow-Water` | Explicit `HOLD` when spring `Ep` residual ownership is asserted without snowpack lineage evidence; no WB17 `Ep` compensation or sidecar-presence gate substitution | HPHYS0268 spring snowpack gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-015` | runtime + governance | Winter melt/snowpack kernel and HPHYS0269 classifier spanning signed `hrmelt_raw`, redistributed hourly melt, rain retained in snowpack, runtime SWE/depth/density, signed `S`, residual liquid rain, WB13 `RM`, and WB13 `Snow-Water` | Typed hard error on missing/non-finite/domain-invalid retained-rain or snowpack state; explicit `HOLD` when full snowpack migration evidence is incomplete; no empirical melt or `Ep` compensation edits | HPHYS0269 winter melt/snowpack baselining gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-016` | governance | Daily carry-state classifier spanning pre-day/post-day SWE, depth, density, settle count, deltas, signed `S`, WB13 `RM`, and WB13 `Snow-Water` | Explicit `HOLD` when residual ownership is asserted without daily carry-state evidence or when final-hour state is substituted for day-begin publication state | HPHYS0270 daily snowpack state gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-017` | governance | H1 day-36 melt-term/hourly-forcing classifier spanning `amelt/bmelt/cmelt/dmelt`, raw/redistributed melt, forcing inputs, warm-branch flags, and `SC-CLIMATE-001#INV-CLIMATE-013` radiation-unit closure | Explicit `HOLD` when day-36 residual ownership is asserted without melt-term, hourly-forcing, and radiation-unit evidence; no WB17/storage/WB13/negative-melt compensation edits | HPHYS0271/HPHYS0272 day-36 melt-forcing gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-018` | runtime + governance | Meltwater infiltration partition classifier spanning redistributed melt, retained rain, residual liquid rain, WB12 infiltration, WB12 runoff, signed `S`, WB13 `RM`, and WB13 `Total-Soil` | Typed hard error / explicit `HOLD` when meltwater is routed only to runoff closure or when spring storage-collapse ownership is asserted without melt/infiltration/runoff evidence | HPHYS0283 spring partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-019` | runtime + governance | Negative-melt carry-state classifier spanning positive/negative hourly melt totals, routed redistributed melt, runtime SWE/depth/density deltas, signed `S`, WB13 `RM`, and `Snow-Water` | Typed hard error / explicit `HOLD` when carried snowpack state is recomputed from routed net melt alone under mixed positive/negative daily melt, when pack exhaustion publishes negative SWE/depth, when carried state-loss overdraw exceeds `0.005 m` water equivalent, or when spring snow-retention ownership is asserted without this evidence | HPHYS0284/HPHYS0285 spring snow-retention gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-020` | runtime + governance | Runtime snow-state domain guard before active/inactive snow branch selection, WB12 same-pass infiltration lineage, and WB14 runoff reconciliation | Typed hard error / explicit `HOLD` when material negative or non-finite snow runtime state is canonicalized to inactive zero state instead of failing before liquid partition | HPHYS0287 snow liquid partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-023` | runtime + governance | Post-winter direct-rain producer seam between snow/winter partitioning and WB13 `RM` publication | Typed hard error / explicit `HOLD` when post-winter rain is missing, negative, non-finite, inferred downstream, or double counted with routed `wmelt` | HPHYS0290 WB13 post-winter-rain publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-024` | runtime + governance | Daily producer/scheduler/WB13 snow publication lifecycle for `snow.post_winter_rain_m` and `snow.routed_melt_m` | Typed hard error / explicit `HOLD` when same-day flux publication is absent or masked by state/default reconstruction before WB13 | HPHYS0291 snow publication lifecycle gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-025` | governance | H1/H7/H39 spring producer-partition classifier spanning melt magnitude/timing, rain retention/release, routed `wmelt`, WB12 infiltration, `Q`, WB13 `RM`, and storage | Explicit `HOLD` when residual ownership is asserted without producer-side partition evidence or when routed melt bypasses WB12 capacity classification | HPHYS0292 spring snowmelt/infiltration capacity gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-026` | governance | H1/H7/H39 spring snow depletion classifier spanning raw/redistributed melt, retained/released rain, runtime SWE/depth/density before/after, corrected negative-melt state authority, and WB13 `RM`/`Snow-Water` | Explicit `HOLD` when residual ownership is asserted without term-level melt/depletion evidence or when corrected negative-melt authority is treated as a pinned-baseline bug to reproduce | HPHYS0293 winter melt magnitude/timing gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-027` | governance | HPHYS0296 snow/`RM` defect-ledger classifier across raw positive/negative melt, corrected state loss, routed melt, post-winter rain, retained/released rain, runtime SWE closure, WB13 publication identity, reconstruction evidence, and independent correctness adjudication | Explicit `HOLD` unless a per-window defective-model verdict is proven; correlation plus internal closure is insufficient; documented legacy-defective windows are re-tiered, never deleted; downstream compensation remains invalid | HPHYS0296 snow/`RM` producer acceptance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-028` | governance | HPHYS0297 per-window reconstruction ledger comparing pinned-baseline negative-melt branch reconstruction against corrected openWEPP trace lineage | Explicit `HOLD` when reconstruction residual is outside named tolerance, source-line provenance is missing, or verdict is absent; no row is silently removed | HPHYS0297 snow/`RM` defect-ledger gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-029` | governance | HPHYS0298 paired lineage partition ledger spanning baseline observe identity, target-window traces, first-divergent cut-point assignment, source-line provenance, and final verdicts | Explicit `HOLD` when observe identity fails, any target window lacks a first-divergent cut-point/verdict, or downstream compensation is asserted without upstream partition closure | HPHYS0298 paired snow/`RM` lineage partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-030` | governance | HPHYS0299 corrected paired partition ledger and source-provenance audit | Explicit `HOLD` when canonical `hrsnow` is mapped to water-equivalent snowfall, when `stmtim.for` provenance is absent, or when HPHYS0298 production-migration authority is reused without corrected depth-vs-depth evidence | HPHYS0299 unit/provenance correction gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-031` | governance | HPHYS0300 raw/post-raw melt lineage ledger spanning corrected HPHYS0299 forcing status, raw `hrmlt`, post-raw `wmelt`, signed melt totals, rain retention/release, and required melt-term/state-input evidence before production edits | Explicit `HOLD` when raw/post-raw ownership is asserted from aggregate deltas alone, when H7 first-2013 is misclassified as legacy-defective without baseline negative raw melt, or when H39 first-2013 forcing is merged into raw-melt closure | HPHYS0300 raw hourly melt/post-raw routing gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-032` | governance | HPHYS0301 H39 residual-rain/release ledger comparing baseline residual rain-on-snow to openWEPP released plus post-winter rain before forcing edits | Explicit `HOLD` when raw-rain aggregate deltas are used as forcing authority, observe tags without source call sites are treated as equations, or H39 is moved to production edit without paired term/state evidence | HPHYS0301 H39 rain-release reclassification gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-033` | governance | HPHYS0302 comparator-surface audit spanning `RM`, `Snow-Water`, raw `hrmlt`, post-raw `wmelt`, and missing melt term/state surfaces | Explicit `HOLD` when aggregate/output deltas are treated as term-level producer authority or when paired baseline/openWEPP term surfaces are missing | HPHYS0302 comparator-surface audit gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-034` | governance | HPHYS0309 carry/depletion lineage ledger spanning fixed-comparator prior-day/hour post-state, openWEPP day-start runtime state, hourly before/after depth, and same-day depletion lead | Explicit `HOLD` when branch-extra rows are explained by pre-day carry deficits, prior-day openWEPP meltout, incomplete state evidence, or downstream compensation is asserted | HPHYS0309 snow carry/depletion lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-035` | governance | HPHYS0310 prior-day snow carry divergence ledger spanning affected H1/H7/H39 groups, paired fixed-comparator/openWEPP hourly snow state, daily start/end state, melt/rain/snowfall aggregates, and first material divergence classification | Explicit `HOLD` when first divergence is initial carry-state projection, accumulation/settling onset, corrected negative-melt state loss, retained-liquid handling, raw/routed melt magnitude, incomplete evidence, or downstream compensation is asserted before source-line proof | HPHYS0310 prior-day carry divergence gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-036` | governance | HPHYS0311 source-line carry-state parity ledger spanning prior-year terminal carry, day-1 projection, settling/depth equations, initial `snodpy`/`densg` provenance, and openWEPP runtime aliases | Explicit `HOLD` when day-1 deltas are inherited from prior-year terminal state, fixed-observe precision is insufficient for a settling equation defect, source-line citations are absent, or downstream compensation is asserted before openWEPP-owned carry-state proof | HPHYS0311 snow carry source-line parity gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-037` | governance | HPHYS0312 prior-year terminal snowpack lineage ledger scanning the full prior calendar year for first material inherited terminal-state divergence | Explicit `HOLD` when divergence is year-start inherited, cold settling/depth update without full-precision `wdayct`, incomplete source-line evidence, or downstream compensation before source-owned proof | HPHYS0312 prior-year terminal snowpack lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-038` | governance | HPHYS0313 split-route ledger reconstructing full-precision 2013 settling/depth equations, branch-gated `hrsnow`/snowfall input lineage, and recursively scanning 2014 terminal carry-state lineage for 2016-target inherited rows | Explicit `HOLD` when full-precision settling inputs are missing, branch-gated snowfall/drift lineage is unresolved, reconstruction residuals do not prove openWEPP source ownership, earlier carry-state inheritance remains unresolved, or downstream compensation is asserted | HPHYS0313 split-route snowpack settling/carry recursion gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-039` | governance | ADR0017 comparator-verdict gate requiring same-unit/same-lineage proof, independent correctness authority, symmetric verdict taxonomy, owned `HOLD`, and supersession/retraction of invalidated HPHYS0298-era verdicts | Explicit `HOLD` when `OPENWEPP-DEFECTIVE` is asserted from comparator disagreement alone, criterion C is waived, surface mismatches are not classified as `HARNESS-SURFACE-MISMATCH` or `UNRESOLVED`, or a `HOLD` row lacks owner/follow-on gate | ADR0017 comparator-flag snow/`RM` ratification gate | `[INFERENCE][Static]` |
| `INV-SNOWFREEZE-040` | governance | HPHYS0314 consolidated ADR0017 route ledger preserving HPHYS0313 split-route counts, source routes, verdict taxonomy, owned follow-on gates, and supersession of stale HPHYS0298 verdicts | Explicit `HOLD` when any carried row is omitted, stale `OPENWEPP-DEFECTIVE` labels remain authoritative, `HOLD` lacks owner/follow-on package, or production edits/downstream compensation are asserted from reclassification alone | HPHYS0314 ADR0017 route-ledger gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-041` | governance | HPHYS0315 hourly snowfall input-lineage ledger pairing fixed-baseline `stmtim.for` `hrsnow` depth with openWEPP `snow.hourly.snowfall_m_####` and all controlling hourly input surfaces before producer edits | Explicit `HOLD` when paired `rain`/`stmdur`/`wntdur`/`wnttim`/`hrtemp`/`rst`/`hrsnow`/`hrrain` source-line evidence is incomplete, code resemblance is treated as proof, or production edits/downstream compensation are asserted from unresolved snowfall-depth mismatch alone | HPHYS0315 hourly snowfall input-lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-042` | governance | HPHYS0316 2013 terminal carry-recursion ledger connecting spring-2016 inherited year-start rows to the 2013 terminal state and the first material 2013 day 11 hour 11 hourly snowfall input blocker | Explicit `HOLD` when 2014 day-1/2013 terminal continuity is missing, the first 2013 material lane is not connected, HPHYS0317 ownership is absent, or inherited carry alone is used to authorize producer/downstream edits | HPHYS0316 2013 terminal carry-recursion gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-043` | governance | HPHYS0317 paired hourly snowfall input-surface ledger joining the `24` spring-2014 and `33` spring-2016 carried rows at the 2013 day 11 hour 11 `hrsnow` key | Explicit `HOLD` when controlling input surfaces are missing on either side, source-code resemblance is treated as proof, row totals are incomplete, or production/downstream edits are asserted before ADR0017 same-unit same-lineage proof and independent correctness authority | HPHYS0317 paired hourly snowfall input-surface gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-044` | runtime + governance | HPHYS0318 OpenWEPP `stmtim` trace maps plus paired fixed-baseline observe continuation for the `57` carried-row route | Explicit `HOLD` when OpenWEPP diagnostics are absent, fixed-baseline paired observe values remain unavailable, or production/downstream edits are asserted from instrumentation alone | HPHYS0318 `stmtim` control-surface trace gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-045` | governance | HPHYS0319 fixed-baseline `stmtim` observe recovery and paired classification ledger for H1/H7/H39 2013 day 11 hour 11 | Explicit `HOLD` when fixed-baseline observe recovery is absent, not paired with OpenWEPP `stmtim` traces, lacks source-line ownership, or is used to authorize producer/downstream edits without independent correctness authority | HPHYS0319 fixed-baseline `stmtim` observe recovery gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-046` | runtime + governance | HPHYS0320 SIMIMPL28 start-time normalization and paired H1/H7/H39 trace rerun | Typed hard error for non-finite start time; finite below-hour-one starts normalize to `1.0` before snow/rain branch evaluation; `HOLD` only for residual paired divergence assigned to a named follow-on lane | HPHYS0320 `stmtim` start-time closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow WEPP Chapter-3 notation and lineage
names by default. SIMIMPL27 ratifies concrete openWEPP boundary/API aliases for
winter snow/freeze migration scope. Existing typed aliases use
`HillslopeProductionStateSymbol` / `HillslopeProductionFluxSymbol` mappings;
hourly-internal aliases that are not yet produced on runtime writeback are
reserved under explicit `snow.hourly.*` / `winter.hourly.*` / `frost.hourly.*`
namespaces for staged SIMIMPL28/SIMIMPL29/SIMIMPL32 implementation.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `Dsold` | `snow.hourly.depth_before_m` | hourly snow-depth state surface | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Dsnew` | `snow.hourly.depth_after_m` | hourly snow-depth state surface | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Dsavail` | `snow.hourly.depth_available_m` | melt upper-bound branch pre-hour state | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ρsold` | `snow.hourly.density_before_kg_m3` | hourly snow-density state surface | `kg m^-3` -> `kg m^-3` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ρsnew` | `snow.hourly.density_after_kg_m3` | hourly snow-density state surface | `kg m^-3` -> `kg m^-3` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrsnow` | `snow.hourly.snowfall_m` | hourly snowfall input | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `faldr`, `grdri` | `snow.drift.faldr_m`, `snow.drift.grdri_m` (inactive/governance-only) | drift formulation provenance only while drift is inactive | `m` -> `m` | `[DIRECT][Static]` |
| `hrmelt` | `snow.hourly.melt_m` | hourly redistributed melt forcing and trace family after daily winter post-processing | `m` -> `m`; final routed values are finite/non-negative and daily routed melt is the summed coupling quantity | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrmelt_raw` | `snow.hourly.melt_raw_m` | signed hourly melt before daily redistribution | `m` -> `m`; negative values are valid diagnostics before daily post-processing | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrain` | `snow.hourly.rain_m` | hourly rainfall forcing before snowpack holding-capacity retention | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrain_store` | `snow.hourly.rain_retained_m` | hourly rain retained in snowpack holding capacity | `m` -> `m`, finite and non-negative | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrain_release` | `snow.hourly.rain_released_m` | residual rain-on-snow added into final `hrmelt`/`wmelt` after holding-capacity accounting | `m` -> `m`, finite and non-negative | `[DIRECT][Static] + [INFERENCE][Static]` |
| `rain(iplane)` after winter processing | `snow.post_winter_rain_m` | daily direct-rain depth remaining after winter clearing/restoration, rain retention, and rain-on-snow promotion into routed `wmelt` | `m` -> `m`, finite and non-negative | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Thr` | `winter.hourly.air_temp_c` | hourly thermal forcing surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Thra` | `winter.hourly.surface_temp_c` | hourly adjusted thermal forcing surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Tmax`, `Tmin` | `tmax`, `tmin` (`HillslopeProductionStateSymbol::{Wb14Tmax,Wb14Tmin}`) | daily thermal forcing surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrad` | `winter.hourly.rad_mj_m2`, `winter.hourly.rad_mj_m2_{idx4}` | hourly radiation surface | `MJ m^-2 h^-1` preserved at registry boundary; legacy term-level conversion remains explicit | `[DIRECT][Static] + [INFERENCE][Static]` |
| `cancov`, `clouds` | `cancov` (`HillslopeProductionStateSymbol::Wb15PlantCancov`), `winter.hourly.cloud_fraction` | melt and surface-temperature modifiers | `fraction` -> `fraction` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Qsrf`, `Quf`, `Ksrf` | `frost.hourly.qsrf_w_m2`, `frost.hourly.quf_w_m2`, `frost.hourly.ksrf_w_m_k` | frost heat-flow bookkeeping surface | `W m^-2` / `W m^-1 degC^-1` unchanged | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Snowd`, `Resd`, `Tilld`, `Utilld` | `frost.hourly.snow_depth_m`, `frost.hourly.residue_depth_m`, `frost.hourly.tilled_frozen_depth_m`, `frost.hourly.untilled_frozen_depth_m` | layered conductivity state inputs | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Dfrost`, `Dthaw` | `frost.runtime_dfrost`, `frost.runtime_dthaw` (`HillslopeProductionStateSymbol::{Wb14FrostRuntimeDfrost,Wb14FrostRuntimeDthaw}`) | hourly frost/thaw depth boundary outputs | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `frdp` | `frost.runtime_frdp_m`, `hillslope_wat.frdp` | active frost-front depth runtime state and WAT publication surface | runtime `m`; WAT publication `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `S` | `S` (`HillslopeProductionFluxSymbol::Wb12SnowCouplingS`) | daily snow-water term in water-balance closure | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Snow-Water` (WB13/hydout publication surface) | `hillslope_wat.Snow-Water`, `hillslope_wat.Snow-Water:mm`; derived alias from `snow.runtime_swe` at publication boundary | replay/output storage-state publication | runtime SWE (`m`) is converted to published snow-water storage units `mm` at boundary without sidecar-control substitution | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow.options.rst`, `snow.options.newsnw`, `snow.options.ssd`, `snow.options.snow_file_present` | identity (`HillslopeProductionStateSymbol::{Wb14SnowRst,Wb14SnowNewsnw,Wb14SnowSsd,Wb14SnowFilePresent}`) | parsed snow sidecar controls projected to runtime seam | scalar controls preserved; `snow_file_present` in `{0,1}` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_runtime_swe` | `snow.runtime_swe` identity (`HillslopeProductionStateSymbol::Wb14SnowRuntimeSwe`) | runtime snow-water-equivalent storage state | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_routed_melt`, `snow_post_winter_rain` | `snow.routed_melt_m`, `snow.post_winter_rain_m` | daily snow/liquid coupling publication surfaces | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_runtime_depth`, `snow_runtime_density`, `snow_runtime_settle_day_count` | `snow.runtime_depth_m`, `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count` | runtime snow carry-state surfaces | `m`, `kg m^-3`, and `count` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `winter_air_temp_hourly`, `winter_dewpoint_hourly`, `winter_wind_hourly`, `winter_cloud_fraction_hourly` | `winter.hourly.air_temp_c_{idx4}`, `winter.hourly.dewpoint_c_{idx4}`, `winter.hourly.wind_m_s_{idx4}`, `winter.hourly.cloud_fraction_{idx4}` | registry-owned hourly winter forcing surfaces | `degC`, `degC`, `m s^-1`, and `dimensionless` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_rain`, `snow_hourly_rain_retained`, `snow_hourly_snowfall` | `snow.hourly.rain_m_{idx4}`, `snow.hourly.rain_retained_m_{idx4}`, `snow.hourly.snowfall_m_{idx4}` | registry-owned hourly precipitation/snowpack forcing surfaces | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_stmtim_control_surfaces` | `snow.hourly.stmtim.{rain_m,stmdur_s,wntdur_h,wnttim_h,hrtemp_c,rst_c,hrrain_m,hrsnow_m,active_interval,rain_branch,snow_branch}_{idx4}` | HPHYS0318 trace-grade OpenWEPP SIMIMPL28 precipitation partition control surfaces | depths `m`, durations `s`/`h`, temperatures `degC`, flags `0/1` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_depth`, `snow_hourly_density` | `snow.hourly.depth_before_m_{idx4}`, `snow.hourly.depth_available_m_{idx4}`, `snow.hourly.depth_after_m_{idx4}`, `snow.hourly.density_before_kg_m3_{idx4}`, `snow.hourly.density_after_kg_m3_{idx4}` | registry-owned hourly snow state families | `m` and `kg m^-3` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_melt`, `snow_hourly_melt_raw`, `snow_hourly_melt_branch_active`, `snow_hourly_melt_terms` | `snow.hourly.melt_m_{idx4}`, `snow.hourly.melt_raw_m_{idx4}`, `snow.hourly.melt_branch_active_{idx4}`, `snow.hourly.melt_amelt_in_{idx4}`, `snow.hourly.melt_bmelt_in_{idx4}`, `snow.hourly.melt_cmelt_in_{idx4}`, `snow.hourly.melt_dmelt_in_{idx4}` | registry-owned hourly melt term/state families | `m`, signed `m`, `dimensionless`, and `in` preserved at diagnostic boundary | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0268/HPHYS0269 snowpack diagnostics | `snow_runtime_swe_m`, `snow_runtime_depth_m`, `snow_runtime_density_kg_m3`, `snow_runtime_settle_day_count`, `snow_s_m`, `snow_hourly_rain_sum_m`, `snow_hourly_rain_retained_sum_m`, `snow_hourly_snowfall_water_equiv_sum_m`, `snow_hourly_melt_raw_sum_m`, `snow_hourly_melt_sum_m`, `snow_runtime_swe_closure_error_m`, `wb13_rm_mm`, `wb13_snow_water_mm` | Opt-in run-trace evidence for classifying H1/H7/H39 spring snowpack/SWE/`RM` lineage and winter melt/rain-retention migration before assigning material `Ep` residual ownership | runtime state `m`/`kg m^-3`/`count`, daily coupling `m`, WB13 publication `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0270 daily snowpack state diagnostics | `snow_runtime_swe_before_m`, `snow_runtime_depth_before_m`, `snow_runtime_density_before_kg_m3`, `snow_runtime_settle_day_count_before`, `snow_runtime_swe_m`, `snow_runtime_depth_m`, `snow_runtime_density_kg_m3`, `snow_runtime_settle_day_count`, `snow_runtime_swe_delta_m`, `snow_runtime_depth_delta_m`, `snow_runtime_density_delta_kg_m3`, `snow_runtime_settle_day_count_delta` | Opt-in run-trace evidence for classifying daily snowpack carry-state residuals before assigning H1/H7/H39 spring divergence ownership to WB17 `Ep`, storage, WB13 publication, or another snowpack production seam | runtime state `m`/`kg m^-3`/`count` and daily deltas in same units | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0271/HPHYS0272 melt-forcing diagnostics | `snow.hourly.melt_raw_m_####`, `snow.hourly.melt_m_####`, `snow.hourly.melt_amelt_in_####`, `snow.hourly.melt_bmelt_in_####`, `snow.hourly.melt_cmelt_in_####`, `snow.hourly.melt_dmelt_in_####`, `snow.hourly.melt_hrtef_f_####`, `snow.hourly.melt_hrdtf_f_####`, `snow.hourly.melt_vwmph_####`, `snow.hourly.melt_rainin_####`, `snow.hourly.melt_wind_adjustment_####`, `snow.hourly.melt_branch_active_####`, `winter.hourly.rad_mj_m2_####`, `winter.hourly.dewpoint_c_####`, `winter.hourly.wind_m_s_####` | Opt-in run-trace evidence for classifying H1 day-36 spurious melt against `melt.for` term-level lineage, hourly forcing, and `SC-CLIMATE-001#INV-CLIMATE-013` radiation units before changing production physics | melt depths `m`; radiation `MJ m^-2 h^-1`; melt terms in inch-equivalent pre-`0.0254` conversion, temperatures `degF`/`degC`, wind `mph`/`m s^-1`, rain `in`, flags `0/1` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ws_frz`, `Nft` | `frost.runtime_ws_frz`, `frost.runtime_nft` (`HillslopeProductionStateSymbol::{Wb14FrostRuntimeWsFrz,Wb14FrostRuntimeNft}`) | frozen-soil coupling boundary outputs | units preserved as declared | `[DIRECT][Static] + [INFERENCE][Static]` |
| `InfCap_frz` | `frost.runtime_infcap_frz` (`HillslopeProductionStateSymbol::Wb14FrostRuntimeInfcapFrz`) | frozen-soil infiltration-capacity boundary output | `m s^-1` required at exported boundary | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| Snow-free hour/day | `Dsnew = 0`, `ρsnew = 0`, `hrmelt = 0` | No snowpack is a valid boundary condition. `[DIRECT][Static] + [INFERENCE][Static]` |
| Cold non-melt day | `Tmax < -3 degC` with snowpack present and `hrmelt = 0` | Explicit melt assumption in Section 3.6. `[DIRECT][Static]` |
| Density-buildup pre-melt state | `ρsnew < 350 kg m^-3` and computed melt retained in pack (no liquid export) | Explicit density gate before liquid melt reaches soil. `[DIRECT][Static]` |
| All-snow precipitation day | `Tmax < 0 degC`, all precipitation routed as snowfall | Explicit partition rule. `[DIRECT][Static]` |
| All-rain precipitation day | `Tmin > 0 degC`, all precipitation routed as rainfall | Explicit partition rule. `[DIRECT][Static]` |

## Invalid States

- Negative snow depth or snow density (`Ds* < 0`, `ρs* < 0`) beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Melt water export where `ρsnew < 350 kg m^-3`. `[DIRECT][Static]`
- Post-branch positive `hrmelt_raw > Dsavail`; negative `hrmelt_raw` is valid only as signed `melt.for`/corrected daily redistribution lineage and must not be silently clamped before the `winter.for` daily post-processing branch. `[DIRECT][Static]`
- Negative retained rain (`hrrain_store < 0`) or residual direct rain below zero after rain-on-snow holding-capacity accounting. `[DIRECT][Static]`
- `ρsnew > 522 kg m^-3`. `[DIRECT][Static]`
- `Dsnew = 0` while `ρsnew > 0`. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing required winter boundary payload fields for runoff/infiltration/water-balance coupling. `[DIRECT][Static] + [INFERENCE][Static]`
- Active CLIM05 coupling with missing/non-finite/out-of-domain `snow.options.*` controls or negative `snow.runtime_swe`. `[DIRECT][Static] + [INFERENCE][Static]`
- Material negative or non-finite projected runtime snow state (`snow.runtime_swe`, `snow.runtime_depth_m`, `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count`) before snow-coupling branch selection, even when the branch would otherwise be inactive. `[DIRECT][Static] + [INFERENCE][Static]`
- Published `Snow-Water` or hydout-equivalent snow storage value sourced from static sidecar control `snow.options.ssd` instead of runtime `snow.runtime_swe`. `[DIRECT][Static] + [INFERENCE][Static]`
- Drift-active process claims in promotion evidence without updated active-lineage authority. `[DIRECT][Static] + [INFERENCE][Static]`
- Active frost branch execution that omits required routine-chain handoff semantics (`frwatc(1)` at active-day hour-1 ingress and `frwatc(0)` at day-end/thaw-complete egress). `[DIRECT][Static] + [INFERENCE][Static]`
- Frost-active conductivity coupling that bypasses `frsoil`/`getFreezeCond` authority mapping or exports non-finite `frost.runtime_infcap_frz`. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-SNOWFREEZE-P-001: Emit hourly winter state/update surfaces (`Ds*`, `ρs*`, `hrmelt`, `Dfrost`, `Dthaw`, `Nft`, `Ws_frz`, `InfCap_frz`) and daily `S` with declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-002: Enforce all `INV-SNOWFREEZE-*` runtime guards before publishing boundary payloads. `[INFERENCE][Static]`
- OBL-SNOWFREEZE-P-003: Route meltwater to downstream runoff/infiltration only when density-gate conditions are satisfied. `[DIRECT][Static]`
- OBL-SNOWFREEZE-P-004: Surface typed errors for violated melt bounds, snow-state domains, and frost-branch inconsistencies; no silent fallback values. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-005: Keep drift-activation assumptions explicit; do not imply active drift transport without authority-backed contract amendment. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-006: When parsed snow controls are projected, publish `S` and `snow.runtime_swe` as explicit coupled boundary/state outputs and hard-fail on active-coupling symbol/domain violations. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-007: Publish day-key `Snow-Water` storage outputs from runtime SWE state mapping (not static sidecar controls) and preserve cold-day partition closure (`Tmax <= rst` implies no direct liquid `RM` contribution). `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-008: Preserve baseline-authoritative frost routine-chain dispatch semantics in active branches (`winter -> frostN -> {frzng/mlt*}` with required `frwatc` ingress/egress handoff) and hard-fail on sequencing violations. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-009: Apply frozen-soil conductivity authority via `frsoil` + `getFreezeCond` lineage when frost is present and publish bounded, finite `frost.runtime_infcap_frz` seam outputs. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-010: Preserve signed hourly `melt.for` output through the corrected daily `winter.for` negative-melt redistribution branch before publishing daily melt/SWE closure; do not clamp negative hourly melt at the equation boundary and do not reproduce the pinned baseline sign/branch bug. `[DIRECT][Static]`
- OBL-SNOWFREEZE-P-011: Apply `snowd.for` rain-on-snow holding-capacity accounting before runoff/infiltration liquid forcing: retained rain increases snowpack density/SWE, residual rain remains liquid, and daily `S` includes retained rain as snowpack storage gain. `[DIRECT][Static]`
- OBL-SNOWFREEZE-P-012: Validate projected runtime snow-state domains before inactive snow fallback or runoff/infiltration reconciliation; do not canonicalize material negative snow state to zero as a continuation path. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-013: Preserve producer-side traceability for H1/H7/H39 spring snowmelt diagnostics: daily evidence must expose runtime SWE/depth/density state, retained/released rain, raw/routed melt, `snow.routed_melt_m`, `wb12_infiltration`, and `Q` before assigning residual ownership. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-014: For HPHYS0293 continuation, preserve term-level snow depletion evidence: raw hourly melt, redistributed hourly melt, retained/released rain, runtime SWE/depth/density before and after snow coupling, snow-state closure error, WB13 `RM`, and WB13 `Snow-Water`. Evidence must label corrected negative-melt carried-state residuals separately from true melt formula or publication defects. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-015: For HPHYS0296 acceptance, classify H1/H7/H39 snow/`RM` residuals from cumulative windows and target rows using raw positive/negative hourly melt, corrected routed melt/state-loss lineage, retained/released rain-on-snow, post-winter rain, runtime SWE closure, and WB13 `RM`/`Snow-Water` publication identity, then require a per-window defective-model verdict with mechanistic `file:line` root cause in both openWEPP and `/workdir/wepp-forest_260430_baseline`, reconstruction to named tolerance, independent correctness adjudication, and auditable ADR0017 peer disposition (`HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, or `UNRESOLVED`) before allowing any residual to leave the failing set. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-016: For HPHYS0297-era archaeology, publish a per-window defect ledger for H1/H7/H39 that reconstructs archived original `/workdir/wepp-forest_260430_baseline/src/winter.for:434-448` negative-melt behavior from openWEPP hourly trace fields, records the corrected openWEPP source lineage, compares reconstructed `RM` to the archived comparator using a named tolerance, and assigns an ADR0017 peer verdict (`HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, or `UNRESOLVED`) without downstream compensation. After ADR-0016/HPHYS0303, active comparator reruns use the fixed `wepp_260430` anchor instead of the archived original bug branch. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-017: For HPHYS0298, publish a paired baseline/openWEPP lineage partition ledger for all nine H1/H7/H39 target windows with observe identity evidence, ordered first-divergent cut-point classification, canonical winter symbols and units, source-line provenance, closed downstream identity context, and explicit `HARNESS-SURFACE-MISMATCH`/`LEGACY-DEFECTIVE`/`OPENWEPP-DEFECTIVE`/`UNRESOLVED` verdicts before any semantic re-tiering or follow-on correction. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-018: Comparator and ledger producers after ADR0017 must publish same-quantity unit pairing, lineage-stage pairing, independent correctness authority, owner/follow-on-gate fields for `HOLD`, and supersession/retraction references for invalidated prior verdicts before labeling a snow/`RM` row `OPENWEPP-DEFECTIVE` or authorizing production edits. `[INFERENCE][Static]`
- OBL-SNOWFREEZE-P-019: HPHYS0314 and later snow/`RM` ledger producers must preserve route counts and source lineage from the prior package ledger, explicitly reclassify stale HPHYS0298-era `OPENWEPP-DEFECTIVE` labels under ADR0017, and publish the next owner/follow-on gate for every unresolved row before HPHYS0315/HPHYS0316 or production edits begin. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-020: HPHYS0315 and later hourly snowfall input-lineage producers must publish paired fixed-baseline/openWEPP values and source-line provenance for `rain`, `stmdur`, `wntdur`, `wnttim`, `hrtemp`, `rst`, `hrsnow`, `hrrain`, active storm interval, and rain/snow branch choice before assigning `OPENWEPP-DEFECTIVE` ownership or authorizing production edits; otherwise the row remains owned `HOLD` with the next follow-on gate. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-021: HPHYS0316 and later recursive carry producers must publish 2014 day-1, 2013 terminal, and first-material-2013 divergence continuity for each H1/H7/H39 spring-2016 inherited row group, preserve the `33` carried-row total, and route any unresolved inherited terminal delta to a concrete follow-on owner before any snow producer, branch-predicate, melt-term, or downstream compensation edit is considered. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-022: HPHYS0317 and later paired input-surface producers
  must preserve the combined `57` carried-row route, publish fixed-baseline and
  openWEPP controlling `stmtim` input surfaces for the 2013 day 11 hour 11
  positive-`hrsnow` key, and keep snow producer, drift, melt-term,
  branch-predicate, and downstream water-balance edits in `HOLD` unless the
  ledger proves source-line-owned openWEPP defect authority.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-023: HPHYS0318 OpenWEPP snow/freeze trace producers must
  publish `snow.hourly.stmtim.*_####` control-surface maps and preserve the
  combined `57` carried-row route. When fixed-baseline paired `stmtim` observe
  values are absent, the route remains `paired-fixed-baseline-stmtim-observe-
  hold` and no snow-producer or downstream compensation edit is authorized.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-024: HPHYS0319 snow/freeze observe-recovery producers must
  publish fixed-baseline H1/H7/H39 2013 day 11 hour 11 `stmtim` control
  surfaces from the pinned baseline, pair them with regenerated OpenWEPP
  `snow.hourly.stmtim.*_0011` diagnostics, record the temporary observe patch,
  and keep the combined `57` carried-row route in `HOLD` unless the ledger
  proves source-line-owned OpenWEPP defect authority plus independent
  correctness authority.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-025: HPHYS0320 snow/freeze producers must rerun the H1/H7/H39
  2013 day 11 hour 11 trace lane after the SIMIMPL28 start-time normalization,
  prove normalized `wnttim = 1`, active interval `1`, snow branch `1`, and
  `hrsnow ~= 0.00074545 m`, and preserve downstream compensation prohibitions
  for any residual not owned by the start-time seam.
  `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-SNOWFREEZE-C-001: Infiltration/runoff consumers treat redistributed `hrmelt`/daily `wmelt` as event liquid forcing with the same rigor as rainfall forcing. WB12 must expose meltwater to Green-Ampt infiltration capacity before assigning the residual to runoff. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-002: Daily water-balance consumer treats melted snow as rainfall contribution in Eq. [5.1.1] accounting semantics. `[DIRECT][Static]`
- OBL-SNOWFREEZE-C-003: Soil/erosion-related consumers receiving frost outputs (`Dfrost`, `Dthaw`, `Nft`, `Ws_frz`, `InfCap_frz`) must fail explicitly on missing or invalid winter payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-004: Consumers propagate invariant-violation context as typed errors without silent clamping/defaulting. `[INFERENCE][Static]`
- OBL-SNOWFREEZE-C-005: Runoff and storage reconciliation consumers must apply signed `S` coupling semantics and reject active-coupling payloads missing required `snow.options.*` controls. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-006: Output/publication consumers must reject static-control substitution where snow-storage publication aliases (`Snow-Water`, hydout-equivalent snow-water surfaces) fail runtime SWE derivation checks. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-007: Consumer implementations must preserve the baseline `wmelt -> fin/smrate -> layer ingress -> infiltration/runoff residual` lineage from `watbal_hourly.for` and `grna.for`; melt-only runoff shortcuts or publication-only storage compensation are invalid. `[DIRECT][Static]`
- OBL-SNOWFREEZE-C-008: WB12/WB14/WB13 consumers must reject domain-invalid runtime snow state before treating snow coupling as inactive or publishing zero snow storage. `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Melt bounds and density gate (`INV-SNOWFREEZE-001/002`) | melt post-processing and pre-routing checks | Explicit branch applies melt bounds; hard error only if post-branch state remains invalid or if density gate is violated | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Snow depth/density domain (`INV-SNOWFREEZE-003/004`) | hourly snowpack state update | Hard error on domain/branch inconsistency | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Partition and activation branches (`INV-SNOWFREEZE-005/009`) | daily/hourly branch selection | Hard error on branch mismatch or silent bypass | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Frost heat-flow semantics (`INV-SNOWFREEZE-006`) | frost routine bookkeeping | Hard error on invalid equation domain; investigate hourly-heavy deltas per ADR confidence tiers | Tier-B investigation gate | `[DIRECT][Static]` |
| Coupling completeness (`INV-SNOWFREEZE-007`) | winter payload boundary handoff | Hard error on missing/invalid field or unit mismatch | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| CLIM05 parsed snow-control coupling (`INV-SNOWFREEZE-010`) | runtime snow-control adaptation and hydrology coupling branch | Hard error on missing/non-finite/out-of-domain controls or invalid `S`/`snow.runtime_swe` closure | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Frost routine-chain dispatch/handoff (`INV-SNOWFREEZE-012`) | active winter-frost branch execution path | Hard error on routine-chain dispatch mismatch or missing `frwatc` handoff sequencing | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Frozen-soil conductivity closure (`INV-SNOWFREEZE-013`) | frost-active conductivity/infiltration-capacity coupling path | Hard error on non-authoritative conductivity path or non-finite/invalid `frost.runtime_infcap_frz` export | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Spring snowpack lineage closure (`INV-SNOWFREEZE-014`) | HPHYS0268 targeted/full-suite evidence gate | Governance `HOLD` until material spring `Ep` divergence evidence includes baseline snowpack/SWE/`RM` lineage and rules in a production defect before production physics edits | HPHYS0268 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Winter melt/rain-retention closure (`INV-SNOWFREEZE-015`) | HPHYS0269 targeted/full-suite evidence gate and active snowpack kernel | Hard error for invalid retained-rain/signed-melt state; governance `HOLD` until baseline `winter.for`/`snowd.for`/`melt.for` migration evidence is complete | HPHYS0269 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Residual rain-on-snow routed-melt closure (`INV-SNOWFREEZE-021`) | HPHYS0288 contract-derived test and WB12/WB18 liquid-forcing assembler | Hard error / `HOLD` when residual rain-on-snow is left on the direct-rain-only path, omitted from `wmelt`, or double counted across direct rain and routed melt | HPHYS0288 rain-on-snow partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB13 snow publication closure (`INV-SNOWFREEZE-022`) | HPHYS0289 contract-derived tests and WB13 mapper | Hard error / `HOLD` when daily `wmelt` is not exposed to WB13, `RM` is reconstructed from raw precipitation/SWE delta, or `Snow-Water` does not derive from runtime snowpack storage | HPHYS0289 WB13 publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Daily snowpack carry-state closure (`INV-SNOWFREEZE-016`) | HPHYS0270 targeted/full-suite evidence gate | Governance `HOLD` until pre-day/post-day SWE, depth, density, and settle-count lineage is explicit enough to localize H1/H7/H39 spring residual ownership | HPHYS0270 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Day-36 melt-forcing closure (`INV-SNOWFREEZE-017`) | HPHYS0271/HPHYS0272 targeted/full-suite evidence gate | Governance `HOLD` until H1 day-36 `melt.for` term-level, hourly-forcing, and radiation-unit lineage is explicit enough to localize or correct the spurious melt event | HPHYS0271/HPHYS0272 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0318 `stmtim` control-surface trace closure (`INV-SNOWFREEZE-044`) | SIMIMPL28 runtime trace publication and paired fixed-baseline observe ledger | Governance `HOLD` until OpenWEPP diagnostics and fixed-baseline paired `stmtim` observe values exist for the 2013 day 11 hour 11 route; instrumentation alone does not authorize producer/downstream edits | HPHYS0318 `stmtim` trace gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0319 fixed-baseline `stmtim` observe recovery (`INV-SNOWFREEZE-045`) | Temporary fixed-baseline observe recovery plus paired OpenWEPP trace classification | Governance `HOLD` unless paired same-key `stmtim` values prove source-line-owned OpenWEPP defect authority and independent correctness; observe instrumentation alone does not authorize producer/downstream edits | HPHYS0319 fixed-baseline observe gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0320 `stmtim` start-time closure (`INV-SNOWFREEZE-046`) | SIMIMPL28 start-time projection and paired H1/H7/H39 trace rerun | Hard error for non-finite start time; finite below-hour-one starts use baseline `wnttim = 1.0` normalization before snow/rain branch selection; residual divergence stays owned `HOLD` with a named follow-on lane | HPHYS0320 snow/freeze timing closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Drift activation governance (`INV-SNOWFREEZE-008`) | review/disposition/promotion gate | Governance `HOLD` until active-implementation authority is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not bitwise
parity). Contract-specific interpretation tolerances:

| Tolerance ID | Definition | Value | Notes |
|---|---|---|---|
| TOL-SNOWFREEZE-001 | Melt bound residual tolerance for post-branch `0 <= hrmelt <= Dsavail` | `1e-12 m` | Comparator interpretation only; runtime branch/hard-fail behavior remains explicit for material violation. |
| TOL-SNOWFREEZE-002 | Snow depth/density non-negative comparator tolerance | lower bound `>= -1e-12` | Runtime hard-fail still required for values materially below zero. |
| TOL-SNOWFREEZE-003 | Density threshold gate proximity tolerance around `350 kg m^-3` | `1e-9 kg m^-3` | Prevents floating-point noise from flipping density-gate branch semantics. |
| TOL-SNOWFREEZE-004 | Zero-depth/zero-density closure tolerance | If `Dsnew <= 1e-12 m`, require `ρsnew <= 1e-9 kg m^-3` | Prevents false closure from product-only checks that can mask invalid nonzero density. |
| TOL-SNOWFREEZE-005 | Frost heat-flow equation residual tolerance for iterative closure diagnostics | `<= 1e-8` in routine-native flux units | Diagnostic/comparator aid; not a silent runtime correction mechanism. |
| TOL-SNOWFREEZE-006 | Stage-1 snow storage closure tolerance | `1e-9 m` water equivalent per daily snow coupling step | Runtime conservation gate for `old SWE + snowfall water equivalent + retained rain - new SWE - routed snowpack melt`; violations are typed errors, not hidden clamps. |

## CLIM05 Parsed Snow-Control Runtime Coupling Addendum

## HPHYS0247 Winter Activation Trigger Addendum

1. Winter hourly forcing must trigger from runtime state and climate forcing:
   - existing `snow.runtime_swe > 0`,
   - active frost runtime state,
   - or daily average temperature below `0 degC`.
2. Active snow coupling must trigger from snow/cold runtime state, not sidecar
   presence: existing `snow.runtime_swe > 0` or daily average temperature below
   `0 degC` with projected snow controls/defaults present. Frost-active
   processing can invoke winter/frost hourly forcing without forcing the
   snow-coupling branch when there is no existing snowpack and no projected
   snow-control payload.
3. `snow.options.snow_file_present` records sidecar discoverability/override
   provenance only. It must not suppress winter hourly processing when a
   runtime/thermal trigger is true and parsed snow controls are present from
   the explicit missing-file defaults branch.
4. Contract-derived vectors must prove a cold precipitation day without a snow
   sidecar emits hourly snowfall inputs, mutates `snow.runtime_swe`, and
   publishes `RM` from liquid rain plus melt rather than raw precipitation
   passthrough.

### CLIM05 Required Surfaces

| Surface | Symbols |
|---|---|
| Parsed sidecar controls | `snow.options.rst`, `snow.options.newsnw`, `snow.options.ssd`, `snow.options.snow_file_present` |
| Climate drivers | `Tmax`, `Tmin`, hyetograph precipitation depth |
| Coupled runtime state/output | `snow.runtime_swe`, `S` |

### CLIM05 Deterministic Rules

1. Active coupling is explicit when `snow.runtime_swe > 0` or when the day is
   thermally snow-active and projected snow controls/defaults are present on
   the runtime state surface. `snow.options.snow_file_present` is validated as
   binary sidecar discoverability/override provenance only and must not
   activate coupling by itself.
2. Partition logic uses `rst` threshold:
   - `Tmax <= rst`: all precipitation accumulates as snow;
   - `Tmin >= rst`: precipitation is rain pathway;
   - mixed-day branch: deterministic partition driven by `Tmin/Tmax/rst`.
3. Signed snow-water term is `S = melt - accumulation`; snow storage update is
   `snow.runtime_swe(new) = snow.runtime_swe(old) + accumulation - melt` with
   non-negative closure.
4. Parsed control domains must remain finite and physically valid:
   `newsnw > 0`, `ssd > 0`, and `newsnw <= ssd`.
5. Active-coupling missing/non-finite/domain-invalid controls or invalid
   `S`/`snow.runtime_swe` closure are hard-fail states.
6. Publication mapping closure is required: downstream day-key `Snow-Water`
   aliases must derive from runtime SWE state and must not publish static
   `snow.options.ssd` control values as dynamic storage state.

### CLIM05 Contract-Test Vectors

1. Active coupling nominal vector produces deterministic `S`, updated
   `snow.runtime_swe`, and valid hydrology-coupled closure.
2. Missing active-coupling required parsed control symbol hard-fails with typed
   missing-input posture.
3. Non-finite or out-of-domain active-coupling control/state hard-fails with
   typed non-finite/domain posture and no fallback.

## CLIM06 Frozen-Soil Runtime Coupling Addendum

### CLIM06 Required Surfaces

| Surface | Symbols |
|---|---|
| Parsed frost sidecar controls | `frost.options.wintRed`, `frost.options.fineTop`, `frost.options.fineBot`, `frost.options.ksnowf`, `frost.options.kresf`, `frost.options.ksoilf`, `frost.options.kfactor1`, `frost.options.kfactor2`, `frost.options.kfactor3`, `frost.options.frost_file_present` |
| Climate freeze/thaw drivers | `Tmax`, `Tmin`, hyetograph precipitation depth |
| Coupled runtime state/output | `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` |

### CLIM06 Deterministic Rules

1. Active CLIM06 coupling is explicit when
   `frost.options.frost_file_present = 1` and `frost.options.wintRed = 1`.
2. Freeze/thaw branch selection is climate-driven and explicit:
   - `Tmin <= 0 degC`: freeze-active branch updates `Dfrost` and
     infiltration-capacity reduction state;
   - `Tmin > 0 degC`: thaw/inactive branch reduces frozen-depth influence.
3. CLIM06 infiltration-capacity coupling uses a bounded frozen-soil reduction
   envelope:
   - `kfactor_floor = min(kfactor1, kfactor2, kfactor3)`
   - `freeze_fraction = clamp(Dfrost / 0.20, 0, 1)`, where `0.20 m` is the
     tilled-layer conductivity depth scale, not a frost-depth model cap
   - `InfCap_frz = Ke * (1 - freeze_fraction + freeze_fraction * kfactor_floor)`
4. Active-coupling missing/non-finite/out-of-domain controls or derived frost
   runtime surfaces are hard-fail states; no silent fallback/default branch is
   allowed.

### CLIM06 Contract-Test Vectors

1. Active CLIM06 vector emits deterministic
   `Dfrost`/`Dthaw`/`Nft`/`Ws_frz`/`InfCap_frz` and reduced infiltration
   coupling with valid runoff closure.
2. Missing active-coupling frost control symbol hard-fails with typed
   missing-input posture.
3. Non-finite active-coupling frost control symbol hard-fails with typed
   non-finite posture.
4. Out-of-domain active-coupling frost control/state hard-fails with typed
   domain posture and no fallback.

## FDHP01 Frost Depth Heat-Flow Addendum

FDHP01 defines the executable frost-depth phase boundary for single-OFE frost
runtime publication, but the 2026-06-11 cohort validation did not close that
boundary. The requirements below remain active correction authority:

1. Active frost-depth update uses hourly signed surface heat flow through the
   snow/residue/frozen-soil path. Freezing energy extends `frdp`; positive thaw
   energy reduces `frdp`/increases `Dthaw`. The branch must publish finite
   `Qsrf`, `Quf`, and `Ksrf` hourly surfaces.
2. `frdp`, `Dfrost`, `Dthaw`, `thdp`, `tfrdp`, and `tthawd` are bounded by the
   physical profile depth (`solthk`/layer topology), not by the tilled-layer
   `0.20 m` conductivity depth.
3. The retired daily freeze-index proxy
   `frdp = 0.20 m * clamp((0 - mean_temperature) / 6 degC, 0, 1)` is invalid
   in production frost-depth publication paths.
4. WAT publication must expose `frdp` in `mm` from the runtime frost-front
   depth state. `frozwt` remains frozen-water storage, not frost depth.
5. The executable frost state must carry per-layer frozen depth and frozen
   water as first-class runtime state (`wb18_perc_frozen_depth_####` and
   `wb18_perc_frzw_####`). Aggregate `Ws_frz` and downstream `frozwt` lineage
   are sums of the legacy `soilf` layer store
   `wb18_perc_frzw_#### + thetdr_#### * wb18_perc_frozen_depth_####`.
   Defining frozen storage as `frdp * scalar`, including
   `frdp * (theta_field_capacity - theta_residual)`, is invalid.
6. The depth update must consume the same layered state for resistance/capacity
   bookkeeping: a layer cannot freeze beyond its depth, frozen water cannot
   exceed the layer's active storage capacity, and frost-front advance must stop
   when no valid layer capacity remains.
7. This phase does not authorize changing `kfactor` magnitude, forest
   `ksatadj`, snow/radiation/ET/runoff production, `p11`, or MOFE routing.
   Full fine-layer water redistribution and heat-flow depth calibration remain
   active FDHP01 closure evidence items unless focused validation proves they
   are not required to close the single-OFE depth/duration and conservation
   defect.

## SIMIMPL27 Boundary/API Closure and Contract-Test Requirements Addendum

### Boundary/API Closure Scope

SIMIMPL27 closes the authority-side boundary/API ambiguity for hourly snow and
freeze migration scope by ratifying concrete alias names in this contract.

Cross-contract consumer ownership for the ratified winter payload is explicit:

| Boundary payload family | Producer authority | Consumer authority |
|---|---|---|
| `S`, `snow.runtime_swe`, day-key `Snow-Water` publication lineage | `SC-SNOWFREEZE-001`, `SC-WATBAL-001` | `SC-RUNOFFPART-001`, `SC-SYSTEM-001` |
| `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` | `SC-SNOWFREEZE-001` | `SC-SOIL-001`, `SC-WATBAL-001`, `SC-SYSTEM-001` |
| Hourly reserved aliases under `snow.hourly.*`, `winter.hourly.*`, `frost.hourly.*` | `SC-SNOWFREEZE-001` | SIMIMPL28/SIMIMPL29/SIMIMPL32 runtime implementation and downstream coupling tests |

### Contract-Derived Test Requirements (Downstream)

SIMIMPL28/SIMIMPL29/SIMIMPL32 must implement contract-derived tests covering:

1. Alias projection closure:
   existing typed runtime symbols map exactly to the contract alias names for
   `snow.options.*`, `snow.runtime_swe`, `frost.runtime_*`, `tmax`, `tmin`, and
   flux `S`.
2. SIMIMPL28 reserved hourly forcing payload completeness:
   when hourly winter forcing synthesis is active, required climate-driven
   families (`winter.hourly.rad_mj_m2_####`, `winter.hourly.air_temp_c_####`,
   `winter.hourly.cloud_fraction_####`, `snow.hourly.rain_m_####`,
   `snow.hourly.snowfall_m_####`) must be present with valid units/domains
   before boundary publication.
3. SIMIMPL29 reserved hourly snow-kernel payload completeness:
   migration-complete hourly snow families (`snow.hourly.depth_*`,
   `snow.hourly.density_*`, `snow.hourly.depth_available_m`,
   `snow.hourly.melt_m`) must be present with valid units/domains before
   boundary publication.
4. `Dsavail` timing closure:
   melt upper-bound branch uses pre-hour available snow-depth state
   (`snow.hourly.depth_available_m`) with no off-by-one timing drift.
5. SIMIMPL32 frost-hourly payload and routine-chain completeness:
   required `frost.hourly.*` families plus active routine-chain dispatch/
   handoff semantics (`winter -> frostN`, `frwatc` ingress/egress, `frzng`/
   `frznw` freeze lineage, `frsoil`/`getFreezeCond` conductivity lineage) are
   present and validated with typed failures.
6. Typed failure posture:
   missing/non-finite/out-of-domain winter payload symbols in active hourly
   branches hard-fail with typed error paths and no silent fallback.

### SIMIMPL28 Forcing-Emission Scope Clarification

SIMIMPL28 closure scope is restricted to baseline-authoritative hourly forcing
families generated from climate + slope + active snow controls (`sunmap`,
`radcur`, `hr_tmp`, `stmtim` lineage). Frost heat-flow hourly families remain
SIMIMPL32 closure scope, while hourly snow depth/density/melt state families
remain SIMIMPL29 closure scope. Promotion claims for deferred families must
remain explicit `HOLD` ownership until corresponding evidence exists.

### SIMIMPL29 Snow Kernel Port and Hourly State Closure

SIMIMPL29 ports baseline-authoritative snow update and melt branch structure
(`snowd` + `melt` lineage) into openWEPP hydrology runtime coupling for active
snow control execution. Required closure surface for this wave:

1. Hourly snow kernel state publication:
   - `snow.hourly.depth_before_m_####`
   - `snow.hourly.depth_available_m_####`
   - `snow.hourly.density_before_kg_m3_####`
   - `snow.hourly.depth_after_m_####`
   - `snow.hourly.density_after_kg_m3_####`
   - `snow.hourly.melt_m_####`
2. Runtime carry-state publication for day-to-day continuity:
   - `snow.runtime_swe`
   - `snow.runtime_depth_m`
   - `snow.runtime_density_kg_m3`
   - `snow.runtime_settle_day_count`
3. Active branch inputs are required and typed:
   - `snow.hourly.{rain,snowfall}_####`
   - `winter.hourly.{rad_mj_m2,air_temp_c,cloud_fraction}_####`
   Missing or non-finite active-branch symbols are hard-fail conditions.
4. Snow coupling closure remains `S = melt - accumulation` with accumulation
   derived from hourly snowfall water-equivalent partition and non-negative
   runtime snow-state closure.

SIMIMPL29 does not claim full baseline frost energy-balance migration closure.
`frost.hourly.*` family closure remains explicit follow-on ownership.

### HPHYS0268 Spring Snowpack Re-Anchor Addendum

HPHYS0268 narrows the H1/H7/H39 continuation from generic seasonal `Ep`
residuals to the first material spring-snowmelt divergences (`|Ep diff| >
1 mm`). Closure evidence must compare the runtime snow carry state and WB13
publication lineage to pinned baseline `winter.for`, `snowd.for`, and
`melt.for` behavior before changing `Ep`/WB17 production code.

Required HPHYS0268 evidence:

1. Material-divergence anchor:
   H1/H7/H39 reports must identify first `|Ep diff| > 1 mm` dates and same-day
   WAT context for `Ep`, `RM`, `Snow-Water`, storage, runoff, lateral flow, and
   percolation.
2. Runtime snowpack lineage:
   diagnostics must expose `snow.runtime_swe`, `snow.runtime_depth_m`,
   `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count`, hourly
   rainfall, hourly snowfall water equivalent, hourly melt, and signed `S`.
3. Publication lineage:
   WB13 `Snow-Water` must derive from runtime SWE and WB13 `RM` must reconcile
   to precipitation plus snowpack change plus irrigation under contract units.
4. Trigger semantics:
   `snow.options.snow_file_present` is provenance for parsed-vs-default
   controls only. Runtime snowpack processing remains active when snowpack,
   frost, snowfall/rainfall partition, or cold thermal triggers require winter
   processing. HPHYS non-agricultural parity disables frost through legacy
   `ksflag`/land-use semantics, not snow.
5. Disposition rule:
   if the snowpack trace is incomplete or reconciles internally while semantic
   parity still diverges, disposition remains `HOLD` with narrowed ownership.
   Production changes require a baseline-authoritative defect proof and must not
   compensate through WB17 `Ep` or WB13 publication edits.

### HPHYS0269 Winter Melt/Snowpack Baselining Addendum

HPHYS0269 implements the HPHYS0268 continuation at the architecture seam rather
than tuning residuals. The required baseline source scope is
`/workdir/wepp-forest_260430_baseline/src/winter.for`, `snowd.for`, and
`melt.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

Required HPHYS0269 migration authority:

1. `melt.for` computes signed hourly melt at lines 275-301. Negative values are
   not clamped at the equation boundary; positive values are bounded by the
   current snow-depth/density water equivalent.
2. `winter.for` lines 420-464 perform daily signed-melt post-processing after
   the hourly loop and before daily `wmelt` publication. Runtime closure
   evidence must distinguish raw signed melt from redistributed daily melt.
3. `snowd.for` lines 240-279 retain rain in snowpack while density remains
   below `350 kg m^-3`; retained rain raises density/SWE and reduces direct
   liquid forcing. `winter.for` lines 456-459 then add positive residual
   `hrrain(hour)` into `hrmlt(hour,iplane)` before `wmelt` publication.
4. Daily signed `S` must equal redistributed melt minus snowfall water
   equivalent minus retained rain, and downstream runoff/infiltration forcing
   must preserve residual rain-on-snow through the `hrmlt`/`wmelt` lineage
   without double counting it as independent direct rain.
5. If any part of the source scope remains unmigrated, package disposition must
   remain `HOLD` with the unmigrated line range and state transition recorded.

## SIMIMPL31 Frost Routine-Chain Authority and Contract-Test Requirements Addendum

### Baseline Routine-Chain Authority Map

SIMIMPL31 closes migration-scope authority ambiguity for the baseline frost
routine chain by ratifying routine-to-alias/invariant ownership in canonical
contract text.

| Baseline routine (source) | Call lineage authority | Canonical responsibility | Contract boundary aliases |
|---|---|---|---|
| `winter` (`winter.for`) | Active hourly trigger dispatches `frostN(hour)` when frost trigger conditions are present; frost path is explicitly skipped only when winter coupling is disabled. | Winter-hourly orchestration entry point for frost processing within daily winter loop. | `winter.hourly.*`, `snow.hourly.*`, downstream `frost.runtime_*` payload handoff |
| `frostN` (`frostn.for`) | Main hourly frost driver; performs ingress `frwatc(1)` handoff once at active-day hour-1 initialization (`frostn.for:335-337`), branch-specific freeze/thaw process dispatch, and egress `frwatc(0)` handoff at hour-24 or thaw-complete closure. | Freeze/thaw branch routing, heat-flow bookkeeping, and daily handoff closure. | `frost.hourly.*`, `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` |
| `frzng` (`frzng.for`) | Freeze-active branch extension path called by `frostN`; invokes `frznw` when infiltrated/frozen-zone liquid-water freezing is required. | Energy-limited frost-front extension and latent-heat bookkeeping. | `frost.hourly.qsrf_w_m2`, `frost.hourly.quf_w_m2`, `frost.runtime_dfrost`, `frost.runtime_ws_frz`, `frost.runtime_nft` |
| `frznw` (`frznw.for`) | Layer-local freezing helper called by `frzng`. | Fine-layer freezing time/energy closure and frozen/liquid partition updates. | `frost.runtime_ws_frz`, `frost.runtime_dfrost` (through parent update lineage) |
| `frwatc` (`frwatc.for`) | Bidirectional handoff routine: `wbtofs=1` maps water-balance state to frost fine-layer state; `wbtofs=0` maps frost-updated fine-layer state back to coarse soil/water-balance state. | Water-state exchange seam between frost routines and water-balance/soil consumers. | `frost.runtime_ws_frz`, soil-water lineage surfaces consumed by `SC-SOIL-001` / `SC-WATBAL-001` |
| `frsoil` (`frsoil.for`) + `getFreezeCond` (`getfreezecond.for`) | Soil conductivity adjustment path for frost-active conditions; `getFreezeCond` selects frozen-soil coefficient from land-use/plant class; `frsoil` aggregates fine-layer conductivity to layer exports. | Frozen-soil conductivity and infiltration-capacity coupling authority under frost-active state. | `frost.runtime_infcap_frz`, `frost.options.kfactor{1,2,3}`, `frost.options.wintRed` |
| `winthd` (`winthd.for`) | Winter report/output helper routine. | Output/reporting surface for winter diagnostics; does not replace runtime boundary authority. | governance/reporting only |

### SIMIMPL32 Contract-Derived Test Requirements (Frost Scope)

SIMIMPL32 must implement contract-derived tests that demonstrate:

1. Dispatch trigger closure:
   active winter-frost trigger conditions dispatch frost processing and
   explicit disabled conditions skip processing without ambiguous side effects.
2. Handoff direction closure:
   `frwatc(1)` and `frwatc(0)` direction semantics are preserved at runtime
   seam boundaries with typed failures on missing/invalid handoff state.
   `frwatc(1)` ingress is the daily hour-1 handoff from water balance to the
   frost fine-layer state, not an every-hour delta application.
3. Freeze lineage closure:
   `frzng`/`frznw` branch execution preserves finite/non-negative
   freeze-depth/water-state updates with explicit failure posture on invalid
   energy-time domains. Implementations must consume hourly freezing energy
   against layer water while advancing `slfsd`/`slsic`-equivalent stores, not
   advance a scalar `frdp` independently and reconcile layer mass afterward.
4. Conductivity lineage closure:
   `frsoil` + `getFreezeCond` land-use coefficient selection remains explicit
   and drives frost-active infiltration-capacity coupling exports.
5. Cross-contract seam closure:
   frost runtime payloads consumed by `SC-SOIL-001`, `SC-RUNOFFPART-001`,
   `SC-WATBAL-001`, and `SC-SYSTEM-001` remain complete, finite, and typed.

### FDHP01 Increment A Fine-State Shadow Aliases

FDHP01 Increment A authorizes behavior-preserving shadow publication of the
baseline fine-layer frost state before it drives depth or WAT publication. The
shadow state is a conservation and handoff proof surface only: active `frdp`,
`frozwt`, `wb18_perc_frozen_depth_####`, `wb18_perc_frzw_####`, and
water-balance publication remain unchanged until the subsequent freeze/thaw
increments rebind authority.

| Legacy symbol | openWEPP shadow alias | Increment-A rule |
|---|---|---|
| `fgfrst(j,i)` | `frost.runtime_fgfrst_LLLL_FFFF` | Fine-layer frost flag, persisted as diagnostic state with integer domain `0..3`; it must not drive active depth in Increment A. |
| `slfsd(j,i)` | `frost.runtime_slfsd_m_LLLL_FFFF` | Fine-layer frozen thickness in metres; aggregate shadow handoff must keep `0 <= slfsd <= dzfine`. |
| `slsic(j,i)` | `frost.runtime_slsic_m_LLLL_FFFF` | Fine-layer ice water-equivalent store; shadow `frwatc(0)` derives `soilf`/`frzw` diagnostics from this sum. |
| `slsw(j,i)` | `frost.runtime_slsw_theta_LLLL_FFFF` | Fine-layer liquid volumetric water over unfrozen thickness; `frwatc(1)` applies the daily `st - yst` delta to this surface and `nwfrzz`. |
| `sltime(j,i)` | `frost.runtime_sltime_s_LLLL_FFFF` | Hour-local redistribution timer; Increment A resets/publishes it as zero diagnostic state. |
| `yst(i)` | `frost.runtime_yst_m_LLLL` | Prior day-end active storage for the next hour-1 `frwatc(1)` delta. |
| `nwfrzz(i)` | `frost.runtime_nwfrzz_m_LLLL` | Liquid water residing in frozen zones before `frznw`; shadow handoff updates it without changing active stores. |

Increment A must also publish an internal conservation residual proving the
handoff seam:
`Delta(fine liquid + nwfrzz + slsic) == st - yst` when no freeze/thaw arms are
bound to the shadow state. Any residual beyond roundoff is an implementation
hard stop before the state is allowed to drive depth or publication.

## Known Gaps

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SNOWFREEZE-001 | Per-invariant comparator vectors for hourly winter outputs (`hrmelt`, frost depth/thaw depth, freeze-thaw cycles) are not yet curated. | Limits immediate automated regression depth on hourly-heavy winter internals. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-SNOWFREEZE-002 | Frost-depth heat-flow executable parity remains open after FDHP01 Addendum 3 validation. The D2 layered-store continuation closed additive storage for years 2-6 on all 43 prefixes at numerical noise and cleared `p2`, but D3 remains open: openWEPP frost depth still pins near the profile bound (`~1780.3..1783.4 mm`), depth correlation is poor, frozen duration under-persists severely (mean open-minus-legacy `-518.5348837209302` days), and depth/mass remain decoupled. The remaining defect is no longer frozen-water publication; it is the missing bidirectional `frostn`/`frzng`/`mltbtm` coupling where front advance consumes layer water and thaw retreats the same active layer store under `Σ dz/k` frozen-layer resistance. | Blocks frost-depth heat-flow closure and MOFE advancement until the single-OFE cohort runs 43/43, annual `Total-Soil + frozwt` closure stays at numerical noise, depth enters a physical heat-flow envelope without profile-bound pinning, duration residual materially collapses, and the year-7 boundary effect is explained or eliminated without comparator tuning. | active-defect | `[DIRECT][Ran] + [INFERENCE][Static]` |
| GAP-SNOWFREEZE-003 | Snow drifting equations are documented in Chapter 3 but explicitly inactive in the August 1995 lineage; active-path authority for openWEPP is unresolved. | Drift-related claims cannot be promoted as active behavior yet. | non-promotable | `[DIRECT][Static]` |
| GAP-SNOWFREEZE-004 | Cross-contract boundary ownership with `SC-SOIL-001` and `SC-RUNOFFPART-001` is explicit, but executable cross-contract comparator vectors for frost-hourly internals are still incomplete. | Promotable contract authority exists; evidence depth for coupled frost vectors remains limited pending SIMIMPL32 and SIMIMPL35. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SNOWFREEZE-005 | `Dsavail` alias is fixed (`snow.hourly.depth_available_m`) and SIMIMPL29 emits the hourly family, but comparator-tier depth/density/melt vector breadth remains limited for broad climate regimes. | Residual risk is evidence-depth, not missing alias/state publication. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-06-11` | `58` | `Codex` | FDHP01 Increment A amendment: corrected `INV-SNOWFREEZE-012` from ambiguous hourly `frwatc(1)` ingress to pinned hour-1 daily ingress, and added fine-layer shadow-state aliases plus a required internal handoff conservation residual before the state may drive depth/publication. |
| `2026-06-11` | `57` | `Codex` | FDHP01 D3 amendment: specified that hourly freeze/thaw energy must move the same layer frozen-depth/frozen-water state that `frwatc` publishes, prohibited post-hoc scalar depth projection into layer stores, and updated `GAP-SNOWFREEZE-002` to the Addendum 3 D3 depth/duration signature after D2/p2 closure. |
| `2026-06-11` | `56` | `Codex` | FDHP01 layered-state amendment: required executable frost to carry per-layer frozen-depth/frozen-water state, derive aggregate `Ws_frz`/`frozwt` lineage from the legacy `soilf` layer sum, and reject scalar `frdp * theta` frozen-water surrogates as the shared D2/D3 defect. |
| `2026-06-11` | `55` | `Codex` | FDHP01 post-review cohort validation amendment: reopened `GAP-SNOWFREEZE-002` after the 43-prefix `algebraic-radium` run failed `p2`, annual closure residuals reached `75.439 mm` on emitted prefixes, and frost-depth metrics overreached the physical legacy envelope. |
| `2026-06-06` | `52` | `Codex` | SNOWSCI-S1 amendment: made runtime SWE derived from the authoritative depth/density store and bound routed snowpack melt to the same storage-loss scalar under `INV-SNOWFREEZE-019`/`TOL-SNOWFREEZE-006`. |
| `2026-06-11` | `54` | `Codex` | FDHP01 amendment: retired the frost-depth freeze-index proxy and `0.20 m` model cap under `INV-SNOWFREEZE-006`, bound executable depth to hourly heat flow and physical profile depth, added WAT `frdp` publication authority, and closed/re-scoped `GAP-SNOWFREEZE-002`. |
| `2026-06-06` | `51` | `Codex` | HPHYS0320 amendment: added `stmtim` start-time normalization snow/freeze closure authority (`INV-SNOWFREEZE-046`) for the combined 57-row route. |
| `2026-06-06` | `50` | `Codex` | HPHYS0319 amendment: added fixed-baseline `stmtim` observe recovery and paired classification authority (`INV-SNOWFREEZE-045`) before snow producer or downstream edits. |
| `2026-06-06` | `49` | `Codex` | HPHYS0318 amendment: added OpenWEPP `stmtim` control-surface trace authority (`INV-SNOWFREEZE-044`) while preserving the fixed-baseline paired-observe `HOLD`. |
| `2026-06-06` | `48` | `Codex` | HPHYS0317 amendment: added paired hourly snowfall input-surface closure authority (`INV-SNOWFREEZE-043`) joining the 57 carried spring rows under the 2013 day 11 hour 11 `stmtim` blocker. |
| `2026-06-06` | `47` | `Codex` | HPHYS0316 amendment: added 2013 terminal carry-recursion authority (`INV-SNOWFREEZE-042`) requiring spring-2016 inherited rows to connect through the 2013 terminal state and remain owned `HOLD` under the hourly snowfall input-surface blocker absent paired source proof. |
| `2026-06-06` | `46` | `Codex` | HPHYS0315 amendment: added hourly snowfall input-lineage authority (`INV-SNOWFREEZE-041`) requiring paired fixed-baseline/openWEPP precipitation forcing surfaces before producer edits and preserving `UNRESOLVED` ownership for the spring-2014 rows. |
| `2026-06-06` | `45` | `Codex` | HPHYS0314 amendment: added consolidated ADR0017 snow/`RM` route-ledger reclassification authority (`INV-SNOWFREEZE-040`) with HPHYS0313 route-count preservation, stale-verdict supersession, and owned HPHYS0315/HPHYS0316 continuation gates. |
| `2026-06-05` | `44` | `Codex` | ADR0017 ratification amendment: added comparator-flag verdict governance (`INV-SNOWFREEZE-039`) with same-unit/same-lineage proof, independent correctness authority, `HARNESS-SURFACE-MISMATCH`, and owned `HOLD` requirements. |
| `2026-06-05` | `43` | `Codex` | HPHYS0313 correction: branch-gated the settling-route lineage and reclassified the 2013 day 11 hour 11 material final-depth increment from no-snow `driftg` to positive-`hrsnow` hourly snowfall input lineage. |
| `2026-06-05` | `42` | `Codex` | HPHYS0313 amendment: added `INV-SNOWFREEZE-038` requiring split-route full-precision settling reconstruction and earlier-year carry recursion before production ownership or compensation. |
| `2026-06-05` | `41` | `Codex` | HPHYS0312 amendment: added `INV-SNOWFREEZE-037` requiring prior-year terminal snowpack lineage localization before producer or downstream edits. |
| `2026-06-05` | `40` | `Codex` | HPHYS0311 amendment: added `INV-SNOWFREEZE-036` requiring source-line carry-state parity classification for year-boundary and density/settling rows before producer or downstream edits. |
| `2026-06-05` | `39` | `Codex` | HPHYS0310 amendment: added `INV-SNOWFREEZE-035` requiring episode-level prior-day/day-start snow carry divergence reconstruction before producer or downstream water-balance edits. |
| `2026-06-05` | `38` | `Codex` | HPHYS0309 amendment: added `INV-SNOWFREEZE-034` requiring prior-day/day-start snow carry-state and same-day depletion-lead evidence before branch-predicate, melt-term, or downstream water-balance edits. |
| `2026-06-05` | `37` | `Codex` | HPHYS0303 ratification amendment: promoted fixed `wepp_260430` negative-melt comparator commit `47ac4c32faeea81bb99081f955a14c38b815ef4d`, preserving `dac3c950` only as archived original bug context. |
| `2026-06-05` | `36` | `Codex` | HPHYS0302 amendment: added `INV-SNOWFREEZE-033`, requiring like-for-like physical-quantity/unit comparator proof and paired melt term/state surfaces before any snow/melt producer edit. |
| `2026-06-05` | `35` | `Codex` | HPHYS0301 amendment: added `INV-SNOWFREEZE-032`, requiring H39 first-2013 baseline residual rain-on-snow evidence to be compared against openWEPP released plus post-winter rain before any forcing edit, and prohibiting observe-tag-only source authority. |
| `2026-06-05` | `34` | `Codex` | HPHYS0300 Claude review disposition: added sufficiency/forcing-function closure criteria so paired melt-term/state evidence cannot become an unbounded diagnostic gate, and kept H39 first-2013 forcing correction separable from raw-melt instrumentation. |
| `2026-06-05` | `33` | `Codex` | HPHYS0300 amendment: added `INV-SNOWFREEZE-031`, requiring term/state lineage evidence before raw hourly melt or post-raw routed-melt production corrections and preserving corrected negative-melt authority. |
| `2026-06-05` | `32` | `Codex` | HPHYS0299 amendment: added `INV-SNOWFREEZE-030`, corrected HPHYS0298 stale partition provenance to `winter.for:296-300`/`stmtim.for:43-95`, and required `hrsnow` parity diagnostics to use openWEPP snowfall-depth surfaces rather than derived water-equivalent summaries. |
| `2026-06-05` | `31` | `Codex` | HPHYS0298 Claude review disposition: clarified that `hrsnow`/`hrrain` hourly-forcing verdicts are porting-fidelity defects against the un-impeached pinned-baseline `winter.for:410-412` precipitation-phase partition routine, and recorded paired instrumented baseline observation as an available comparator capability. |
| `2026-06-05` | `30` | `Codex` | HPHYS0298 amendment: added paired baseline/openWEPP lineage partition authority requiring observe identity, ordered first-divergent cut-point evidence, canonical winter symbols, and per-window verdicts for all nine H1/H7/H39 target windows before re-tiering or downstream hydrology focus. |
| `2026-06-05` | `29` | `Codex` | HPHYS0297 amendment: added snow/`RM` defect-ledger reconstruction authority requiring pinned-baseline negative-melt branch reconstruction, corrected openWEPP source-line provenance, named tolerance, and explicit per-window verdict before any residual leaves the failing set. |
| `2026-06-05` | `28` | `Codex` | HPHYS0296 review disposition: tightened snow/`RM` acceptance so correlation plus internal closure is insufficient; per-window defective-model verdict, reconstruction, independent correctness adjudication, and auditable re-tiering are required before residuals leave the failing set. |
| `2026-06-05` | `27` | `Codex` | HPHYS0296 amendment: added snow/`RM` producer acceptance classifier requiring corrected-negative-melt evidence or explicit producer-migration hold before semantic acceptance, with downstream compensation prohibited. |
| `2026-06-05` | `26` | `Codex` | HPHYS0291 amendment: added same-day snow publication lifecycle authority requiring producer-owned flux publication before WB13 and prohibiting state/default masking. |
| `2026-06-05` | `25` | `Codex` | HPHYS0290 amendment: added explicit `snow.post_winter_rain_m` authority for post-winter `rain(iplane)` consumed by WB13 `RM`. |
| `2026-06-04` | `24` | `Codex` | HPHYS0289 amendment: added WB13 snow publication authority requiring daily `wmelt` exposure and `Snow-Water` from runtime snowpack storage rather than raw precipitation/SWE-delta reconstruction. |
| `2026-06-07` | `53` | `Codex` | FQ4 ksflag frost activation amendment: clarified `INV-SNOWFREEZE-009` so missing-file default frost controls with `wintRed=1` can activate standard frozen-soil coupling; `frost.options.frost_file_present` is provenance, not an activation gate. |
| `2026-06-04` | `23` | `Codex` | HPHYS0288 amendment: added residual rain-on-snow routed-melt authority from pinned `snowd.for`/`winter.for` so unretained rain after holding-capacity accounting enters `hrmlt`/`wmelt` before WB12/WB18 forcing. |
| `2026-06-04` | `22` | `Codex` | HPHYS0287 amendment: added fail-closed runtime snow-state authority so material negative SWE/depth/density/settle values cannot be silently canonicalized to inactive zero state before liquid partition. |
| `2026-06-04` | `21` | `Codex` | HPHYS0285 review-disposition amendment: bounded corrected negative-melt pack-exhaustion canonicalization to `0.005 m` water equivalent and restored typed fail-closed behavior for material carried-state overdraw. |
| `2026-06-04` | `20` | `Codex` | HPHYS0285 amendment: clarified corrected negative-melt pack-exhaustion handling so carried state loss can exhaust runtime snowpack but must publish zero SWE/depth/density rather than negative snow storage or downstream failure. |
| `2026-06-04` | `19` | `Codex` | HPHYS0284 amendment: completed corrected negative-melt authority by adding the companion carried snow-depth/SWE adjustment from `/workdir/wepp-forest` and distinguishing routed melt from snowpack state lineage. |
| `2026-06-04` | `18` | `Codex` | HPHYS0283 amendment: linked routed snowmelt partition authority to WB18 same-pass layer ingress (`SC-PERC-001#INV-PERC-016`) so reduced runoff without storage mutation remains invalid. |
| `2026-06-04` | `17` | `Codex` | HPHYS0283 amendment: added baseline-authoritative `wmelt` infiltration/runoff partition authority (`watbal_hourly.for` `fin` and `grna.for` `smrate`) and prohibited melt-only runoff shortcuts. |
| `2026-06-03` | `16` | `Codex` | HPHYS0272 amendment: linked day-36 melt-forcing residual ownership to `SC-CLIMATE-001#INV-CLIMATE-013` radiation-unit closure so Langley-scale radiation cannot be assigned to snowmelt physics or compensated downstream. |
| `2026-06-03` | `15` | `Codex` | HPHYS0271 amendment: added day-36 melt-forcing lineage authority (`INV-SNOWFREEZE-017`) requiring `melt.for` term-level and hourly-forcing evidence before assigning H1 spurious melt residual ownership. |
| `2026-06-03` | `14` | `Codex` | HPHYS0270 amendment: added daily snowpack carry-state closure authority (`INV-SNOWFREEZE-016`) requiring pre-day/post-day SWE, depth, density, settle-count, and delta evidence before assigning H1/H7/H39 spring residual ownership. |
| `2026-06-03` | `13` | `Codex` | HPHYS0269 follow-up amendment: accepted corrected `/workdir/wepp-forest` commit `03fee455` as superseding authority for the daily negative-melt redistribution branch, rejecting pinned-baseline sign/branch bug compatibility as target behavior. |
| `2026-06-03` | `12` | `Codex` | HPHYS0269 amendment: added pinned-baseline `winter.for`/`snowd.for`/`melt.for` authority for signed hourly melt, daily negative-melt redistribution, and rain-on-snow retention (`INV-SNOWFREEZE-015`) with trace aliases and producer obligations. |
| `2026-06-03` | `11` | `Codex` | HPHYS0268 amendment: added spring snowpack lineage invariant requiring runtime SWE/depth/density/settle, hourly rain/snow/melt, signed `S`, WB13 `RM`, and WB13 `Snow-Water` evidence before returning material H1/H7/H39 `Ep` residual ownership to WB17. |
| `2026-06-02` | `10` | `Codex` | HPHYS0247 amendment: clarified `INV-SNOWFREEZE-009` so winter activation is driven by runtime snow/frost/thermal triggers rather than snow-sidecar presence alone when parsed default snow controls are explicitly available. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-05 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with authority anchors, invariants, guard map, alias map, obligations, boundary disposition, tolerances, and gap register. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: resolved drift runtime/governance conflict, added missing frost/thaw symbols, fixed `InfCap_frz` unit declaration, clarified melt bound timing semantics, and tightened zero-depth/zero-density tolerance rule. |
| `2026-05-23` | `3` | `Codex` | CLIM05 amendment: added parsed snow-control runtime coupling authority (`snow.options.*`), signed `S` and `snow.runtime_swe` closure requirements, and active-coupling typed guard posture. |
| `2026-05-23` | `4` | `Codex` | CLIM06 amendment: added parsed frost-control runtime coupling authority (`frost.options.*`), explicit frozen-soil infiltration-capacity reduction envelope, and active-coupling typed guard posture for derived frost runtime surfaces. |
| `2026-05-25` | `5` | `Codex` | SIMIMPL18 amendment: added day-key cold-partition/publication closure (`INV-SNOWFREEZE-011`), explicit runtime-SWE-to-`Snow-Water` publication authority, and obligations preventing static sidecar (`snow.options.ssd`) leakage into dynamic storage outputs. |
| `2026-05-25` | `6` | `Codex` | SIMIMPL27 amendment: finalized concrete snow/freeze boundary alias mapping (typed + reserved hourly namespaces), added downstream contract-derived test requirements for hourly migration waves, and reclassified boundary/API gap posture from non-promotable ambiguity to implementation-queued promotable-with-risk. |
| `2026-05-25` | `7` | `Codex` | SIMIMPL28 amendment: split hourly reserved-family obligations into SIMIMPL28 forcing-emission scope vs SIMIMPL29 kernel-state closure scope, clarified deferred frost/depth-density family ownership, and updated gap posture to reflect staged hourly alias closure. |
| `2026-05-25` | `8` | `Codex` | SIMIMPL29 amendment: added baseline-authoritative snow kernel (`snowd`/`melt`) hourly state closure authority, runtime carry-state publication requirements, active hourly symbol hard-fail posture, and updated gap posture to reflect snow-family closure with frost-hourly follow-on ownership. |
| `2026-05-26` | `9` | `Codex` | SIMIMPL31 amendment: ratified baseline frost routine-chain authority (`winter`/`frostN`/`frwatc`/`frzng`/`frznw`/`frsoil`/`getFreezeCond`/`winthd`), added frost dispatch/conductivity closure invariants (`INV-SNOWFREEZE-012/013`), and defined SIMIMPL32 contract-derived frost test obligations with updated gap ownership. |
