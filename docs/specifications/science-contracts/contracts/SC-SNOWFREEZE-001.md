---
contract_id: SC-SNOWFREEZE-001
title: Snow and Freeze Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 78
producer_scope:
  - Winter precipitation phase partition surfaces (rain vs snow)
  - Snowpack depth/density/water-equivalent state surfaces
  - Melt and freeze-thaw transition surfaces
consumer_scope:
  - Daily water-balance accounting consumers
  - Infiltration/runoff partition consumers affected by frozen-soil state
  - Soil/erosion coupling consumers requiring freeze-thaw context
evidence_level: static
last_reviewed: 2026-06-26
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
| REF-SNOWFREEZE-LEGACY-TMPADJ | `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for:38-48`, `/workdir/wepp-forest_260430_baseline/src/tmpadj.for:112-363`, and `/workdir/wepp-forest_260430_baseline/src/frostn.for:467-480`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative frost surface-temperature lineage: `hr_tmp` calls `tmpadj` hourly before frost processing, `tmpadj` computes `surtmp(hour)` from hourly air temperature, radiation, cloud cover, wind, albedo, canopy roughness, snow/residue/frost conductance, and caps positive snow-covered surface temperature to `0 degC`; `frostn` consumes that adjusted `surtmp(hour)` for top heat flow. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-HRRAD | `chap3.pdf` §3.5, Eq. [3.5.1]-[3.5.7] | Hourly radiation derivation used by snowmelt energy terms. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-MELT | `chap3.pdf` §3.6, Eq. [3.6.1]-[3.6.6] | Melt equation structure and component terms (`amelt`, `bmelt`, `cmelt`, `dmelt`). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-MELT-ASSUMP | `chap3.pdf` §3.6 assumptions list | Melt gating assumptions (`Tmax` thresholds, density threshold, bounded melt, albedo assumption). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-SNOWDENS | `chap3.pdf` §3.7, Eq. [3.7.1]-[3.7.5] | Snow depth/density update rules under snowfall, settling, melt, and mixed melt+snowfall. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-SNOWDENS-LIM | `chap3.pdf` §3.7 terminal paragraph | Explicit upper density limit (`522 kg m^-3`). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-FROST | `chap3.pdf` §3.8, Eq. [3.8.1]-[3.8.4] | Frost/thaw heat-flow relations, layered thermal conductivity, and hourly bookkeeping outputs. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-FROSTN-QDRY | `/workdir/wepp-forest_260430_baseline/src/frostn.for:430-458`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative lower-front heat path: compute per-fine-layer unfrozen conductivity from `slsw`, `bdcons`, and `ksoilf` over one metre below the frost front, aggregate as a harmonic path, use `kufz = 0.2` only when no positive terms exist, then publish `Qdry = kufz * tmpbl / 1.0`. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-WINTER-RESDEP | `/workdir/wepp-forest_260430_baseline/src/winter.for:247-249`, `/workdir/wepp-forest_260430_baseline/src/res_dp.for:81-126`, and `/workdir/wepp-forest_260430_baseline/src/frostn.for:501-528`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative frost surface-resistance residue path: `winter` computes current `resdep` from `iresd`, `rmogt`, `lanuse`, and plant `diam`, `frostn` adds `resdep/kres` to the surface heat denominator, and below-freezing surface heat flow floors the top frozen conduction distance to `dpfsfl = dg(1)/nfine(1)/2` with `0.005 m` fallback only when fine-layer geometry is unavailable. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-FROZEN-PATH-KF | `/workdir/wepp-forest_260430_baseline/src/frostn.for:188-193`, `:530-534`, `/workdir/wepp-forest_260430_baseline/src/frzng.for:135`, `:295-304`, and `/workdir/wepp-forest_260430_baseline/src/frznw.for:106-108`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative frozen surface-path conductivities: `kftill = 1.75 W m^-1 degC^-1` and `kfutil = 2.1 W m^-1 degC^-1` are fixed constants for the frozen tilled/untilled path. They are not soil-property functions and are not multiplied by `ksoilf`; `ksoilf` belongs to the unfrozen lower-front `kufzfl` path. | `[DIRECT][Static]` |
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
| REF-SNOWFREEZE-FROST-OBS | `tests/fixtures/snowfreeze_observed/` (five WEPP hillslope fixtures + per-site manifests) and the historic frost-depth observation corpus they bind: USGS Sleepers River (`DOI 10.5066/P96753GI`, frost tube + paired snow depth), NRCS SCAN soil temperature (`stationTriplets=2020:ND:SCAN`, derived `0 degC` isotherm), NSIDC GGD498 Midwest frost tubes (`DOI 10.7265/1mcs-q536`), USDA-ARS Reynolds Creek soil temperature (CC-BY), and the WEPP-lineage Dun et al. 2010 Pullman/Morris frost validation (`doi:10.13031/2013.34896`, request-only). | External-authority frost-depth observations under ADR-0017 (legacy/compatibility frost output is a flag, not the acceptance target). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-SNOWDENSITY01 | `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/` | Evidence reconciliation showing current openWEPP and pinned legacy share the same structural snow-density/depth lineage for the SNOTEL comparison, with maximum as-built openWEPP-vs-legacy density delta `4.351046738461008 kg m^-3`; this routes remediation away from bit-parity and toward a contract-scoped physics candidate. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-ANDERSON1976-CANDIDATE | Anderson snow accumulation/ablation physics lineage as used by SNOBAL/PySnobal-style bulk snowpack densification references; package evidence begins in `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/` and follow-on SNOWDENSITY-03/04 artifacts must bind exact equations/constants before runtime promotion. | Candidate snow-density physics authority only; not a ratified production formula. | `[INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOBAL-CANDIDATE | Local PySnobal/SNOBAL diagnostic lane and three-way profile evidence from SNOWFROST-FIDELITY-G/H packages. | Reference-implementation profile and sanity evidence for SWE/depth/density behavior; diagnostic flag profile only, not target output and not a runtime dependency. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-MELT-OHMURA2001 | `references/copyrighted/Ohmura2001_meltindex.pdf` | Temperature-index melt factors include radiation/longwave/sensible terms implicitly and are not transferable defaults for the production CoE energy-balance melt path. | `[INFERENCE][Static]` |
| REF-SNOWFREEZE-MELT-PELLICCIOTTI2005 | `references/copyrighted/pellicciotti2005.pdf` | Enhanced temperature-index melt separates radiation from temperature-index behavior, supporting explicit shortwave/albedo operands rather than retuned degree-day factors. | `[INFERENCE][Static]` |
| REF-SNOWFREEZE-MELT-CARENZO2009 | `references/copyrighted/carenzo2009.pdf` | Melt-model parameter sensitivity and transferability evidence supporting no site-specific default fitting for snowmelt modernization. | `[INFERENCE][Static]` |
| REF-SNOWFREEZE-MELT-BROCK2000 | `references/copyrighted/brock2000.pdf` | Temperature/age albedo decay candidate authority for a future opt-in albedo state; exact constants require SNOWDENSITY-05C ratification before code. | `[INFERENCE][Static]` |
| REF-SNOWFREEZE-MELT-WALTER2005 | `references/copyrighted/walter2005.pdf` | Energy-balance snowmelt partitioning support for explicit melt operands and conservation checks before production activation. | `[INFERENCE][Static]` |
| REF-SNOWFREEZE-MELT-GUPTA2023 | `references/vendorable/Gupta2023_HESS.pdf` or annotated strategy citation when the local file is absent | Equifinality/parameter-identifiability guard: shared radiation forcing may classify residuals but must not be tuned or rescaled to improve snowmelt fit because it also drives other hydrology/ET consumers. | `[INFERENCE][Static]` |

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
| `watpdg` | `m` | Frost/thaw excess liquid routed to the surface ponding/upper overflow surface when fine-layer storage cannot retain it. | frost routine | water-balance publication and closure identity |
| `watbtm` | `m` | Frost/thaw excess liquid routed below the active profile when fine-layer storage cannot retain it. | frost routine | water-balance publication and closure identity (`Dp` loss lineage) |
| `Snow-Water` | `mm` | WB13/hydout snow-water storage publication surface converted from runtime SWE at the output boundary. | winter runtime state publication | WB13/hillslope WAT output |
| `Snow-Depth` | `mm` | Diagnostic hillslope WAT physical snowpack-depth publication converted from `snow.runtime_depth_m`; distinct from SWE and invalid as water-storage evidence. | winter runtime state publication | snow/frost observed-site correspondence and snow-insulation confound control |
| `winter_shortwave_daily_radly` | `Ly d^-1` | Canonical daily shortwave source accepted by openWEPP for winter shortwave/melt modernization; this is the existing climate `rad`/`radly` parser/runtime field, not a separate snow forcing column. | climate parser/runtime daily forcing seam | `SC-CLIMATE-001#INV-CLIMATE-013`, ET forcing, and opt-in melt diagnostics |
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
| `snow_model` | `enum` | Snow model selector. Current accepted values are `legacy_wepp` and opt-in candidate `physics_bulk`; `legacy_wepp` remains the default unless a future ratified activation package changes it. | runtime configuration / winter column | snow solver dispatch and diagnostics |
| `snow_cold_content` | `J m^-2` | Candidate `physics_bulk` bulk snowpack energy deficit relative to isothermal melt conditions. | candidate physics-bulk snow solver | conservation diagnostics and melt/refreeze gates |
| `snow_liquid_water` | `m water equivalent` | Candidate `physics_bulk` retained liquid-water store within the snowpack before routed release. | candidate physics-bulk snow solver | conservation diagnostics and liquid forcing |
| `snow_bulk_temperature` | `degC` | Candidate `physics_bulk` bulk snowpack temperature or equivalent thermal state used with cold content. | candidate physics-bulk snow solver | energy-state diagnostics |
| `snow_cover_age` | `h or day count` | Candidate `physics_bulk` snow-cover age / metamorphism clock independent of legacy `snow_runtime_settle_day_count` naming. | candidate physics-bulk snow solver | densification diagnostics |
| `snow_hourly_rain` | `m` | Hourly rainfall forcing before snowpack retention. | hourly precipitation partition | snow/rain-retention diagnostics |
| `snow_hourly_rain_retained` | `m` | Hourly rain retained in snowpack holding capacity. | snow density update | snowpack liquid-storage diagnostics |
| `snow_hourly_snowfall` | `m` | Hourly snowfall depth forcing. | hourly precipitation partition | snowpack accumulation diagnostics |
| `snow_hourly_depth` | `m` | Hourly snow-depth before/available/after state family. | winter snow routine | melt-bound and snow-state diagnostics |
| `snow_hourly_density` | `kg m^-3` | Hourly snow-density before/after state family. | winter snow routine | density gate diagnostics |
| `snow_hourly_melt` | `m` | Hourly post-redistribution meltwater surface. | winter routine | melt/routing diagnostics |
| `snow_hourly_melt_raw` | `m` | Signed hourly raw melt before daily redistribution. | melt routine | negative-melt diagnostics |
| `snow_hourly_melt_branch_active` | `dimensionless` | Hourly melt-branch active flag. | melt routine | melt-forcing diagnostics |
| `snow_hourly_melt_terms` | `in` | Hourly `amelt`/`bmelt`/`cmelt`/`dmelt` term family before metric conversion. | melt routine | term-level melt diagnostics |
| `snow_melt_model` | `enum` | Melt model selector. Current accepted values are `legacy_coe` and opt-in candidate `coe_shortwave_albedo_v1`; `legacy_coe` remains the default unless a future ratified activation package changes it. | runtime configuration / winter column | CoE melt-term dispatch and diagnostics |
| `snow_albedo` | `fraction` | Opt-in snow-surface albedo state consumed by the future `coe_shortwave_albedo_v1` shortwave term; accepted domain is `0 <= snow_albedo <= 0.85` under `brock2000_temperature_age_v1`. | SNOWDENSITY-05C albedo state update | `coe_shortwave_albedo_v1` melt path diagnostics |
| `snow_albedo_accumulated_positive_temperature_c_day` | `degC day` | Accumulated positive-temperature age index (`Ta`) since the latest fresh-snow reset for `brock2000_temperature_age_v1`. | SNOWDENSITY-05C albedo state update | albedo decay diagnostics |
| `snow_albedo_fresh_snow_reset_water_equiv_m` | `m water equivalent` | Fresh-snow water-equivalent increment threshold that resets `Ta` and returns albedo toward the fresh-snow cap; default core threshold is `0.001 m` water equivalent. | SNOWDENSITY-05C albedo state update | albedo reset diagnostics |
| `snow_albedo_model_id` | `enum/string` | Albedo-state provenance/model identifier; accepted opt-in value is `brock2000_temperature_age_v1`. | SNOWDENSITY-05C albedo state update | opt-in melt diagnostics and rollback evidence |
| `winter_shortwave_source_provenance` | `enum/string` | Candidate provenance ledger naming the upstream gridded/provider source when known, the normalized climate `rad`/`radly` acceptance seam, units, slope/aspect and hourly transformation lineage, and proof that snowmelt does not receive a snow-only radiation scalar. | orchestration provenance plus climate runtime seam | `SC-CLIMATE-001#INV-CLIMATE-013` anti-alias evidence |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SNOWFREEZE-001 | Melt bound and non-negativity branch semantics: post-branch exported melt satisfies `0 <= hrmelt <= Dsavail`, where `Dsavail` is the pre-hour available snow-depth state used by Eq. [3.6.1] branch logic. | hard-fail | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-MELT-ASSUMP, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-002 | Snow-density melt gate: liquid melt export to infiltration/runoff is not allowed until post-update snow density reaches `ρsnew >= 350 kg m^-3`; below this threshold melt remains in-pack and increases density. | hard-fail | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-SNOWDENS | `[DIRECT][Static]` |
| INV-SNOWFREEZE-003 | Snow depth-density domain bounds: `Dsold >= 0`, `Dsnew >= 0`, `ρsold >= 0`, `ρsnew >= 0`, and `ρsnew <= 522 kg m^-3`; when `Dsnew = 0`, `ρsnew = 0`. | hard-fail | REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH3-SNOWDENS-LIM, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-004 | Active snow-update branch consistency: fresh snowfall contribution uses `100 kg m^-3` density and active depth/density updates follow Eq. [3.7.1]-[3.7.5] for settling, snowfall, melt, and melt+snowfall cases; drift-term equations remain governance-only while drift is inactive. | hard-fail | REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH3-DRIFT-INACTIVE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-005 | Rain/snow partition consistency: precipitation phase partition follows daily temperature logic (`Tmax < 0` => all snow; `Tmin > 0` => all rain; mixed day uses hourly occurrence/diurnal temperature function). | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-HRPRECIP, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static]` |
| INV-SNOWFREEZE-006 | Frost heat-flow formulation consistency: frost/thaw bookkeeping uses explicit layered heat-flow relations (`Qsrf`, `Quf`) and harmonic-mean layered thermal conductivity per Eq. [3.8.1]-[3.8.4]. Frost top heat flow must consume the legacy `hr_tmp`/`tmpadj` adjusted hourly surface temperature (`surtmp(hour)`), not raw hourly air temperature, with positive snow-covered surface temperatures capped to `0 degC`. Executable frost-depth progression must derive from hourly signed heat flow, latent-heat increments, and the active fine-layer frozen-depth/frozen-water state (`wb18_perc_frozen_depth_####`, `wb18_perc_frzw_####`). Freeze energy advances the front only by freezing layer water into the same store that `frwatc` later publishes; thaw energy retreats the active front by melting that same layer ice back into liquid storage. The surface heat path must be resisted by the current frozen-layer thickness (`Σ dz/k` through snow, residue, tilled frozen soil, and untilled frozen soil), with residue depth following the legacy `resdep` lineage, frozen tilled/untilled conductivity using the fixed legacy `kftill = 1.75` and `kfutil = 2.1 W m^-1 degC^-1` constants, and below-freezing shallow-front soil distance floored to the midpoint of the first fine layer (`dpfsfl = dg(1)/nfine(1)/2`, legacy fallback `0.005 m` only when fine-layer geometry is unavailable). The lower heat path remains a separate `Quf` term computed from the seasonal `tmpbl` curve and the `frostn.for:430-458` per-fine-layer unfrozen harmonic conductivity path over one metre below the frost front (`slsw`, `bdcons`, `ksoilf`; `kufz = 0.2` only when no positive conductivity terms exist), and freeze-active `frzng` execution must recompute surface resistance and `Qsrf` after each in-hour fine-layer front advance before consuming additional freezing time. Depth must be bounded by the physical soil profile and by the layer/fine-layer capacity exposed by that state; it must not use the retired `0.20 m * clamp(mean-temperature / 6 degC)` freeze-index proxy, a `0.20 m` model cap, post-hoc scalar depth projection into layer stores, a scalar `frdp * theta` frozen-water surrogate, raw hourly air temperature as the frost surface-temperature driver, zero-residue publication when management residue is present, per-soil frozen-path conductivity in place of fixed `kftill`/`kfutil`, or a constant lower-front conductivity when positive unfrozen layer terms are available. | hard-fail | REF-SNOWFREEZE-CH3-FROST, REF-SNOWFREEZE-LEGACY-TMPADJ, REF-SNOWFREEZE-LEGACY-FROSTN-QDRY, REF-SNOWFREEZE-LEGACY-WINTER-RESDEP, REF-SNOWFREEZE-LEGACY-FROZEN-PATH-KF | `[DIRECT][Static]` |
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
| INV-SNOWFREEZE-047 | Frost-depth observation-validation correspondence invariant (`GAP-SNOWFREEZE-002`; draft): modeled frost-front depth `frdp` is validated against historic site observations as an external authority (ADR-0017 - observation agreement is the acceptance target; legacy/compatibility frost output is only a flag). The observation-to-`frdp` correspondence is fixed by measurement method and must not be conflated: (a) frost-tube depth (the frozen/unfrozen free-water boundary) is the magnitude authority and compares to `frdp` directly within `TOL-SNOWFREEZE-007`; (b) soil-temperature `0 degC`-isotherm depth is a timing authority and an upper bound on `frdp` (the soil ice front is shallower than the `0 degC` isotherm by the freezing-point-depression band), validated for onset/thaw timing and frozen duration within `TOL-SNOWFREEZE-008` and for magnitude only as `frdp <= isotherm_depth + TOL-SNOWFREEZE-007`; (c) penetrometer/mechanical-resistance depth is method-dependent and is secondary/non-authoritative for magnitude. A frost-depth divergence may be classified `OPENWEPP-DEFECTIVE` only when all hold: (1) modeled snow depth agrees with paired observed snow depth within `TOL-SNOWFREEZE-009` so the snow-insulation confound is controlled; (2) the comparison is like-for-like by method per (a)-(c); (3) censoring is honored - left-censored onset (frost-tube observers begin at 1-2 in of frost) is excluded from onset-timing error and right-censored sensor-depth caps (e.g. SCAN ~1.0 m) are excluded from magnitude error; and (4) the divergence exceeds the tier tolerance over a defined aggregation (seasonal-maximum depth and the observation-date depth series). Divergences failing (1)-(3) are `HARNESS-SURFACE-MISMATCH` or `UNRESOLVED`, never silently a model defect. This invariant validates fidelity only; it does not relax the `INV-SNOWFREEZE-006` heat-flow formulation authority, and every tolerance is provisional pending hydrology-reviewer ratification and the first validation pass. | governance-hold | REF-SNOWFREEZE-FROST-OBS, INV-SNOWFREEZE-006, INV-SNOWFREEZE-012, ADR-0017, TOL-SNOWFREEZE-007, TOL-SNOWFREEZE-008, TOL-SNOWFREEZE-009 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-048 | Snow-depth observation correspondence and anti-alias invariant (`GAP-SNOWFREEZE-002`; draft): the snow-control operand for `INV-SNOWFREEZE-047` is physical snowpack depth, not snow-water equivalent. Modeled depth must be WAT `Snow-Depth` (`mm`) converted from `snow.runtime_depth_m` and may not be substituted with WAT `Snow-Water`, `snow.runtime_swe`, snowfall depth, melt, or any water-storage surface. A snow-control failure may be routed as a snow-depth fidelity issue only after the harness proves all of the following for the paired rows: (1) observed source field semantics are physical snowpack depth with units normalized to `m`; (2) modeled and observed dates represent the same daily stage or any stage difference is explicitly classified; (3) signed residual direction, magnitude, and over/under counts are published, not only absolute residuals; (4) depth-vs-SWE anti-alias evidence shows the failure is not better explained by comparing observed depth to a water-equivalent alias; and (5) missing paired snow rows remain `INCONCLUSIVE`, not a snow or frost defect. When these proofs pass and `TOL-SNOWFREEZE-009` fails, frost-depth attribution stays blocked and the next authorized route is snow-depth fidelity/carry/input/settlement adjudication. When any proof fails, the route is `HARNESS-SURFACE-MISMATCH` or `UNRESOLVED` with a named correspondence blocker. | governance-hold | REF-SNOWFREEZE-FROST-OBS, INV-SNOWFREEZE-047, TOL-SNOWFREEZE-009, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-049 | SNOTEL observed-density correspondence invariant (`GAP-SNOWFREEZE-002`; draft): paired SNOTEL `WTEQ` and `SNWD` rows provide an external-authority snowpack bulk-density surface for snow-depth fidelity adjudication. `WTEQ` is snow-water equivalent in inches and must be normalized to millimeters water equivalent; `SNWD` is physical snowpack depth in inches and must be normalized to meters. Observed bulk density is computed as `observed_density_kg_m3 = observed_swe_mm / observed_snow_depth_m` only when both SWE and physical depth are positive and like-for-like for the same date. Rows with absent depth, absent SWE, zero/trace depth, or impossible/non-finite density are excluded from density verdicts rather than repaired. The `snow.txt` SSD comparison arm may use an observed climatological density only when it is derived before residual comparison from peak-SWE-period SNOTEL density and documented with the derivation; choosing SSD by minimizing modeled-vs-observed depth residual is invalid. Legacy WEPP and PySnobal remain diagnostic flags under ADR-0017; observed SNOTEL SWE/depth/density plus this correspondence are the adjudication authority for the over-accumulation vs low-density fork. Density and SWE tolerances remain provisional under `TOL-SNOWFREEZE-010`; failure routes to snow-depth/density fidelity characterization, not directly to frost or production physics edits. | governance-hold | INV-SNOWFREEZE-003, INV-SNOWFREEZE-048, TOL-SNOWFREEZE-009, TOL-SNOWFREEZE-010, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-050 | Snow/frost fidelity evaluation-rubric invariant (`GAP-SNOWFREEZE-002`; draft): model fidelity to snow/frost observations is assessed by the signature-based, multi-timescale rubric in the GAP-SNOWFREEZE-002 Snow/Frost Fidelity Evaluation Rubric Addendum, not by any single residual tolerance. The rubric decomposes the comparison into process-diagnostic signatures across long-term, seasonal, and event timescales (accumulation, densification/settling, peak magnitude and timing, ablation, rain-on-snow, regime ordering, conservation; for frost: onset, deepening, thaw, frozen duration). Each signature is tagged forcing-robust (`R`) or forcing-limited (`L`): `R` signatures (bulk density, densification trajectory, depth-SWE slope, timing, regime ordering, conservation) are intensive or relative quantities that survive the forcing and representativeness uncertainty budget and carry model verdicts; `L` signatures (absolute peak SWE and depth magnitude) are reported but may not by themselves produce an `OPENWEPP-DEFECTIVE` verdict because they are dominated by gridded-precip, lapse/spatialization, and point-vs-hillslope representativeness error. Time-series cells score by Kling-Gupta-Efficiency decomposition into correlation, bias-ratio, and variability-ratio so the failed mode (timing vs magnitude vs spread) is named rather than smeared; magnitude cells score by median signed bias and IQR; timing cells by date offset. Cell pass-levels are provisional noise-floor estimates under `TOL-SNOWFREEZE-011` and the per-quantity bands `TOL-SNOWFREEZE-007/008/009/010`, calibrated to the uncertainty budget and not fidelity targets. The output is a per-model, per-site, per-cell profile, not a scalar; the ADR-0017 verdict taxonomy is applied per cell; legacy WEPP and PySnobal are scored on the same rubric as diagnostic flag profiles, never as targets. This invariant governs evaluation method only, changes no physics, and supersedes use of `TOL-SNOWFREEZE-009` as a standalone snow-model acceptance band. | governance-hold | INV-SNOWFREEZE-047, INV-SNOWFREEZE-048, INV-SNOWFREEZE-049, REF-SNOWFREEZE-FROST-OBS, ADR-0017, TOL-SNOWFREEZE-011 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-051 | SNOWDENSITY-02 opt-in `physics_bulk` snow-model envelope: openWEPP may develop a deliberate-legacy-divergence snow-density candidate only behind `snow_model = legacy_wepp | physics_bulk`, with `legacy_wepp` remaining the default and `physics_bulk` explicitly opt-in until a later activation package passes this contract's gates. `physics_bulk` is a candidate physics-bulk snowpack model, not an authorized production formula in this amendment. Its candidate state envelope includes SWE, physical depth, bulk density, retained liquid water, snow cold content or bulk temperature, and snow-cover age/metamorphism clock. Its candidate process envelope may include temperature/wind-dependent fresh-snow density, Anderson-1976/SNOBAL-style metamorphism, overburden compaction, wet-snow compaction, liquid retention/release/refreeze, and internal mass/energy closure, but exact equations/constants require SNOWDENSITY-03/04 evidence and hydrology-reviewer ratification before runtime promotion. Site-specific calibration is prohibited: SNOTEL, observed snow depth, legacy WEPP, and PySnobal may classify profile cells and diagnose failure modes, but they may not fit per-site constants, choose SSD by residual minimization, or tune `physics_bulk` separately for a site. `ssd` remains a legacy control-state/provenance input, not a `physics_bulk` density or settlement parameter. PySnobal/SNOBAL and legacy WEPP remain diagnostic flag profiles under ADR-0017, never acceptance targets. Frost physics attribution remains blocked until snow-control failures are passable or bounded by the v74/v75 rubric. | governance-hold | INV-SNOWFREEZE-048, INV-SNOWFREEZE-049, INV-SNOWFREEZE-050, REF-SNOWFREEZE-SNOWDENSITY01, REF-SNOWFREEZE-ANDERSON1976-CANDIDATE, REF-SNOWFREEZE-SNOBAL-CANDIDATE, ADR-0017, ADR-0026, ADR-0027 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-052 | SNOWDENSITY-05A CoE melt-modernization contract and sign-convention envelope: openWEPP may develop a deliberate-legacy-divergence melt candidate only behind `snow_melt_model = legacy_coe | coe_shortwave_albedo_v1`, with `legacy_coe` remaining the default, compatibility comparator surface, and rollback path. `coe_shortwave_albedo_v1` is opt-in only and is not production-authorized until SNOWDENSITY-05B/05C/05D bind the shortwave source, albedo state, implementation, and conservation evidence. The modernized path must preserve the WEPP Chapter 3 / Corps-of-Engineers melt-term family (`amelt`, `bmelt`, `cmelt`, `dmelt`), current canopy attenuation `(1 - cancov)`, signed raw melt, corrected daily redistribution, density gate, and routed-melt lineage. Sign convention is fixed before code: WEPP Chapter 3 prose writes this term as `- bmelt`, while openWEPP trace field `melt_bmelt_in` stores the signed `bmelt` contribution, so the runtime raw-melt identity is `hrmelt_raw = 0.0254 * (amelt + melt_bmelt_in + cmelt + dmelt)` as guarded by `tests/integration/clim05_snow_runtime_kernel_contract.rs`; any silent sign flip or double subtraction is invalid without a contract amendment. `dense_slow_melt_v1` remains a negative benchmark, not a promotable production melt path, because its profile improvement reduced a degree-day melt factor to mask a density gap. Shared radiation forcing must not be tuned or rescaled for snowmelt because `SC-CLIMATE-001#INV-CLIMATE-013` owns the winter radiation unit/provenance seam and the same forcing drives other hydrology/ET consumers. | governance-hold | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-MELT-ASSUMP, SC-CLIMATE-001#INV-CLIMATE-013, REF-SNOWFREEZE-MELT-OHMURA2001, REF-SNOWFREEZE-MELT-PELLICCIOTTI2005, REF-SNOWFREEZE-MELT-CARENZO2009, REF-SNOWFREEZE-MELT-BROCK2000, REF-SNOWFREEZE-MELT-WALTER2005, REF-SNOWFREEZE-MELT-GUPTA2023, INV-SNOWFREEZE-015, INV-SNOWFREEZE-050, INV-SNOWFREEZE-051 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-053 | SNOWDENSITY-05B shortwave source/provenance binding: the canonical openWEPP shortwave source is the daily climate `rad`/`radly` field in `Ly d^-1` at the climate parser/runtime seam. Upstream gridded radiation products may feed openWEPP only by being normalized by orchestration into this field with external provenance; openWEPP must not fetch, select, spatialize, or tune gridded shortwave products or maintain a snow-only radiation source. Runtime transformation is exactly the `SC-CLIMATE-001#INV-CLIMATE-013` lineage: preserve `radly` as `Ly d^-1`, perform the single conversion `radmj = radly * 0.04184`, apply slope/aspect transformation through `sunmap`, distribute hourly through `radcur`/`hr_tmp` including the near-isothermal `radmj/24` branch, and publish `winter.hourly.rad_mj_m2_####` in `MJ m^-2 h^-1`. ET and snowmelt consume the same daily radiation authority (`rad`/`RA`/`radiation_ly`); any fitted radiation scalar, already-MJ double conversion, Langley-scale value under an MJ label, silent clipping, or snow-only radiation scalar is invalid. | governance-hold | SC-CLIMATE-001#INV-CLIMATE-013, SC-EVAP-001#INV-EVAP-021, REF-SNOWFREEZE-CH3-HRRAD, REF-SNOWFREEZE-CH3-MELT, INV-SNOWFREEZE-052 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-054 | SNOWDENSITY-05C albedo-state core: the accepted opt-in albedo model for future `coe_shortwave_albedo_v1` implementation is `brock2000_temperature_age_v1`, a temperature-age snow-albedo state derived from Brock et al. (2000) and not fitted to openWEPP SNOTEL or frost-site observations. The state carries `snow_albedo`, `snow_albedo_model_id`, and `snow_albedo_accumulated_positive_temperature_c_day` (`Ta`) and must reset `Ta` when fresh snowfall meets `snow_albedo_fresh_snow_reset_water_equiv_m`. For active snow cover, compute deep-snow albedo as `0.713 - 0.112 * log10(Ta)`, shallow-snow albedo as `underlying_albedo + 0.442 * exp(-0.058 * Ta)`, and combine them with `alpha = (1 - exp(-d / d_star)) * alpha_deep + exp(-d / d_star) * alpha_shallow`, where `d` is snow water-equivalent depth and `d_star = 0.024 m water equivalent`. Computed albedo is bounded to `0 <= snow_albedo <= 0.85`; the upper cap is Brock's reported fresh-snow bound and the lower bound is physical. `legacy_coe` must not consume or require this state. `coe_shortwave_albedo_v1` on active snow must fail closed on missing opt-in albedo state, missing model id, non-finite/negative `Ta`, non-finite/out-of-range albedo, or model-id mismatch; it must not silently synthesize a hidden default. This invariant ratifies the albedo state core only and does not authorize routed-melt wiring, default activation, radiation-source changes, or coefficient fitting. | hard-fail | REF-SNOWFREEZE-MELT-BROCK2000, INV-SNOWFREEZE-052, INV-SNOWFREEZE-053 | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| `Thra`/`surtmp(hour)` | `frost.hourly.surface_temp_c_{idx4}` | hourly adjusted surface-temperature forcing consumed by frost top heat flow after legacy `hr_tmp`/`tmpadj` synthesis | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
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
- `physics_bulk` selected as a default model, coupled into production runtime, or parameterized from per-site fitted constants before a ratified follow-on package proves the `INV-SNOWFREEZE-051` promotion gates. `[DIRECT][Static] + [INFERENCE][Static]`
- `coe_shortwave_albedo_v1` selected as a default melt model, activated without SNOWDENSITY-05B/05C/05D source/albedo/implementation gates, or executed with missing albedo/source provenance. `[DIRECT][Static] + [INFERENCE][Static]`
- `dense_slow_melt_v1` or any degree-day snowbench variant promoted into production melt physics. `[INFERENCE][Static]`
- Shared radiation forcing tuned, rescaled, clipped, or reinterpreted to improve snowmelt fit instead of preserving `SC-CLIMATE-001#INV-CLIMATE-013` unit/provenance authority. `[DIRECT][Static] + [INFERENCE][Static]`
- openWEPP runtime or kernel code fetching, selecting, or spatializing gridded shortwave products instead of consuming the normalized climate `rad`/`radly` seam. `[INFERENCE][Static]`
- A snow-only radiation scalar, parser column, or fitted multiplier feeding `coe_shortwave_albedo_v1` while ET and other hydrology consumers see a different daily radiation authority. `[DIRECT][Static] + [INFERENCE][Static]`
- Treating already-`MJ m^-2 d^-1` radiation as `radly`, double-converting a daily MJ value, publishing Langley-scale values under `winter.hourly.rad_mj_m2_####`, or clipping high radiation rather than failing `SC-CLIMATE-001#INV-CLIMATE-013`. `[DIRECT][Static] + [INFERENCE][Static]`
- `coe_shortwave_albedo_v1` proceeding on active snow with missing opt-in albedo state, missing `snow_albedo_model_id`, non-finite/negative `snow_albedo_accumulated_positive_temperature_c_day`, or `snow_albedo` outside `[0, 0.85]`. `[DIRECT][Static] + [INFERENCE][Static]`
- `legacy_coe` default runs requiring, mutating, or consuming `snow_albedo` state. `[INFERENCE][Static]`
- Any albedo constants or reset thresholds fitted to SNOTEL, frost-site observations, legacy residuals, or PySnobal residuals. `[INFERENCE][Static]`
- `melt_bmelt_in` sign semantics changed by silent sign flip or double subtraction without a new contract amendment and source-line proof. `[DIRECT][Static] + [INFERENCE][Static]`

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
- OBL-SNOWFREEZE-P-026: Any `physics_bulk` producer must remain opt-in,
  preserve `legacy_wepp` as default and rollback path, expose typed SWE,
  physical depth, density, liquid-water, and thermal/cold-content state, prove
  internal mass and energy closure from independent operands, reject per-site
  tuned constants, and publish v74/v75 rubric profiles before runtime
  activation or default-candidate promotion.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-027: Any `coe_shortwave_albedo_v1` producer must preserve
  `legacy_coe` as default and rollback path, preserve the signed
  `melt_bmelt_in` convention, publish typed melt-model selector, shortwave
  source/provenance, albedo state, albedo model id, term-level melt operands,
  signed raw melt, corrected redistributed melt, and routed `wmelt` operands,
  prove independent raw/routed-melt and SWE-loss closure, reject radiation
  forcing retuning and per-site fitted defaults, and keep degree-day snowbench
  variants as negative benchmarks only until a later contract amendment says
  otherwise.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-028: Any `coe_shortwave_albedo_v1` producer or
  pre-implementation package must publish a source/provenance ledger naming the
  daily `rad`/`radly` input, units, upstream gridded/provider provenance when
  available, single-conversion path, slope/aspect and hourly transformation
  lineage, ET shared-authority proof, and anti-alias rejections. It must close
  `HOLD` rather than implement if the only available path requires openWEPP to
  fetch/select/spatialize gridded radiation, tune radiation to snow, or add a
  snow-only radiation scalar.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-029: Any `brock2000_temperature_age_v1` albedo-state
  producer must expose typed `snow_albedo`, `snow_albedo_model_id`, accumulated
  positive-temperature age, fresh-snow reset threshold, snow water-equivalent
  depth, underlying-surface albedo, and reset/decay decision evidence. It must
  prove albedo bounds, monotonic decay under positive-temperature aging on deep
  snow, fresh-snow reset behavior, missing-state fail-closed behavior for
  `coe_shortwave_albedo_v1`, and no effect on `legacy_coe` default runs before
  any routed-melt implementation consumes the state.
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
| `physics_bulk` snow-density candidate envelope (`INV-SNOWFREEZE-051`) | Contract, offline snowbench, and future runtime activation review | Governance `HOLD` unless `physics_bulk` remains opt-in, no-site-tuning is proven, candidate equations/constants are ratified, mass/energy closure is independently reconstructed, and v74/v75 rubric profiles show forcing-robust improvement or a documented non-promotion reason | SNOWDENSITY-02 and successors | `[DIRECT][Static] + [INFERENCE][Static]` |
| `coe_shortwave_albedo_v1` melt-modernization envelope (`INV-SNOWFREEZE-052`) | Contract, source/albedo/implementation gates, and future runtime activation review | Governance `HOLD` unless the path remains opt-in, signed `melt_bmelt_in` semantics are preserved, `SC-CLIMATE-001#INV-CLIMATE-013` radiation provenance is proven, albedo/source operands are typed, no-site-tuning is proven, and raw/routed melt plus SWE loss are independently reconstructed | SNOWDENSITY-05A through 05F | `[DIRECT][Static] + [INFERENCE][Static]` |
| Shortwave source binding (`INV-SNOWFREEZE-053`) | Contract, source/provenance ledger, climate/ET anti-alias review, and future opt-in melt implementation | Governance `HOLD` unless `coe_shortwave_albedo_v1` consumes the existing climate `rad`/`radly` daily radiation authority, proves the single `radly -> radmj -> sunmap/radcur/hr_tmp -> winter.hourly.rad_mj_m2_####` path, shares radiation authority with ET, and rejects snow-only/fitted/scaled/clipped/double-converted radiation | SNOWDENSITY-05B and successors | `[DIRECT][Static] + [INFERENCE][Static]` |
| Albedo state core (`INV-SNOWFREEZE-054`) | Contract, typed albedo update core, unit tests, and future opt-in melt implementation | Hard error for missing/invalid opt-in albedo state under active `coe_shortwave_albedo_v1`; governance `HOLD` for production melt wiring until 05D independently proves raw/routed melt and downstream liquid closure | SNOWDENSITY-05C and successors | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| TOL-SNOWFREEZE-007 | Frost-depth magnitude acceptance band (frost-tube authority; `INV-SNOWFREEZE-047`) | provisional: greater of `0.10 m` or `25 %` of observed seasonal-maximum depth | DRAFT validation band (not a comparator FP tolerance). To be calibrated from the first `tests/fixtures/snowfreeze_observed/` validation pass and ratified by the hydrology reviewer; bounded below by frost-tube read/registration uncertainty. |
| TOL-SNOWFREEZE-008 | Frost onset/thaw timing and frozen-duration acceptance band (`INV-SNOWFREEZE-047`) | provisional: `+/- 14 days` | DRAFT. Bounded by observation cadence (frost tubes/penetrometer read 2-4x/month to biweekly), so sub-fortnight timing is unresolvable from the data; not a runtime tolerance. |
| TOL-SNOWFREEZE-009 | Snow-insulation confound-control band for attributing a frost-depth divergence (`INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`) | provisional: paired modeled-vs-observed snow depth within greater of `0.10 m` or `30 %` | DRAFT. Beyond this band the frost-depth comparison is inconclusive (snow-driven), not a frost-model verdict; snow depth and density, not SWE, govern insulation. This band is not by itself a snow-model calibration target; `INV-SNOWFREEZE-048` must first prove like-for-like snow-depth correspondence and anti-alias evidence. |
| TOL-SNOWFREEZE-010 | SNOTEL SWE/density fork-adjudication band (`INV-SNOWFREEZE-049`) | provisional: SWE within greater of `0.05 m` water equivalent or `25 %`; density within greater of `60 kg m^-3` or `25 %` | DRAFT characterization band for separating over-accumulation from low-density/settling structure. It is not a calibration objective and does not authorize SSD residual fitting, production constant edits, or PySnobal/legacy-as-target decisions. |
| TOL-SNOWFREEZE-011 | Snow/frost rubric cell pass-levels and forcing-robustness tiering (`INV-SNOWFREEZE-050`) | provisional noise-floor: forcing-robust (`R`) cells score `pass` at KGE `>= 0.6` (correlation, bias-ratio, variability-ratio each near 1) and `marginal` at `0.3-0.6`; timing cells within `+/- 14 days`; density/regime per `TOL-SNOWFREEZE-010`. Forcing-limited (`L`) magnitude cells are reported only - a peak SWE/depth median bias up to `~30 %` is uncertainty-consistent, not a defect. | DRAFT. Pass-levels are noise-floor estimates calibrated to the rubric addendum uncertainty budget, refined from the `tests/fixtures/snotel_observed/` corpus, pending hydrology-reviewer ratification; evaluation bands, not calibration objectives (the `INV-SNOWFREEZE-049` anti-tuning rule applies). |

## Snow-Density Physics-Bulk Candidate Envelope Addendum

1. The only accepted snow-model selector values are
   `snow_model = legacy_wepp | physics_bulk`. `legacy_wepp` is the default,
   current production behavior, compatibility comparator surface, and rollback
   path. `physics_bulk` is opt-in candidate scope only until a later package
   ratifies exact equations, validates profile evidence, and explicitly changes
   activation status.
2. `physics_bulk` must carry at least SWE, physical snow depth, bulk density,
   retained liquid water, snow cold content or bulk temperature, and a snow-cover
   age/metamorphism clock as typed state. A bulk model may use a single-column
   snowpack state at first; layer-resolved snow physics is not required by this
   amendment.
3. Candidate process families are limited to temperature/wind-dependent
   fresh-snow density, Anderson-1976/SNOBAL-style destructive metamorphism,
   overburden compaction, wet-snow compaction, liquid retention/release/refreeze,
   and internally closed mass/energy accounting. These names define the
   allowable research envelope only; no equation or constant is a production
   authority until SNOWDENSITY-03/04 evidence and hydrology review ratify it.
4. Site-specific tuning is invalid. SNOTEL and paired snow-depth observations
   evaluate profile cells and classify failure modes; they do not fit per-site
   constants. Legacy WEPP and PySnobal are diagnostic flag profiles under
   ADR-0017, not targets. `ssd` remains legacy control provenance and cannot be
   repurposed as a `physics_bulk` density/settlement parameter.
5. A promotable `physics_bulk` implementation must prove independent SWE,
   depth, density, retained-liquid, and thermal-state closure; publish v74/v75
   rubric profiles; preserve `legacy_wepp` rollback; and leave frost-depth
   attribution blocked unless snow-control failures are passable or explicitly
   bounded.

## SNOWDENSITY-05A CoE Melt Modernization Contract Addendum

Status: draft (2026-06-26). This addendum scopes the melt-modernization
contract/sign gate. It changes no production physics, constants, parser
surfaces, output schemas, or default behavior. Production implementation is
deferred to the SNOWDENSITY-05B/05C/05D ladder.

1. Accepted melt selector shape is
   `snow_melt_model = legacy_coe | coe_shortwave_albedo_v1`. `legacy_coe`
   remains the default, compatibility comparator surface, and rollback path.
   `coe_shortwave_albedo_v1` is opt-in only.
2. The production target family remains the WEPP Chapter 3 / CoE melt equation
   lineage, not the SNOWDENSITY-04 degree-day snowbench branch. The modernized
   path may alter only the shortwave/albedo operand family after its source and
   state are separately ratified. It must preserve the existing canopy
   attenuation `(1 - cancov)`, `cmelt`/rain terms, signed raw melt, daily
   redistribution, density gate, and routed `wmelt` lineage.
3. Sign convention is bound before code: WEPP Chapter 3 prose writes the
   energy term as `amelt - bmelt + cmelt + dmelt`, while openWEPP stores the
   already-signed contribution in `melt_bmelt_in`. Therefore the executable
   raw-melt identity for current trace fields is
   `hrmelt_raw = 0.0254 * (amelt + melt_bmelt_in + cmelt + dmelt)`. A later
   implementation may rename terms for clarity only if it keeps this identity
   or amends the contract with source-line proof; silent sign flip or double
   subtraction is invalid.
4. `dense_slow_melt_v1` remains a negative benchmark. Its profile improvement
   came from reducing a degree-day melt factor and masking a density gap; it is
   not a promotable production melt model and must not be used as the opt-in
   runtime path.
5. Shared radiation forcing must not be tuned, rescaled, clipped, or
   reinterpreted to improve snowmelt fit. SNOWDENSITY-05B owns source/provenance
   binding under `SC-CLIMATE-001#INV-CLIMATE-013`, and that forcing also feeds
   non-snow hydrology/ET consumers.
6. SNOWDENSITY-05C owns albedo state selection and constants. Brock-style
   temperature/age albedo is a leading candidate only; no albedo default,
   coefficient, or state update is production authority from this 05A
   amendment.
7. SNOWDENSITY-05D may implement the opt-in melt path only after 05B and 05C
   are green. Its gate must independently reconstruct raw melt, redistributed
   melt, routed `wmelt`, SWE loss, and downstream liquid forcing from typed
   operands without aliasing SWE, depth, density, or observation residuals.

## SNOWDENSITY-05B Shortwave Source Binding Addendum

Status: draft (2026-06-26). This addendum scopes the radiation-source binding
gate for the opt-in `coe_shortwave_albedo_v1` melt path. It changes no
production physics, constants, parser surfaces, output schemas, runtime
defaults, or upstream forcing ownership.

1. Source decision: openWEPP's canonical shortwave acceptance point is the
   existing daily climate `rad`/`radly` field in `Ly d^-1`. The engine may use
   gridded radiation only after orchestration has normalized it into that
   climate field and preserved provider/spatialization provenance externally or
   through a typed provenance ledger. openWEPP must not fetch, select,
   spatialize, or tune gridded shortwave products.
2. Transformation decision: the executable lineage for winter melt forcing is
   `SC-CLIMATE-001#INV-CLIMATE-013`: `radly` remains `Ly d^-1`, `radmj =
   radly * 0.04184` is applied exactly once, `sunmap` performs the slope/aspect
   daily transformation, `radcur`/`hr_tmp` produce hourly radiation, the
   near-isothermal branch uses `radmj/24`, and publication is
   `winter.hourly.rad_mj_m2_####` in `MJ m^-2 h^-1`.
3. Coupling decision: ET and snowmelt consume the same daily radiation
   authority (`rad`/`RA`/`radiation_ly`). A separate snow-only radiation scalar,
   parser field, fitted radiation multiplier, or site-specific radiation
   correction is invalid because it breaks `SC-EVAP-001#INV-EVAP-021` and the
   no-radiation-tuning guard from `INV-SNOWFREEZE-052`.
4. Anti-alias decision: Langleys/day, `MJ m^-2 d^-1`, and hourly
   `MJ m^-2 h^-1` must remain distinct. A package or runtime path that treats
   already-MJ daily radiation as `radly`, double converts daily radiation,
   publishes Langley-scale values under `winter.hourly.rad_mj_m2_####`, clips a
   high source value, or changes shared radiation to improve snowmelt fit must
   fail or close `HOLD`.
5. Handoff: SNOWDENSITY-05C may ratify albedo state and constants against this
   source binding. SNOWDENSITY-05D may implement production opt-in melt only
   after 05C is complete and must preserve this source/provenance lineage.

## SNOWDENSITY-05C Albedo State Core Addendum

Status: draft (2026-06-26). This addendum ratifies the opt-in albedo-state core
for `coe_shortwave_albedo_v1`. It changes no production routed melt, runtime
default, radiation source, parser surface, or output schema.

1. Accepted albedo model id: `brock2000_temperature_age_v1`. The model is
   authority-derived from Brock et al. (2000) and is not calibrated to
   openWEPP SNOTEL, frost-site, legacy, or PySnobal residuals.
2. State variables: `snow_albedo`, `snow_albedo_model_id`,
   `snow_albedo_accumulated_positive_temperature_c_day`, and
   `snow_albedo_fresh_snow_reset_water_equiv_m`. `Ta` is reset by a material
   fresh-snow increment, then ages by non-negative accumulated positive
   temperature.
3. Formula authority: deep snow uses `0.713 - 0.112 * log10(Ta)`; shallow snow
   uses `underlying_albedo + 0.442 * exp(-0.058 * Ta)`; the depth transition
   uses `d_star = 0.024 m water equivalent`. Computed albedo is constrained to
   `[0, 0.85]`, with upper cap `0.85`.
4. Fail-closed rule: when `snow_melt_model = coe_shortwave_albedo_v1` and
   active snow has no fresh-snow reset, the previous opt-in albedo state and
   model id are required. Missing state is a contract violation, not a reason to
   synthesize a hidden default.
5. Rollback rule: `legacy_coe` remains default and does not require or consume
   this state. The albedo core may be tested independently, but routed-melt
   consumption is SNOWDENSITY-05D scope.

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

### FDHP01 Fine-State Aliases and Incremental Authority

FDHP01 Increment A authorizes behavior-preserving shadow publication of the
baseline fine-layer frost state before it drives depth or WAT publication. The
shadow state is a conservation and handoff proof surface only: active `frdp`,
`frozwt`, `wb18_perc_frozen_depth_####`, `wb18_perc_frzw_####`, and
water-balance publication remain unchanged until the subsequent freeze/thaw
increments rebind authority.

| Legacy symbol | openWEPP alias | Incremental rule |
|---|---|---|
| `fgfrst(j,i)` | `frost.runtime_fgfrst_LLLL_FFFF` | Fine-layer frost flag, persisted as diagnostic state with integer domain `0..3`; it must not drive active depth in Increment A. |
| `slfsd(j,i)` | `frost.runtime_slfsd_m_LLLL_FFFF` | Fine-layer frozen thickness in metres; aggregate shadow handoff must keep `0 <= slfsd <= dzfine`. |
| `slsic(j,i)` | `frost.runtime_slsic_m_LLLL_FFFF` | Fine-layer ice water-equivalent store; shadow `frwatc(0)` derives `soilf`/`frzw` diagnostics from this sum. |
| `slsw(j,i)` | `frost.runtime_slsw_theta_LLLL_FFFF` | Fine-layer liquid volumetric water over unfrozen thickness; `frwatc(1)` applies the daily `st - yst` delta to this surface and `nwfrzz`. |
| `sltime(j,i)` | `frost.runtime_sltime_s_LLLL_FFFF` | Hour-local redistribution timer; Increment A resets/publishes it as zero diagnostic state. |
| `yst(i)` | `frost.runtime_yst_m_LLLL` | Prior day-end active storage for the next hour-1 `frwatc(1)` delta. |
| `nwfrzz(i)` | `frost.runtime_nwfrzz_m_LLLL` | Liquid water residing in frozen zones before `frznw`; shadow handoff updates it without changing active stores. |
| `frzflg` | `frost.hourly.frzflg_####` | Hourly branch diagnostic with legacy integer domain `0..4`; Increment B uses it to prove freeze/thaw arm selection before the full thaw-arm port. |

Increment A must also publish an internal conservation residual proving the
handoff seam:
`Delta(fine liquid + nwfrzz + slsic) == st - yst` when no freeze/thaw arms are
bound to the shadow state. Any residual beyond roundoff is an implementation
hard stop before the state is allowed to drive depth or publication.

FDHP01 Increment B rebinds active freeze-depth authority to this fine-layer
state. During freeze-active hours, `frzng`/`frznw` semantics must mutate
`slfsd`, `slsic`, `slsw`, and `nwfrzz` first; `frdp`, `thdp`, `tfrdp`, and
`tthawd` are then derived by scanning `fgfrst`/`slfsd` equivalent to
`watdst.for:300-511`. `frznw` must freeze liquid already held in frozen zones
before ordinary front extension, respecting `(thetdr + ul/dg) * slfsd - slsic`
capacity and preserving total layer water. When exchange summation would debit more
liquid than available by no more than the kernel zero threshold
(`WB11_ZERO_THRESHOLD`, metres), the freeze debit may be limited to available
liquid at the handoff boundary; larger overruns are hard domain violations.
Production code must not advance scalar `frdp` and then project the target
depth into layer mass. The former
`apply_layered_frost_target` lineage is retained only as historical evidence
for the D2 repair and is not production authority after Increment B.

Increment B does not complete `mlttp`/`mltbtm` sandwich and thaw-through
authority. Any surviving thaw behavior in this increment is a bounded
carry-over/minimal retreat path and must remain recorded as incomplete until
Increment C ports the top/bottom thaw arms and closes D3 acceptance.

FDHP01 Increment C1b authorizes the water-side capacity and overflow
infrastructure required before the thaw arms may be retained. `frwatc(1)` and
freeze-active `frzng`/`frznw` movement must keep fine-layer ice within
`(thetdr(i) + ul(i)/dg(i)) * slfsd(j,i) - slsic(j,i)` capacity and fine-layer
unfrozen liquid within the same total pore capacity
`slsw(j,i) <= thetdr(i) + ul(i)/dg(i)`. Existing or produced states that
exceed those capacities beyond roundoff are hard domain violations; production
code must not silently clamp them. Excess valid incoming liquid must move
through the `watdst`-owned fine-layer redistribution/overflow path, preserving
single ownership by the fine state between `frwatc(1)` ingress and
`frwatc(0)` egress. Unretained lower overflow is published as
`frost.runtime_watbtm_m` and enters WB13 `Dp`; upper overflow is published as
`frost.runtime_watpdg_m` and remains an explicit closure surface for the C2
top-thaw path. WB13 may canonicalize non-negative `D` and
`frost.runtime_watbtm_m` publication roundoff at or below `1e-11 m` to zero
before summing `Dp`; WB18 percolation applies the same threshold before state
debit/publication so zero-published deep-percolation dust cannot remove
storage. WB18 may also rebalance no-flux scalar/layer storage roundoff at or
below `2e-11 m` by adjusting layer storage to the preserved incoming
`wb11_soil_water` scalar; this storage-only tolerance must not canonicalize or
hide positive deep percolation, frost overflow, negative values, or non-finite
values. Larger positive values are preserved, and any negative or non-finite
value remains a typed failure. The internal handoff residual accounts for
overflow as `after + watpdg + watbtm - before - (st - yst)` so valid overflow
does not masquerade as a storage leak.

FDHP01 Increment C2 completes the thaw-arm ownership rules required by
`INV-SNOWFREEZE-012`. Bottom thaw (`mltbtm`) must consume lower-front thaw
energy against the active fine-layer `slsic` store, reduce `slfsd` from the
bottom of the frozen zone, leave partially thawed layers in frost-at-top
geometry (`fgfrst=2`), release proportional `nwfrzz` liquid from thawed frozen
thickness, and route any liquid that cannot be retained under the C1b fine-layer
capacity through `watbtm`. Top thaw (`mlttp`) must consume positive surface
thaw energy against the same `slsic` store from the soil surface downward,
leave partially thawed layers in frost-at-bottom geometry (`fgfrst=3`), publish
positive `thdp` while remaining frost persists beneath the thawed surface zone,
route unretained upper liquid through `watpdg`, and set `fgthwd=1` when the fine
state thaws through so early `frwatc(0)` semantics can recompute the coarse
water-balance stores. Mixed/sandwich arms retain legacy directionality: arm 2
uses top freezing with optional bottom thaw when `qdry > 0`; arm 3 uses top thaw
with optional bottom thaw when `qdry > 0`; arm 4 is bottom thaw only. Repeated
freeze/thaw cycles without external water input must not amplify
`Total-Soil + frozwt + watpdg + watbtm`; any overflow must remain a named WAT
identity surface rather than hidden storage. Gross `frwatc` freeze-debit and
thaw-credit diagnostics remain frozen-store motions; the net liquid-delta
diagnostic is the actual scalar handoff after fine-state aggregation
(`liquid_after - liquid_before`) and need not equal the gross thaw credit when
capacity redistribution or overflow routing changes the retained liquid store.

FDHP01 Increment Db binds freeze-arm front advance to the legacy `frzng`
in-hour resistance feedback required by `INV-SNOWFREEZE-006`. During a
freeze-active hour, openWEPP must not compute `Qsrf` once from the start-hour
frozen depth and spend the resulting energy across multiple fine layers.
After each fine-layer advance, the newly frozen tilled/untilled path must grow
the surface resistance term and the next freeze-energy slice must use the
updated `Qsrf`, matching `frzng.for:235-240`, `frzng.for:287-305`, and
`frzng.for:334-335`. This Db rule preserves the C1b/C2 state ownership and
capacity gates; it does not authorize thaw, storage, publication, or unit
conversion changes.

FDHP01 Increment De binds the lower-front heat term and thaw energy spending
to the same seasonal/fine-layer authority without reopening the D2 storage
identity. The stable `7 degC` lower-front surrogate and the interim constant
`kufz = 0.2` accepted in Dc1 are retired for positive-conductivity frost heat
flow. Bottom heat must instead use the legacy `tmpcft`/`tmpfun` annual monthly
temperature curve and `frostn.for:386-397`/`frostn.for:430-458` form:
`tmpbl = YavgT + YampT * exp(-(frdp + 1.0)/2.0) *
sin(2*pi/365*(sdate - YpshfT) - (frdp + 1.0)/2.0)`, with `Qdry = 0` when
`tmpbl <= 0`, otherwise `Qdry = kufz * tmpbl / 1.0`. For `tmpbl > 0`, `kufz`
must be the harmonic path through one metre of unfrozen fine layers below the
front, using each fine layer's current `slsw` and parent-layer `bdcons`:
`k = (0.5096 + 7.4493*slsw - 8.7484*slsw^2) *
(0.0014139*bdcons - 1.0588) * ksoilf`, accumulating `Σ(dz/k)` over positive
terms. The legacy fallback `kufz = 0.2 W m^-1 degC^-1` is allowed only when no
positive terms exist; it must not be used as a standing surrogate or silently
replaced by the old `kfutil * ksoilf` form. Top and bottom thaw must spend energy in
in-hour slices that recompute the active resistance/front geometry after each
fine-layer retreat, matching the legacy `mlttp`/`mltbtm` resistance-feedback
shape instead of spending the start-hour flux across multiple fine layers.
Fine-layer liquid theta values may be canonicalized to the residual lower
bound only for finite roundoff within `1e-10` volumetric theta at the
read/post-compute/publication boundary. Material sub-residual values remain a
typed domain violation. The Dc1 acceptance boundary is conservation-first:
years 2-6 `Total-Soil + frozwt` closure must remain at the Db WAT-publication
texture, while depth/duration changes from the known snow-insulation F4 seam
are recorded but not accepted as D3 closure.

FDHP01 Increment Dg binds the surface-resistance terms localized by Df. Winter
residue depth consumed by frost must be the legacy `resdep` lineage, not a
zero frost-sidecar default: cropland initial/current ground residue mass is
converted through `res_dp.for` pithy/woody/hollow factors and published at
`frost.runtime_residue_depth_m` for the frost heat path. For below-freezing
surface conditions, the top frozen soil conduction distance must not be less
than `dpfsfl`, the midpoint of the first fine layer; `0.005 m` is only the
legacy fallback when fine-layer geometry is unavailable. These terms are
surface-resistance corrections and do not authorize any D2 storage identity or
lower-front heat changes.

FDHP01 Increment Dh refutes the proposed per-soil frozen-path conductivity
port. Pinned-source inspection shows `kftill` and `kfutil` are fixed legacy
constants in the active `frostn`/`frzng`/`frznw` surface resistance path. The
soil-property-dependent conductivity term involving `bdcons`, `slsw`, and
`ksoilf` is the lower-front unfrozen `kufzfl` path already bound by Increment
De, not a replacement for frozen tilled/untilled surface-path conductivity.
Implementations must preserve fixed `kftill`/`kfutil` unless a newer canonical
authority supersedes the pinned baseline.

FDHP01 Increment Dj binds frost top heat flow to the legacy `hr_tmp`/`tmpadj`
surface-temperature synthesis. Active frost must consume an adjusted hourly
surface temperature equivalent to `surtmp(hour)`, not raw hourly air
temperature, when computing the `Qsrf`/top-thaw path. The synthesis requires
hourly winter air temperature, radiation, cloud fraction, wind, albedo,
canopy/roughness, snow depth/density, residue depth/conductivity, and current
frost/thaw depth geometry; missing required inputs are fail-closed. Positive
computed surface temperature under snow deeper than `0.001 m` is capped to
`0 degC`, matching `tmpadj.for:362-363` and the companion `frostn.for:467-480`
guard. Runtime projection must therefore emit the winter hourly air,
radiation, and cloud forcing families whenever frost processing is enabled by
runtime frost state or `frost.options.wintRed`, even on warm/no-snow days.
`frost.hourly.surface_temp_c_####` is the frost-consumed adjusted diagnostic;
it does not authorize retuning snow depth/density, residue resistance,
`dpfsfl`, fixed `kftill`/`kfutil`, lower-front `Qdry`, D2 storage, or WAT
publication surfaces.

FDHP01 Increment Dk closes the single-OFE frost-depth heat-flow parity package
at the declared ADR-0017 boundary. The bounded residue pre-check found the
management-derived static initial `frost.runtime_residue_depth_m` clean against
legacy first-row `resdep` for all 43 prefixes within winter-output rounding,
and specifically refuted undervaluation of the lower-residue outlier subgroup.
Any remaining daily `resdep` lifecycle mismatch is a residue/decomposition
producer-surface handoff, not a frost projection defect. Certification consumes
the clean Dj native and forced-snow cohorts: `43/43` clean, years 2-6
`Total-Soil + frozwt` closure at WAT-publication texture, no profile-bound
pinning, depth correlation materially improved from the FDMC01 `0.13` baseline
to about `0.76`, and forced-snow frozen-duration residual collapsed from
`+258` to `+61` days. The legacy depth envelope remains a comparator flag, not
a millimetre target; the stable upper-envelope outlier set is a characterized
handoff rather than an authorization for comparator tuning.

## GAP-SNOWFREEZE-002 Frost-Depth Observation Validation Addendum

Status: draft (2026-06-24). This addendum scopes the observation-anchored
validation method reopened by `GAP-SNOWFREEZE-002` and bound by
`INV-SNOWFREEZE-047`. It changes no physics; `INV-SNOWFREEZE-006` remains the
frost heat-flow formulation authority. All tolerances are provisional pending
hydrology-reviewer ratification and the first validation pass.

### Why observations, not legacy

FDHP01 closed `GAP-SNOWFREEZE-002` at the ADR-0017 conservation/activation
boundary: frost-depth timing/shape improved (depth correlation ~`0.76`,
frozen-duration residual `+61` days) but absolute frost-depth magnitude was never
closed to a physical envelope and was held as a comparator flag. Under ADR-0017
the legacy binary is a flag, not an acceptance oracle, so the missing authority
is external. This addendum supplies it: measured frost depth at instrumented
sites, modeled with the same WEPP hillslope inputs the operator builds through
wepp.cloud.

### Validation substrate

The pilot substrate is `tests/fixtures/snowfreeze_observed/` (see its `README.md`
and per-site `manifest.md`): five single-hillslope WEPP input sets with frost
enabled (`ksflag = 1`), spanning agricultural + forest + rangeland and
frost-tube + soil-temperature authorities, each modeled centroid matched to an
observation site. Observed series are fetched from the datasets in
`REF-SNOWFREEZE-FROST-OBS`; they are not stored in-repo.

### Measurement correspondence (binding under `INV-SNOWFREEZE-047`)

| Observation method | Pilot sites | Relation to model `frdp` | Authority role |
|---|---|---|---|
| Frost tube (frozen/unfrozen free-water boundary) | Sleepers River, GGD498 Morris | Closest field analog to the ice front; direct comparison to `frdp` | Magnitude (`TOL-SNOWFREEZE-007`) |
| Soil-temperature `0 degC` isotherm | SCAN Mandan, Reynolds Creek | Deeper than the ice front by freezing-point depression; `frdp <= isotherm + TOL` | Timing/duration (`TOL-SNOWFREEZE-008`); magnitude upper-bound only |
| Penetrometer / mechanical resistance | (Marcell; not in pilot) | Method-dependent | Secondary; non-authoritative for magnitude |

Conflating these definitions manufactures a false model error - a `0 degC`-isotherm
target makes a correct ice front read as too shallow. The correspondence is a
contract decision, fixed here before any divergence is adjudicated.

### Snow-depth correspondence (binding under `INV-SNOWFREEZE-048`)

The snow-control operand is physical snowpack depth at the modeled hillslope,
published as WAT `Snow-Depth` from `snow.runtime_depth_m`. WAT `Snow-Water`
is snow-water equivalent and is a water-storage/output operand; it is invalid
as a snow-depth proxy even when its numerical residual is smaller.

Before a paired snow-control failure routes to snow physics, the harness must
prove source field semantics, units, daily timing/stage, signed residual
direction, and depth-vs-SWE anti-alias evidence. Rows without paired observed
snow depth stay inconclusive for frost attribution. Rows with paired observed
snow depth and like-for-like proof but failed `TOL-SNOWFREEZE-009` block frost
attribution and route to snow-depth fidelity, carry-state, input/forcing, or
settlement/density adjudication.

### SNOTEL density correspondence (binding under `INV-SNOWFREEZE-049`)

SNOTEL paired `WTEQ`/`SNWD` rows provide the external-authority density surface
for the snow-depth fork that the frost-tube pilot could not close. `WTEQ` is
SWE and `SNWD` is physical snowpack depth; the normalized density operand is
`SWE / depth`, reported as `kg m^-3`. Density is only valid on same-date rows
with positive SWE and positive physical depth. Trace or missing depth rows are
not repaired or carried forward because that would manufacture a density
target.

The SSD arm used for WEPP empirical models must be derived from observed
peak-SWE-period density before modeled residuals are inspected. The SSD arm is
site characterization, not calibration. If an observed-density SSD arm reduces
depth error, that supports a `LOW-DENSITY` route; if SWE itself is high, that
supports `OVER-ACCUMULATION`; if depth error persists after an observed-density
SSD arm, the route is `STRUCTURAL`. These are fork-routing labels only and do
not by themselves authorize production physics edits.

### Required validation obligations

1. Compare modeled `frdp` to observed depth only like-for-like by method.
2. Gate every magnitude/timing verdict on snow-insulation control
   (`TOL-SNOWFREEZE-009`): when modeled snow depth diverges from paired observed
   snow beyond the band, the frost comparison is inconclusive, not a frost
   verdict. Snow depth and density (not SWE) govern insulation.
3. Prove snow-depth correspondence per `INV-SNOWFREEZE-048`; do not let
   `Snow-Water`, `snow.runtime_swe`, snowfall depth, or melt aliases stand in
   for physical snowpack depth.
4. Honor censoring: exclude left-censored onset (observers begin at 1-2 in) from
   onset-timing error; exclude right-censored sensor-depth caps from magnitude
   error.
5. Aggregate over both seasonal-maximum depth and the observation-date depth
   series; do not reduce to a single annual scalar.
6. Apply the ADR-0017 verdict taxonomy (`HARNESS-SURFACE-MISMATCH`,
   `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, `UNRESOLVED`); `OPENWEPP-DEFECTIVE`
   additionally requires independent correctness authority (conservation/energy
   balance or documented WEPP reference equations), not observation disagreement
   alone.
7. For SNOTEL density adjudication, prove `WTEQ`/`SNWD` unit conversion,
   positive-depth density filtering, same-date pairing, observed-density SSD
   derivation, and no residual-fit SSD selection before assigning
   `OVER-ACCUMULATION`, `LOW-DENSITY`, or `STRUCTURAL`.

### Open items for ratification

- Calibrate `TOL-SNOWFREEZE-007/008/009` from the first validation pass.
- Characterize the frost-tube-vs-ice-front bias rather than assuming it is zero
  (free water in the tube freezes near `0 degC`; the residual offset to the matric
  ice front is expected small but unmeasured).
- Decide whether to acquire the request-only Dun-2010 Pullman/Morris series to
  anchor the WEPP-lineage agricultural tier directly (Site 4 GGD498 is the public
  ~11 km bridge in the interim).
- Resolve the per-site climate caveats in the manifests (Site 4 observation
  period vs DAYMET availability; Site 5 CLIGEN-station-to-hillslope lapse).
- Ratify the snow-depth daily timing/stage convention for snow-course rows after
  signed residual audits quantify whether timing could explain the paired
  failures.
- Ratify `TOL-SNOWFREEZE-010` after the first SNOTEL three-way pass, including
  whether a separate maritime/intermountain/continental density band is needed.

## GAP-SNOWFREEZE-002 Snow/Frost Fidelity Evaluation Rubric Addendum

Status: draft (2026-06-25). This addendum defines how modeled snow (and frost)
fidelity to observations is *scored*, under `INV-SNOWFREEZE-050`. It changes no
physics. All thresholds are provisional noise-floor estimates pending
hydrology-reviewer ratification and calibration from the SNOTEL corpus.

### 1. Why a rubric, not a tolerance

A single residual band (e.g. `TOL-SNOWFREEZE-009`) is blunt: one number hides
*which process* failed and conflates timing, magnitude, and density errors. A
snow hydrologist evaluates a model by *signatures* - process-diagnostic metrics
across timescales - and reads the resulting *profile*, not a scalar. This rubric
adopts that practice (hydrologic-signature / SnowMIP / KGE-decomposition
tradition). Robustness comes from aggregating the right way and weighting
forcing-robust signals; sensitivity comes from process decomposition.

### 2. Irreducible uncertainty budget

A modeled point snow depth is the product of: gridded DAYMET precip (large error
in complex terrain, snow undercatch); closest-CLIGEN storm patterns at a valley
station often far below the alpine hillslope; PRISM lapse/orographic
spatialization; rain/snow phase partition at the `0 degC` knife-edge; the
empirical WEPP snow parameters; point-pillow-vs-hillslope representativeness
(elevation, aspect, canopy, wind redistribution); and sensor noise/QC. These do
not cancel; the floor on absolute daily depth is tens of percent. **Frost depth
is strictly worse** - downstream of the entire snow chain plus soil thermal
properties, the frost model, and the measurement-definition gap. Acceptance
thresholds are therefore calibrated to this floor, and absolute-magnitude cells
are reported-but-discounted.

### 3. The rubric matrix

Each cell is tagged `R` (forcing-robust: intensive or relative; carries a model
verdict) or `L` (forcing-limited: dominated by forcing/representativeness;
reported, never a standalone defect).

| Timescale | Signature (process) | Tier | Metric |
|---|---|---|---|
| Long-term | mean peak SWE bias | L | median signed bias, IQR |
| Long-term | mean peak depth bias | L | median signed bias, IQR |
| Long-term | mean cold-season bulk density | R | median bias vs `TOL-SNOWFREEZE-010` |
| Long-term | snow-cover duration; inter-annual variability ratio | R | ratio; KGE-gamma |
| Seasonal | accumulation onset date, build-up rate | R | date offset; KGE |
| Seasonal | peak SWE/depth magnitude + date of peak | L mag / R date | bias; date offset |
| Seasonal | densification trajectory rho(t) | R | KGE (r/beta/gamma) |
| Seasonal | depth-SWE seasonal slope | R | slope ratio |
| Seasonal | ablation: melt-out date, ablation rate | R | date offset; KGE |
| Event | new-snow density (per storm) | R | median bias vs `TOL-SNOWFREEZE-010` |
| Event | rain-on-snow response (dSWE/ddepth/drho) | R | event-paired delta |
| Event | mid-winter melt; post-storm settling rate | mixed | event timing/magnitude |
| Cross-cutting | regime ordering across the five SNOTEL climates | R | rank correlation |
| Cross-cutting | outliers/tails: deepest, densest, extreme years | R | quantile bias |
| Cross-cutting | bias-sign consistency | R | sign fraction |
| Cross-cutting | conservation (mass/energy) | R, hard | closure residual |

Frost parallel (looser, behavior-only, downstream-noisier): onset timing, max
frost depth (`L`), frozen duration, thaw timing, freeze-thaw cycles, infiltration
consequence, plus the measurement-correspondence cell
(frost-tube / `0 degC`-isotherm / `frdp`, `INV-SNOWFREEZE-047`).

### 4. Scoring and output

- Time-series cells: Kling-Gupta-Efficiency decomposed into correlation `r`,
  bias-ratio `beta`, and variability-ratio `gamma`, so the failed mode is named.
- Magnitude cells: median signed bias and IQR (robust to tails; signed).
- Timing cells: date offset in days.
- Each cell is scored on an ordinal `0-3` (fail/marginal/pass/strong) at the
  provisional pass-levels in `TOL-SNOWFREEZE-011` and the per-quantity bands
  `TOL-SNOWFREEZE-007/008/009/010`, calibrated to the budget in section 2.
- Output is a per-model, per-site, per-cell **profile** (a heatmap), not a
  scalar. Censoring is honored (`INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`).

### 5. Verdict rule

The ADR-0017 taxonomy (`PASS` / `HARNESS-SURFACE-MISMATCH` / `OPENWEPP-DEFECTIVE`
/ `UNRESOLVED`) is applied **per cell**. An overall snow/frost-model verdict
rides on the `R` cells; an `L` cell may not by itself yield `OPENWEPP-DEFECTIVE`
(it is uncertainty-consistent, not a defect). `OPENWEPP-DEFECTIVE` for an `R`
cell still requires independent correctness authority (conservation/physics), not
observation disagreement alone.

### 6. Three-way application

openWEPP, pinned legacy WEPP, and PySnobal are scored on the identical rubric,
yielding three comparable **profiles**. The comparison is read as a profile
overlay (where each model wins or loses, per process per climate), not as three
scalar residuals. Legacy and PySnobal remain diagnostic flags under ADR-0017,
never targets.

### 7. Ratification

All thresholds are provisional noise-floor estimates, refined from the
`tests/fixtures/snotel_observed/` corpus and pending hydrology-reviewer
ratification. They are evaluation bands, not calibration objectives; the
`INV-SNOWFREEZE-049` anti-tuning rule applies.

## Known Gaps

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SNOWFREEZE-001 | Per-invariant comparator vectors for hourly winter outputs (`hrmelt`, frost depth/thaw depth, freeze-thaw cycles) are not yet curated. | Limits immediate automated regression depth on hourly-heavy winter internals. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-SNOWFREEZE-002 | **Reopened 2026-06-24 (operator-directed).** FDHP01 Increment Dk had closed this at the single-OFE ADR-0017 *conservation/activation* boundary: the fine-layer freeze/thaw state machine, capacity/overflow ownership, in-hour resistance feedback, seasonal lower-front heat, and `hr_tmp`/`tmpadj` surface-temperature synthesis run `43/43` clean with additive storage closure to numerical noise, and timing/shape improved to depth correlation ~`0.76` and frozen-duration residual `+61` days. But absolute frost-depth *magnitude* was never closed to a physical envelope (the legacy 240-503 mm envelope was held as a comparator flag, not a millimetre target; upper-envelope outliers were a characterized handoff). The R7G/R7H array-native winter-column migration made frost a typed sub-solver but established that bit-parity to this conservation-closed-but-magnitude-unvalidated solver is not an acceptable frost acceptance basis. Frost-depth fidelity is reopened as the governing question, to be established against **historic frost-depth observations** through site hillslope models under external-authority discipline (ADR-0017), not by matching legacy or compatibility output. The validation method is bound by `INV-SNOWFREEZE-047` and the GAP-SNOWFREEZE-002 Frost-Depth Observation Validation Addendum; pilot substrate `tests/fixtures/snowfreeze_observed/`. | Blocks frost-depth fidelity sign-off and array-native *default* activation for frost-influenced outputs; does not block opt-in direct mode. Successor: a frost-depth heat-flow fidelity Defect-Closure ExecPlan with an observation-based validation fixture. | open | `[DIRECT][Static] + operator decision` |
| GAP-SNOWFREEZE-003 | Snow drifting equations are documented in Chapter 3 but explicitly inactive in the August 1995 lineage; active-path authority for openWEPP is unresolved. | Drift-related claims cannot be promoted as active behavior yet. | non-promotable | `[DIRECT][Static]` |
| GAP-SNOWFREEZE-004 | Cross-contract boundary ownership with `SC-SOIL-001` and `SC-RUNOFFPART-001` is explicit, but executable cross-contract comparator vectors for frost-hourly internals are still incomplete. | Promotable contract authority exists; evidence depth for coupled frost vectors remains limited pending SIMIMPL32 and SIMIMPL35. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SNOWFREEZE-005 | `Dsavail` alias is fixed (`snow.hourly.depth_available_m`) and SIMIMPL29 emits the hourly family, but comparator-tier depth/density/melt vector breadth remains limited for broad climate regimes. | Residual risk is evidence-depth, not missing alias/state publication. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-06-26` | `78` | `Codex` | SNOWDENSITY-05C albedo-state amendment: added `INV-SNOWFREEZE-054`, `OBL-SNOWFREEZE-P-029`, typed `brock2000_temperature_age_v1` albedo-state variables, Brock et al. (2000) formula constants, fresh-snow reset, `[0, 0.85]` bounds, missing-state fail-closed behavior for active opt-in melt, and no-effect rule for `legacy_coe`. No routed-melt wiring, default activation, source change, parser surface, output schema, or coefficient fitting is ratified by this amendment. |
| `2026-06-26` | `77` | `Codex` | SNOWDENSITY-05B source-binding amendment: added `INV-SNOWFREEZE-053`, `OBL-SNOWFREEZE-P-028`, `winter_shortwave_daily_radly`, and the Shortwave Source Binding Addendum. The canonical openWEPP source is the existing climate `rad`/`radly` daily radiation seam in `Ly d^-1`, transformed through `SC-CLIMATE-001#INV-CLIMATE-013`; gridded-provider selection/spatialization remains outside engine ownership, ET and snowmelt share radiation authority, and snow-only/fitted/scaled/clipped/double-converted radiation is invalid. No production runtime code, parser surface, output schema, source selector, albedo constant, or default activation is ratified by this amendment. |
| `2026-06-26` | `76` | `Codex` | SNOWDENSITY-05A contract/sign amendment: added `INV-SNOWFREEZE-052`, `OBL-SNOWFREEZE-P-027`, opt-in `snow_melt_model = legacy_coe | coe_shortwave_albedo_v1`, shortwave/albedo operand placeholders, no-radiation-tuning guard bound to `SC-CLIMATE-001#INV-CLIMATE-013`, explicit signed `melt_bmelt_in` trace convention, and negative-benchmark disposition for `dense_slow_melt_v1`. No production formula, albedo constants, radiation source, parser surface, output schema, or default activation is ratified by this amendment. |
| `2026-06-25` | `75` | `Codex` | SNOWDENSITY-02 contract/ADR amendment: added `INV-SNOWFREEZE-051`, `OBL-SNOWFREEZE-P-026`, candidate `physics_bulk` state variables, candidate authority anchors, and the Snow-Density Physics-Bulk Candidate Envelope Addendum. `legacy_wepp` remains default; `physics_bulk` is opt-in candidate scope only, with no site-specific tuning and no production formula ratified by this amendment. |
| `2026-06-25` | `74` | `Claude Code` | SNOWFROST-FIDELITY rubric amendment: added `INV-SNOWFREEZE-050` + the GAP-SNOWFREEZE-002 Snow/Frost Fidelity Evaluation Rubric Addendum (signature-based, multi-timescale, forcing-robustness-tiered, KGE-decomposed, profile-not-scalar evaluation), provisional `TOL-SNOWFREEZE-011`, and an irreducible-uncertainty budget; supersedes `TOL-SNOWFREEZE-009` as a standalone snow-model acceptance band. Reconciled the header `contract_version` (header was lagging at 72 vs the existing v73 H row). All thresholds provisional pending hydrology-reviewer ratification. |
| `2026-06-25` | `73` | `Codex` | SNOWFROST-FIDELITY-H amendment: added `INV-SNOWFREEZE-049`, provisional `TOL-SNOWFREEZE-010`, and SNOTEL `WTEQ`/`SNWD` observed-density correspondence for over-accumulation vs low-density snow-depth adjudication with an anti-tuning SSD-arm rule. |
| `2026-06-25` | `72` | `Codex` | SNOWFROST-FIDELITY-E amendment: added `INV-SNOWFREEZE-048`, WAT `Snow-Depth` variable authority, and snow-depth correspondence/anti-alias obligations so `TOL-SNOWFREEZE-009` failures route through source semantics, timing/stage, signed residual, and depth-vs-SWE proof before any snow or frost physics work. |
| `2026-06-24` | `71` | `Claude Code` | Drafted the `GAP-SNOWFREEZE-002` frost-depth observation-validation method: added `INV-SNOWFREEZE-047` (measurement-to-`frdp` correspondence + censoring/snow-confound gates), `REF-SNOWFREEZE-FROST-OBS`, provisional `TOL-SNOWFREEZE-007/008/009`, and the GAP-SNOWFREEZE-002 Frost-Depth Observation Validation Addendum bound to `tests/fixtures/snowfreeze_observed/`. Tolerances provisional pending hydrology-reviewer ratification. |
| `2026-06-24` | `70` | `Claude Code` | Reopened `GAP-SNOWFREEZE-002` on operator direction: frost-depth fidelity decoupled from array-native bit-parity. Acceptance basis re-pinned to historic frost-depth observations via site hillslope models (ADR-0017 external authority), not legacy/compatibility output. Closes out the interrupted R7H frost bit-parity grind in favour of a heat-flow fidelity DC. |
| `2026-06-12` | `69` | `Codex` | FDHP01 Increment Dk certification: closed/re-stated `GAP-SNOWFREEZE-002`, recorded the residue pre-check disposition, and unblocked MOFE under ADR-0017. |
| `2026-06-12` | `68` | `Codex` | FDHP01 Increment Dj amendment: bound frost top heat flow to legacy `hr_tmp`/`tmpadj` adjusted surface-temperature synthesis, registered `frost.hourly.surface_temp_c_####`, and recorded Dj's executed-hold cohort outcome. |
| `2026-06-12` | `67` | `Codex` | FDHP01 Increment Dh amendment: refuted per-soil frozen-path conductivity as an implementation target and bound fixed legacy `kftill`/`kfutil` constants for the frozen tilled/untilled surface path. |
| `2026-06-12` | `66` | `Codex` | FDHP01 Increment Dg amendment: promoted legacy `resdep` residue resistance and below-freezing `dpfsfl` shallow-front minimum conduction distance into `INV-SNOWFREEZE-006`. |
| `2026-06-12` | `65` | `Codex` | FDHP01 Increment De amendment: promoted `frostn.for:430-458` lower-front `Qdry` authority, requiring `bdcons`/`slsw`/`ksoilf` per-fine-layer harmonic unfrozen conductivity and limiting `kufz = 0.2` to the legacy no-positive-term fallback. |
| `2026-06-12` | `64` | `Codex` | FDHP01 Increment Dc1 amendment: replaced the stable lower-front heat surrogate with legacy seasonal `tmpbl`/`Qdry` authority, required in-hour thaw resistance feedback, and authorized only bounded fine-theta lower-bound roundoff canonicalization while preserving the Db D2 closure gate. |
| `2026-06-12` | `63` | `Codex` | FDHP01 Increment Db amendment: bound freeze-active `frzng` front advance to in-hour surface-resistance/`Qsrf` recomputation after each fine-layer advance, using Da's p1 trace and legacy `frzng.for` loop provenance. |
| `2026-06-12` | `62` | `Codex` | FDHP01 Increment C2 amendment: completed thaw-arm state-machine authority for `mltbtm`/`mlttp`, including bottom/top thaw geometry, `nwfrzz` release, `watpdg`/`watbtm` capacity-routed overflow, `fgthwd` thaw-through, and non-amplifying repeated freeze/thaw conservation. |
| `2026-06-12` | `61` | `Codex` | FDHP01 Increment C1b amendment: set bounded WB18/WB13 deep-percolation publication dust to `1e-11 m` and WB18 scalar/layer storage rebalance to `2e-11 m` so valid roundoff cannot accumulate as storage drift. |
| `2026-06-11` | `60` | `Codex` | FDHP01 Increment C1b amendment: added capacity-bound fine-layer liquid/ice ownership, fail-closed over-capacity guards, and `watpdg`/`watbtm` overflow publication semantics with `watbtm` entering WB13 `Dp` plus bounded deep-percolation roundoff canonicalization at WB18/WB13. |
| `2026-06-11` | `59` | `Codex` | FDHP01 Increment B amendment: promoted fine-layer `frzng`/`frznw` freeze-arm state as active depth authority, added `frost.hourly.frzflg_####`, required `watdst`-style depth derivation from `fgfrst`/`slfsd`, allowed threshold-bounded exchange-debit limiting at the available-liquid handoff boundary, and retired scalar target-depth projection as production authority. |
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
