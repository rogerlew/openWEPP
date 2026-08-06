---
contract_id: SC-SNOWFREEZE-001
title: Snow and Freeze Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 129
producer_scope:
  - Winter precipitation phase partition surfaces (rain vs snow)
  - Snowpack depth/density/water-equivalent state surfaces
  - Melt and freeze-thaw transition surfaces
consumer_scope:
  - Daily water-balance accounting consumers
  - Infiltration/runoff partition consumers affected by frozen-soil state
  - Soil/erosion coupling consumers requiring freeze-thaw context
evidence_level: static
last_reviewed: 2026-08-06
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
| REF-SNOWFREEZE-FROST-RESIDUE-COVER-20260629 | `docs/work-packages/20260629-frost-residue-cover-implementation-001/` and `SC-RESIDUE-001#INV-RESIDUE-019` | Frost residue-cover implementation authority: residue owns the dynamic surface-residue mass and mass-to-depth conversion; frost consumes the current dynamic `residue_depth_m` as the residue surface-resistance term rather than a static initial seed. | `[DIRECT][Ran] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-LEGACY-FROZEN-PATH-KF | `/workdir/wepp-forest_260430_baseline/src/frostn.for:188-193`, `:530-534`, `/workdir/wepp-forest_260430_baseline/src/frzng.for:135`, `:295-304`, and `/workdir/wepp-forest_260430_baseline/src/frznw.for:106-108`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative frozen surface-path conductivities: `kftill = 1.75 W m^-1 degC^-1` and `kfutil = 2.1 W m^-1 degC^-1` are fixed constants for the frozen tilled/untilled path. They are not soil-property functions and are not multiplied by `ksoilf`; `ksoilf` belongs to the unfrozen lower-front `kufzfl` path. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-FRWATC-EGRESS | `/workdir/wepp-forest_260430_baseline/src/frostn.for:686` and `/workdir/wepp-forest_260430_baseline/src/frwatc.for:80-137`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline-authoritative day-end/thaw-complete egress calls `frwatc(0)` once; that routine derives coarse frozen depth, frozen water, available liquid `st`, and total liquid `soilw` from the fine-layer state, including residual water over the post-thaw unfrozen thickness. | `[DIRECT][Static]` |
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
| REF-SNOWFREEZE-HARDER-POMEROY-2013 | `references/copyrighted/source_pdfs/harder2013.pdf` | Hydrometeor-temperature precipitation-phase candidate authority: psychrometric vapor density, water-vapor diffusivity, air thermal conductivity, latent heat, iterative hydrometeor-temperature solution, and logistic rainfall-fraction coefficients from Harder and Pomeroy (2013). This is candidate meteorology authority only and does not supersede the production WEPP `RST` partition. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative snow depth/water and bounded densities are required for physical validity. | `[INFERENCE][Static]` |
| REF-SNOWFREEZE-FROST-OBS | `tests/fixtures/snowfreeze_observed/` (five WEPP hillslope fixtures + per-site manifests) and the historic frost-depth observation corpus they bind: USGS Sleepers River (`DOI 10.5066/P96753GI`, frost tube + paired snow depth), NRCS SCAN soil temperature (`stationTriplets=2020:ND:SCAN`, derived `0 degC` isotherm), NSIDC GGD498 Midwest frost tubes (`DOI 10.7265/1mcs-q536`), USDA-ARS Reynolds Creek soil temperature (CC-BY), and the WEPP-lineage Dun et al. 2010 Pullman/Morris frost validation (`doi:10.13031/2013.34896`, request-only). | External-authority frost-depth observations under ADR-0017 (legacy/compatibility frost output is a flag, not the acceptance target). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-FROST-RATIFICATION-20260629 | `docs/work-packages/20260629-frost-ratification-default-activation-001/`, plus the frost arc packages `20260629-frost-step1-current-snow-control-rerun-001`, `20260629-frost-step2-sleepers-attribution-001`, `20260629-frost-step3-residue-parameterization-001`, `20260629-frost-residue-cover-implementation-001`, `20260629-frost-thaw-residual-diagnostic-001`, `20260629-frost-snow-persistence-decomposition-001`, and `20260629-frost-h1b-state-machine-thaw-asymmetry-check-001`. | Ratification and default-activation evidence for the observed frost validation method, bounded residual disposition, H1b `frdp`/`thdp` metric correspondence, and direct-production runtime default activation. | `[DIRECT][Ran] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-FROST-DIRECT-CUTOVER-CORRECTION-20260629 | `docs/work-packages/20260629-frost-direct-cutover-correction-001/` | Correction evidence superseding the v114 compatibility-fallback carve-out: no-env `DefaultCandidate` must select direct production for current hillslope runs, including multi-OFE/Wave-2 and legacy sidecar-discovery inputs, unless execution fails closed with a named direct-runtime blocker. | `[DIRECT][Ran] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOWDENSITY01 | `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/` | Evidence reconciliation showing current openWEPP and pinned legacy share the same structural snow-density/depth lineage for the SNOTEL comparison, with maximum as-built openWEPP-vs-legacy density delta `4.351046738461008 kg m^-3`; this routes remediation away from bit-parity and toward a contract-scoped physics candidate. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOWDENSITY1015 | `docs/work-packages/20260627-snowdensity-10-3-15-default-activation-active-cap-001/` | Default-activation package for the active-cap snow-depth bundle after SNOWDENSITY-10.3.14 recorded a workspace-suite no-regression gate under explicit selectors plus composite snow-state conservation closure under active `522 kg m^-3` cap authority; downstream snow-affected output deltas were not separately diffed. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOWDENSITY1017 | `docs/work-packages/20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001/` | Shallow-pack compaction-guard package authority: tests whether reducing only depth-limited compaction aggressiveness below the Marks/SNOBAL active surface-layer depth (`0.25 m`) recovers density-arm-induced under-persistence without worsening over-persistence or changing SWE, cap, melt/liquid, fixtures, schemas, defaults, or frost attribution. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOWDENSITY1019 | `docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/` | Harder-Pomeroy default-activation package authority: promotes `harder_pomeroy_hourly` as the direct-production no-env phase default when composed with the activated melt+density bundle, based on the cross-SNOTEL forcing-robust rubric Policy-B gate and explicit `legacy_rst` rollback. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOWDENSITY1020 | `docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/` | Sublimation diagnosis and Stage-B unlock package authority: diagnoses Stage A degradation by site/signature/magnitude, scores partition+sublimation composition against the new Harder-Pomeroy default, and authorizes only opt-in `coe_open_sublimation_stage_b_v1` testing under the cross-SNOTEL forcing-robust rubric. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOWDENSITY1022 | `docs/work-packages/20260628-snowdensity-10-3-22-climate-class-density-specialization-001/` | Comprehensive climate-class density specialization candidate package authority: reserves opt-in `physics_bulk_climate_class_density_v1`, requires forcing-derived Sturm 1995 class assignment and Sturm 2010 class-density parameter authority, and closes `HOLD` unless the cross-SNOTEL forcing-robust rubric, bidirectional densification flip, persistence guardrail, and conservation gates can be run without fixture fitting. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-PARADIGM2-STAGE1 | `docs/work-packages/20260628-paradigm-2-stage-1-layered-snow-density-001/`, ADR-0029, and `docs/planning/paradigm2-multilayer-snow-specification.md` §6 Stage 1 | Paradigm 2 Stage 1 authority: reserves opt-in `physics_bulk_multilayer_density_v1`, adds persistent winter-column snow layer state, and limits the production delta to applying the existing Anderson/SNOBAL density-compaction constants per layer under local overburden while preserving aggregate public outputs, default, rollback, cap, and melt/liquid/phase/frost boundaries. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-PARADIGM2-STAGE2 | `docs/work-packages/20260628-paradigm-2-stage-2-snow-frost-insulation-profile-001/`, ADR-0029, `docs/planning/paradigm2-multilayer-snow-specification.md` §4 req. 5 and §6 Stage 2, Sturm et al. 1997 (`Journal of Glaciology` 43(143), 26-41, DOI `10.3189/S0022143000002781`), and legacy `/workdir/wepp-forest_260430_baseline/src/tmpadj.for:274-287` plus `/workdir/wepp-forest_260430_baseline/src/frostn.for:483-498` | Paradigm 2 Stage 2 authority: reserves an opt-in snow-to-frost insulation-profile coupling that consumes the Stage 1 layer stack only after a real gradient entry gate, computes layer resistance with the Sturm et al. density-to-conductivity relation already used by the WEPP frost surface heat path, and projects an insulation-equivalent bulk snow density into the existing prior-day frost handoff while preserving the bulk rollback and deferring per-layer thermal solve/melt to Stage 3. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-PARADIGM2-STAGE3 | `docs/work-packages/20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001/`, ADR-0029, `docs/planning/paradigm2-multilayer-snow-specification.md` §4 reqs. 3/4 and §6 Stage 3, Stage 0 `openwepp-meteorology` surface-energy primitives, REF-SNOWFREEZE-LIBSNOBAL-CC0, Marks 1999, Crocus R-40, and the Stage 1/2 package outcomes | Paradigm 2 Stage 3 authority: reserves an opt-in diagnostic per-layer thermal/liquid-routing candidate that uses the Stage 1 layer stack and Stage 0 energy primitives to compute layer temperature, cold content, refreeze, liquid routing, and typed meltwater-flux temperature while preserving the existing CoE SWE/melt mass path, no-env defaults, rollback, public schemas, and deferring full stream-temperature routing. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-PARADIGM2-STAGE3-DECOUPLE | `docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001/`, `docs/planning/snow-frost-fidelity-strategy.md` §10.3 step 10, ADR-0029, ADR-0028, ADR-0026, and the Stage 3 package outcome | Paradigm 2 Stage 3-Decouple authority: supersedes the Stage 3 requirement to compose with `physics_bulk_multilayer_density_v1` for the water-temperature arm. The opt-in thermal/liquid solver may run on a bulk-equivalent layer stack whose per-layer densities all equal the selected bulk density outcome, so public snow-density/frost behavior remains the current bulk default while typed meltwater-flux temperature remains available for the stream-temperature program. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-PARADIGM2-PROMOTION | `docs/work-packages/20260629-paradigm-2-multilayer-promotion-001/`, `docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001/`, `docs/planning/snow-frost-fidelity-strategy.md` §10.3 step 10, ADR-0029, ADR-0028, and the stream-water-temperature backlog | Paradigm 2 multilayer promotion authority: ratifies the Stage 3-Decouple arm as a production-supported opt-in capability rather than a package-local diagnostic. The selector remains default-off and supported-internal; the supported hillslope output is nullable WAT parquet `MeltwaterTemperature` in `degC`; HBP/watershed serialization and full in-stream routing remain deferred. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-EB03 | `SC-SNOWENERGY-001` v2, `docs/planning/snow-surface-energy-balance-roadmap.md`, and `docs/work-packages/20260730-snow-surface-eb-03-shared-thermal-energy-composition-001/` | Shared Stage 3 snow-surface temperature/cold-content provider, orthogonal longwave/sublimation selectors, and exact-one vapor/latent composition. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-EB03A | `SC-SNOWENERGY-001` v3, Marks et al. (1999), Lute et al. (2022) section 2.2.7, CC0 libsnobal commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`, and `docs/work-packages/20260730-snow-surface-eb-03a-active-layer-thermal-coupling-001/` | Active thermal control-volume construction, active/lower conductive coupling, conservative projection, and mass-dependent stability substeps that close `GAP-SNOWENERGY-007`. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-EB04W | `references/50201000/chap3.pdf` §§3.2 and 3.6; `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95`, `melt.for:275-301`, and `winter.for:420-464` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; and `docs/work-packages/20260801-snow-surface-eb-04w-accumulation-under-persistence-001/` | Behavior-neutral hourly phase/accumulation and four-term CoE melt-depth observability, including the inactive modeled-drift boundary and diagnostic-only mountain chronology attribution. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-STAGE3-TRACE-CLOSURE | `docs/work-packages/20260803-snow-prepeak-liquid-evacuation-physics-audit-001/` and `docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/` | Read-only pre-peak liquid audit and behavior-neutral follow-up authority for publishing exact Stage-3 liquid operands plus signed-hour forcing/state lineage through the real internal JSONL consumer. | `[DIRECT][Static] + [INFERENCE][Ran]` |
| REF-SNOWFREEZE-MASS-TRANSITION-LEDGER-PERSISTENCE | `docs/work-packages/20260803-snow-mass-transition-ledger-persistence-001/` | Behavior-neutral architecture authority for retaining the solid-to-liquid and Stage-3 liquid-disposition boundaries as two exact linked compact ledgers over one production snow calculation while making the large hourly research payload optional at collection. | `[DIRECT][Static] + [INFERENCE][Ran]` |
| REF-SNOWFREEZE-STAGE3-EVALUATION-SHADOW-AUTHORITY | `docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/`, the executed Stage 3 complete-carrier shadow package, and the revised snow surface-energy roadmap §6.2 | Contract-scoping authority that preserves one production snow-mass owner while admitting two explicitly tagged, default-off, consumer-forbidden evaluation operators with bounded custody and claims. | `[DIRECT][Static] + [INFERENCE][Ran]` |
| REF-SNOWFREEZE-STAGE3-SHADOW-OBSERVABILITY | `docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/`, `INV-SNOWFREEZE-094`, and the schema-v4 direct snow trace lineage | Runtime-realization authority for the two admitted evaluation operators: typed default-off request/tag custody, exact disabled schema-v4 preservation, enabled internal schema-v5 per-term/fingerprint/support evidence, typed turbulent-error custody, and solver extraction without production-physics change. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-STAGE3-OPERATOR-RECONCILIATION | `docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/`, its admitted result-blind protocol, `INV-SNOWFREEZE-094/095`, and `openwepp-meteorology` Monin-Obukhov surface-exchange lineage | Behavior-neutral authority for additive schema-v6 substep observability and an independent operator-reconciliation consumer that separates initial active-control-volume projection, within-day clone evolution, support censoring, and external-versus-internal energy terms without changing either admitted operator or production behavior. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-STURM2010-DENSITY | `references/copyrighted/sturm2010_swe_climate_classes.pdf` Eq. 6 and Table 4 | Sturm 2010 density trajectory authority for class-specific `rho_max`, `rho_0`, `k1`, and `k2` parameters for alpine, maritime, prairie, tundra, and taiga snow. The same paper states ephemeral snow measurements were excluded, so no ephemeral density parameter row is available from this local authority. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-STURM1995-CLASSIFICATION | `references/copyrighted/sturm1995.pdf` pp. 1273-1276, Figs. 8-9 and Table 7 | Required authority for forcing-derived class assignment from the run's own air-temperature, precipitation, and wind climate. The original tree uses CDM with `Tc=10 degC`, high/low temperature threshold `125 degC-month`, ephemeral threshold `30 degC-month`, precipitation threshold `2 mm d^-1`, and wind low/high evidence bounded by `0.5-2.0 m s^-1`; the original map used vegetation as wind proxy, so direct-runtime wind classification must fail closed for the unresolved `0.5 < wind < 2.0 m s^-1` interval rather than inventing a fitted cutoff. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-STURM2021-CLASSIFICATION-CROSSCHECK | `references/copyrighted/hydr-JHM-D-21-0070.1.pdf` Fig. 2 and §2a; NSIDC-0768 user guide §Documentation | Cross-check for the Sturm classification update, not replacement authority for 1995-named classes. Sturm and Liston 2021 preserve the tree structure but update the ephemeral CDM threshold from `30` to `61 degC-month`, update the precipitation threshold from `2` to `4 mm d^-1`, and rename Taiga to Boreal Forest and Alpine to Montane Forest. Runtime pairing with Sturm 2010 density parameters must map names back to Sturm 1995 labels and must not silently inherit 2021 thresholds. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-NSIDC0768 | NSIDC-0768 Northern Hemisphere Snow Classes, user guide and dataset metadata | Independent cross-check for the Sturm snow-class system and class labels. It may validate broad class consistency but must not be used as geographic lookup, site-identity lookup, or calibration authority for openWEPP runtime class assignment. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-LIBSNOBAL-CC0 | Local PySnobal/libsnobal clone `/home/workdir/pysnobal` at commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`; `setup.py` declares `license="CC0 1.0"` and `deny.toml` allow-lists `CC0-1.0` while GPL/AGPL/LGPL are excluded by omission. | libsnobal C may be used as equation-reference / portable implementation reference for SNOBAL-lineage Stage B structure, subordinate to Marks 1999 and observed-data gates; SNOBAL/PySnobal remains a flag profile, not a target. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-ANDERSON1976-CANDIDATE | Anderson snow accumulation/ablation physics lineage as used by SNOBAL/PySnobal-style bulk snowpack densification references; package evidence begins in `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/` and SNOWDENSITY-06 binds the density-only candidate to Anderson-1976 §III compaction/metamorphism constants and the SNOBAL PTM/POC implementation lineage. | Candidate snow-density physics authority only; not a ratified production formula. | `[INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOBAL-CANDIDATE | Local PySnobal/SNOBAL diagnostic lane and three-way profile evidence from SNOWFROST-FIDELITY-G/H packages; source-level `_time_compact.c` / `_h2o_compact.c` inspection; and PySnobal 0.2.3 source archive `pysnobal-0.2.3.tar.gz`, SHA-256 `78f97faf0452816038494b9fde332a2c2a14d92ec2e5960378abd7606d82fda2`. In `pysnobal/point/snobal.py`, `mass_bal` sequences precipitation, melt, evaporation/condensation, H2O compaction, then runoff; `h2o_compact` supplies `snow_state.melt + input1.m_rain` to the wet-compaction ratio. | Reference-implementation authority for Anderson/SNOBAL compaction chronology and operand meaning plus diagnostic profile evidence. PySnobal remains a flag profile, not an output target or runtime dependency. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-WET-COMPACTION-OPERAND-CLOSURE | `docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/`, PySnobal 0.2.3 source intake, Anderson 1976 `references/copyrighted/noaa_6392_DS1.md`, and pinned WEPP baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`: `src/stmtim.for:32-37,43-95`, `src/winter.for:296-300,366-374,420-464`, `src/melt.for:238-262,272-301`, and `src/snowd.for:180-283,303-316`. | Defect-closure authority for replacing the duplicated wet-compaction liquid alias with generated positive melt plus snow-contact rain exactly once. The pinned WEPP source owns phase, positive/signed melt, rain-contact, density-retention, and later routing cut points but contains no Anderson/SNOBAL H2O-compaction operator; it supplies ordering/sign and anti-alias authority, not the final wet-compaction formula by itself. Rain heat that generates melt and rain mass retained/routed remain distinct water operands. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-21N-MELT-OWNER | `docs/work-packages/20260804-snow-coe-stage3-melt-owner-authority-reconciliation-001/`, frozen 21M lineage/chronology/physical-authority evidence, `SC-SNOWENERGY-001` v7, and pinned libsnobal commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6` (`_e_bal.c`, `_snowmelt.c`, `_advec.c`, `_mass_bal.c`, `_runoff.c`) | Result-blind melt-owner adjudication, Stage 3 cold-content-first phase-change authority, same-substep liquid chronology, exact-one mass ledgers, CoE target retirement, and current-runtime implementation hold. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-MARKS-SHALLOW-LAYER | `references/copyrighted/marks1999.pdf` plus strategy summary in `docs/planning/snow-frost-fidelity-strategy.md` section 10.2 item 7. | Marks/SNOBAL structural precedent for a fixed active surface-layer depth (`max_z_s_0`, approximately `0.25 m`) and special shallow-snow layer collapse behavior. This authorizes a non-fitted shallow-pack guard threshold for density compaction diagnostics, not a port of libsnobal C or a two-layer surface-energy implementation. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-MARKS1998-LIQUID-CAPACITY | Marks et al. 1998 R-55 snow energy/mass-balance report, local authority under `references/copyrighted/` where available; package evidence in `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/`. | In-repo liquid-water holding-capacity authority for the provisional `wc,max` / `max_liquid_water_volume_fraction = 0.01` volume-ratio default used by the opt-in capacity drainage candidate. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-ANDERSON1976-LIQUID | Anderson 1976 NWS-19 accumulation/ablation model (`references/copyrighted/noaa_6392_DS1.md`). | Conceptual retained/free/excess water partition authority for pack liquid transmission: retained liquid is bounded by a pack holding capacity and excess liquid drains rather than being represented as unbounded density-only compaction. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOW17-PLWHC | Anderson 2006 SNOW-17 operational description, local authority under `references/copyrighted/` where available. | Operational precedent for a liquid-water holding-capacity parameter that is separate from snow density and melt-energy terms. | `[INFERENCE][Static]` |
| REF-SNOWFREEZE-SNOBAL-LIQUID-CAPACITY | Local SNOBAL-lineage code inspection, including in-repo candidate constant `max_liquid_water_volume_fraction = 0.01` in `crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs` and package evidence for SNOBAL `_runoff.c` concepts. | Reference-implementation precedent for `h2o_max`/excess-liquid runoff semantics and the existing openWEPP diagnostic constant used as non-fitted capacity authority. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-MARKS1998-TURBULENT | `references/copyrighted/source_pdfs/marks1998.pdf` | Marks/SNOBAL energy-balance authority: snow-cover energy balance is `DQ = Rn + H + LvE + G + M`; turbulent transfer terms solve for sensible heat `H` and snow-surface evaporation/condensation mass flux `E`; model mass-balance output treats evaporation as a snow-cover mass flux. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SNOWFREEZE-MARKS1999-SUBLIMATION | `references/copyrighted/marks1999.pdf` | Marks/ISNOBAL distributed authority: input forcing includes vapor pressure and wind speed; output includes average `LvE` and total snow evaporation `Es`; exposed windy/dry sites show rapid snow-cover loss attributed to sublimation / turbulent mass exchange, while humidity/wind forcing uncertainty is a core limitation. | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| `hydrometeor_temperature` | `degC` | Candidate Harder-Pomeroy hydrometeor temperature (`Ti`) computed from air temperature and humidity before precipitation-phase fractioning. | candidate `openwepp-meteorology` crate | candidate phase diagnostics and future adjudication |
| `relative_humidity` | `fraction` | Candidate unit-interval relative humidity input for psychrometric precipitation-phase methods. | candidate meteorology caller | candidate hydrometeor-temperature solver |
| `dew_point_temperature` | `degC` | Candidate dew-point input/diagnostic used to derive relative humidity and actual vapor pressure where caller supplies dew point rather than RH. | candidate meteorology caller | candidate psychrometric primitives |
| `air_vapor_density` | `kg m^-3` | Candidate actual water-vapor density of free air. | candidate `openwepp-meteorology` crate | hydrometeor-temperature solver diagnostics |
| `hydrometeor_saturation_vapor_density` | `kg m^-3` | Candidate saturated water-vapor density at the hydrometeor surface temperature. | candidate `openwepp-meteorology` crate | hydrometeor-temperature solver diagnostics |
| `harder_pomeroy_rain_fraction` | `fraction` | Candidate rainfall fraction from the Harder-Pomeroy logistic function at `hydrometeor_temperature`. | candidate `openwepp-meteorology` crate | candidate phase diagnostics |
| `harder_pomeroy_snow_fraction` | `fraction` | Candidate snowfall fraction equal to `1 - harder_pomeroy_rain_fraction`; fractions must close to one within roundoff. | candidate `openwepp-meteorology` crate | candidate phase diagnostics |
| `snow_phase_partition_model` | `enum` | Hourly precipitation-phase partition selector. Direct-production no-env and empty selector values default to `harder_pomeroy_hourly` under `INV-SNOWFREEZE-075`; explicit `legacy_rst` remains the rollback/test selector. Parser/runfile/user CLI selectors are not authorized by this contract revision. | typed runtime configuration / package-bound selector | hourly `hrrain`/`hrsnow` dispatch and rollback evidence |
| `harder_pomeroy_hourly_normalized_relative_humidity` | `fraction` | Unit-interval RH consumed by the opt-in hourly Harder-Pomeroy partition after finite vapor-pressure derivation from hourly air temperature and daily dew point or supplied RH. Supersaturated dewpoint-derived ratios may be normalized only to exactly `1.0` with package evidence; negative, zero-saturation, or non-finite ratios fail closed. | opt-in hourly partition caller | hydrometeor-temperature solver |
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
| `cancov_daily_series` | `fraction by simulation day` | Direct-production per-day canopy-cover trajectory used by snow liquid partition and diagnostic CoE melt replay; scalar canopy values are summaries only once this series is available. | direct production growth-state day input | melt attenuation term and snowbench/CoE replay diagnostics |
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
| `MeltwaterTemperature` | `degC` | Nullable hillslope WAT temperature of liquid exported from the layered Stage 3 snow column. | layered snow thermal/liquid runtime and WAT publication adapter | stream-temperature handoff and snow thermal diagnostics |
| `frdp` | `mm` | Hillslope WAT active frost-front depth publication converted from the runtime metre-valued state. | frost runtime state publication | frost-depth observation correspondence |
| `frost_hourly_surface_temp` | `degC` | Hourly adjusted surface temperature consumed by the frost heat-flow path. | hourly frost forcing adapter | frost top-boundary heat-flow calculation |
| `snow_hourly_stmtim_phase_model` | `dimensionless` | Unit-interval diagnostic flag identifying the hourly precipitation-phase selector. | hourly snow partition diagnostics | trace and comparator consumers |
| `snow_hourly_stmtim_rain_fraction` | `dimensionless` | Unit-interval hourly rain fraction produced by the precipitation-phase selector. | hourly snow partition diagnostics | trace and comparator consumers |
| `snow_hourly_stmtim_snow_fraction` | `dimensionless` | Unit-interval hourly snow fraction complementary to the rain fraction. | hourly snow partition diagnostics | trace and comparator consumers |
| `snow_hourly_stmtim_relative_humidity` | `dimensionless` | Unit-interval relative humidity used by the hourly precipitation-phase selector. | hourly meteorology adapter | trace and comparator consumers |
| `snow_hourly_stmtim_hydrometeor_temperature` | `degC` | Hourly hydrometeor temperature used by the Harder-Pomeroy phase selector. | hourly meteorology adapter | trace and comparator consumers |
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
| `snow_model` | `enum` | Snow model selector. Current accepted values are `legacy_wepp` and opt-in family `physics_bulk`; `legacy_wepp` remains the default unless a future ratified activation package changes it. SNOWDENSITY-07 ratifies the first typed runtime opt-in member as `physics_bulk_density_compaction_v1`. | runtime configuration / winter column | snow solver dispatch and diagnostics |
| `snow_density_model` | `enum` | Typed snow-density selector for the winter-column runtime. After SNOWDENSITY-10.3.15, absent direct-production selector state defaults to `physics_bulk_density_compaction_v1`; `legacy_wepp` remains an explicit rollback/test selector and compatibility comparator surface. SNOWDENSITY-10.3.17 authorizes `physics_bulk_shallow_guard_v1` only as an explicit opt-in diagnostic candidate. SNOWDENSITY-10.3.22 reserves `physics_bulk_climate_class_density_v1` only as an explicit opt-in candidate that must fail closed without authoritative forcing-derived class assignment and class density parameters. Paradigm 2 Stage 1 reserves `physics_bulk_multilayer_density_v1` only as an explicit opt-in candidate with persistent winter-column layer state and local-overburden densification. The rejected `physics_bulk_spring_densification_v1` candidate remains historical evidence only and is not an accepted active-default selector. | typed runtime configuration / winter column | snow density/depth mutation and diagnostics |
| `snow_layers` | vector of layer states | Persistent top-to-bottom snow layer stack for Paradigm 2 Stage 1 and Stage 3-Decouple. Each layer carries snow-water-equivalent mass (`m`), physical thickness (`m`), density (`kg m^-3`), and a settle-day counter. Layer aggregates must reconstruct `snow_runtime_swe`, `snow_runtime_depth`, and `snow_runtime_density` within snow storage tolerance before they may drive downstream state. Under Stage 3-Decouple with the bulk density selector, all layer densities must equal the selected aggregate bulk density and may not apply Stage 1 local-overburden densification. | `physics_bulk_multilayer_density_v1` candidate or `layered_thermal_liquid_v1` decoupled water-temperature arm / winter-column sub-solver | density projection, runtime carry, frost insulation-profile coupling, and meltwater-temperature diagnostics |
| `snow_layer_local_overburden` | `kg m^-2` | Per-layer overlying snow mass used by the Paradigm 2 Stage 1 compaction candidate. For a layer, local overburden is the sum of all overlying layer masses, not total pack mass. This is the only densification delta from `physics_bulk_density_compaction_v1`. | `physics_bulk_multilayer_density_v1` candidate | per-layer overburden compaction |
| `snow_frost_insulation_model` | `enum` | Package-bound snow-to-frost insulation handoff selector for Paradigm 2 Stage 2. Accepted values are empty/default `bulk_depth_density` and explicit opt-in `layered_resistance_v1`. It is not a parser/runfile/user CLI surface and must fail closed on unsupported non-empty values. | direct-production winter/frost handoff | opt-in rollback and trace proof |
| `snow_layer_insulation_resistance` | `m^2 K W^-1` | Layer-stack snow thermal resistance for Stage 2, computed as `sum(layer_thickness_m / k_sturm1997(layer_density_kg_m3, ksnowf))` over the prior-day top-to-bottom Stage 1 layer stack. It is diagnostic/internal and must not alter public WAT schemas. | `layered_resistance_v1` candidate / frost handoff | frost surface heat-path equivalent |
| `snow_frost_effective_density` | `kg m^-3` | Insulation-equivalent bulk snow density passed through the existing frost input field when `layered_resistance_v1` is selected. It is the density whose existing `k_sturm1997(rho, ksnowf)` makes `snow_runtime_depth / k` equal the layer-stack resistance; it is not a calibrated snowpack density and cannot replace aggregate snow output density. | `layered_resistance_v1` candidate / frost handoff | existing frost solver rollback-compatible projection |
| `snow_stage3_liquid_routing_model` | `enum` | Production-supported internal Paradigm 2 Stage 3 selector. Accepted values are empty/default `disabled` and explicit opt-in `layered_thermal_liquid_v1`. It is not a parser/runfile/user CLI surface and must fail closed on unsupported non-empty values. The selector does not require `physics_bulk_multilayer_density_v1`; when the selected density model is bulk, the thermal/liquid arm uses bulk-equivalent layers. | direct-production winter snow/liquid water-temperature path | opt-in rollback, trace proof, and supported output publication |
| `snow_layer_temperature` | `degC` | Per-layer snow temperature state for the Stage 3 diagnostic candidate. Snow layers must remain at or below freezing; energy above ripe conditions cannot create temperatures above `0 degC`. | `layered_thermal_liquid_v1` candidate / winter-column sub-solver | cold-content/refreeze and meltwater-temperature diagnostics |
| `snow_layer_cold_content` | `J m^-2` | Positive per-layer energy deficit to the freezing point. Stage 3 may reduce cold content through surface-energy/conduction/refreeze accounting but must close the diagnostic energy ledger. | `layered_thermal_liquid_v1` candidate / winter-column sub-solver | energy closure diagnostics |
| `snow_layer_liquid_water` | `m water equivalent` | Per-layer retained liquid water after routing, bounded by the 10.3.8 liquid holding-capacity authority and non-negative storage. | `layered_thermal_liquid_v1` candidate / winter-column sub-solver | liquid closure diagnostics |
| `snow_layer_refrozen_liquid` | `m water equivalent` | Per-layer diagnostic liquid refrozen by available cold content. It is bounded by incoming liquid and latent-heat capacity and cannot silently remove downstream CoE melt mass. | `layered_thermal_liquid_v1` candidate / winter-column sub-solver | refreeze and energy closure diagnostics |
| `snow_meltwater_flux_temperature` | `degC` | Typed intensive temperature assigned to the daily meltwater flux when positive routed liquid leaves the pack. After Paradigm 2 multilayer promotion, it is a supported hillslope-level water-temperature source published as nullable WAT parquet `MeltwaterTemperature`; it must not by itself activate HBP/watershed serialization or in-stream routing. | `layered_thermal_liquid_v1` production-supported opt-in / winter-column sub-solver / hillslope WAT output | stream-temperature source output |
| `snow_stage3_energy_residual` | `J m^-2` | Diagnostic residual for Stage 3 surface-energy, conduction, cold-content, and latent refreeze accounting. Material non-closure is a hard gate failure, not a hidden correction. | `layered_thermal_liquid_v1` candidate / winter-column sub-solver | Stage 3 conservation evidence |
| `snow_evaluation_operator` | `enum` | Evaluation-only operator identity: `same_state_paired_carrier_v1` or `sequential_resolved_shadow_v1`. Absence means no evaluation shadow. Unsupported or ambiguous values fail the evaluation request without changing production. | typed evaluation request | evaluation runner and evidence consumer only |
| `snow_evaluation_authority_tag` | `record` | Mandatory non-authoritative custody tag containing operator ID, named authoritative source snapshot, interval/support, cadence, carrier identity, coverage, unresolved boundaries, and admitted claim class. For the paired operator it additionally contains one pairing ID, exactly two stable carrier formulation/version arm IDs, per-arm component IDs with explicit not-applicable semantics, and identical snapshot, forcing, geometry (`z_T`, `z_q`, `z_u`, `z_0,aero`), cadence, coverage, and non-formulation-operand fingerprints. | evaluation-request resolver | evaluation evidence consumer only |
| `snow_evaluation_shadow_state` | `typed snow mass/enthalpy clone` | Independently owned evaluation state cloned from the tagged authoritative snapshot. It cannot alias, mutate, replace, route into, publish as, or select production state and cannot outlive the operator boundary. | declared evaluation operator | evaluator only; production consumers forbidden |
| `snow_evaluation_observability` | `internal tagged schema-v5 or schema-v6 record` | Schema v5 retains its historical evaluation-only operator/tag, fingerprint, component, state, residual, support, and coverage semantics. Schema v6 additively carries ordered duration-tagged substep primitives, effective carrier-input custody, control-volume projection, turbulent solver status, stability/exchange operands, radiation/albedo lineage, precipitation-advection operands, clone-state endpoints, and active/lower cold-content changes for exact independent reconstruction. Disabled/default rows remain exact schema v4. | typed evaluation evidence writer | internal evaluation consumer only; production/public consumers forbidden |
| `snow_stage3_reconciliation_identity_support` | `enum/count/s/hex-u64` | Schema-v6 operator, projection, applicability, tuple indices, elapsed/duration support, and source/forcing/geometry/effective/layer-state fingerprints. | enabled evaluation companion | independent internal reconciliation consumer only |
| `snow_stage3_reconciliation_state` | `kg m^-2`, `m`, `kg m^-3`, `J m^-2`, `degC`, or nullable same unit | Schema-v6 active/total clone mass, depth, density, cold content, temperature, layer membership, and before/after endpoints. Nullable active after-state is permitted only after typed post-substep meltout; total empty-state fields remain applicable. | enabled evaluation companion | independent internal reconciliation consumer only |
| `snow_stage3_reconciliation_forcing_radiation` | `degC`, `degC day`, `K`, `Pa`, `m s^-1`, `m`, `MJ m^-2 per hourly bin`, `MJ m^-2 d^-1`, `fraction`, `W m^-2`, `kg m^-2 s^-1`, or `J kg^-1 K^-1` | Exact hourly/daily forcing, virtual-instrument geometry, albedo lineage, shortwave/longwave primitives, and precipitation-advection operands used by the existing carrier. `snow_albedo_accumulated_positive_temperature_c_day` is explicitly `degC day`, not an instantaneous temperature. | enabled evaluation companion | independent internal reconciliation consumer only |
| `snow_stage3_reconciliation_turbulence` | `enum/count/fraction/m/m s^-1/kg m^-3/kg kg^-1/J kg^-1/W m^-2/kg m^-2 s^-1` | Exact Monin-Obukhov options, termination/status, stability corrections, log factors, exchange state, heat/mass fluxes, and nullable Obukhov length from the shared private solver. | enabled evaluation companion | independent internal reconciliation consumer only |
| `snow_stage3_reconciliation_endpoint_fluxes` | `W m^-2`, `J m^-2`, or `kg m^-2` | Exact external carrier, internal active/lower conduction, vapor exchange, melt/sublimation/deposition, active/lower cold changes, cold export, and closure operands over each tuple. | enabled evaluation companion | independent internal reconciliation consumer only |
| `snow_shallow_compaction_guard_depth_threshold` | `m` | Opt-in shallow-pack density guard threshold. SNOWDENSITY-10.3.17 fixes this threshold at `0.25 m`, derived from Marks/SNOBAL active surface-layer depth authority rather than fixture fitting. | `physics_bulk_shallow_guard_v1` candidate | density-compaction branch guard and diagnostic trace reconstruction |
| `snow_climate_class` | `enum` | Sturm snow climate class label for the climate-class density candidate: `tundra`, `taiga`, `alpine`, `maritime`, `prairie`, or `ephemeral`. Runtime class assignment must be derived from the run's own air-temperature, precipitation, and wind climate under Sturm 1995 authority; geographic or site-identity lookup is invalid. | `physics_bulk_climate_class_density_v1` candidate | density specialization and diagnostic trace reconstruction |
| `snow_climate_class_assignment_source` | `enum/string` | Provenance marker for climate-class assignment. Valid promotion evidence must be `forcing_derived_sturm1995`; `NSIDC-0768` may appear only as an independent cross-check and never as the runtime assignment source. Missing, non-authoritative, rare-category, or wind-ambiguous assignment must fail closed. | `physics_bulk_climate_class_density_v1` candidate | class-assignment audit and no-fixture-fitting guard |
| `sturm1995_climate_normals` | `(CDM degC-month, SPR mm d^-1, winter wind m s^-1)` | Run-derived climatological normals for the climate-class candidate: monthly mean air temperature drives CDM with `Tc=10 degC`; monthly precipitation rate is averaged over months with `Ta < Tc`; winter wind is averaged over the same cold-month set. The 1995 thresholds are authority values, never SNOTEL/cancov fitted values. | `physics_bulk_climate_class_density_v1` candidate | forcing-derived class assignment |
| `sturm2010_density_parameters` | `(rho_max, rho_0, k1, k2)` | Sturm 2010 Table 4 class-density trajectory parameters in `g cm^-3`, `g cm^-3`, `cm^-1`, and `day^-1` source units for alpine, maritime, prairie, tundra, and taiga. Ephemeral has no Sturm 2010 parameter row because ephemeral measurements were excluded; when the 1995 tree assigns ephemeral, the candidate must use the existing process-first fresh-snow/Anderson compaction behavior as an explicitly documented fallback and may not fabricate Sturm parameters. | `physics_bulk_climate_class_density_v1` candidate | class-density trajectory calculation and authority coverage |
| `sturm2010_density_day_of_year` | `day` | Day-of-year operand for the Sturm 2010 class-density trajectory form. It is runtime forcing/calendar state, not an observed-residual fitting variable. Non-finite, missing, or out-of-domain values must fail closed in the opt-in candidate. | `physics_bulk_climate_class_density_v1` candidate | class-density trajectory calculation |
| `snow_coe_boundary_depth` | `m` | CoE/legacy snow-depth carry used only to compute the next fixed CoE melt/liquid boundary when opt-in density mutates publication depth. | typed winter runtime carry | CoE melt boundary anti-alias evidence |
| `snow_coe_boundary_density` | `kg m^-3` | CoE/legacy snow-density carry used only to compute the next fixed CoE melt/liquid boundary when opt-in density mutates publication density. | typed winter runtime carry | CoE melt boundary anti-alias evidence |
| `snow_coe_boundary_settle_day_count` | `count` | CoE/legacy settle-day carry used only to compute the next fixed CoE melt/liquid boundary when opt-in density mutates publication depth/density. | typed winter runtime carry | CoE melt boundary anti-alias evidence |
| `snow_cold_content` | `J m^-2` | Candidate `physics_bulk` bulk snowpack energy deficit relative to isothermal melt conditions. | candidate physics-bulk snow solver | conservation diagnostics and melt/refreeze gates |
| `snow_liquid_water` | `m water equivalent` | Candidate `physics_bulk` retained liquid-water store within the snowpack before routed release. | candidate physics-bulk snow solver | conservation diagnostics and liquid forcing |
| `snow_liquid_holding_capacity` | `m water equivalent` | Opt-in CoE capacity-drainage candidate maximum retained liquid-water storage computed from snow depth, snow density/porosity proxy, and the non-fitted in-repo liquid holding-capacity ratio. | `coe_liquid_holding_capacity_v1` candidate | retained/released melt and rain partition |
| `snow_liquid_water_retained` | `m water equivalent` | Persistent retained liquid-water store carried by the opt-in snow lane after bounded melt/rain retention and excess drainage. | `coe_liquid_holding_capacity_v1` candidate | next-day capacity, conservation diagnostics, and snow-state closure |
| `snow_liquid_water_released` | `m water equivalent` | Melt/rain liquid in excess of holding capacity routed to downstream liquid forcing. | `coe_liquid_holding_capacity_v1` candidate | routed melt, WAT liquid balance, and coupled snow-control diagnostics |
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
| `snow_melt_model` | `enum` | Melt model selector. After SNOWDENSITY-10.3.15, absent direct-production selector state defaults to `coe_liquid_holding_capacity_v1`; `legacy_coe` remains an explicit rollback/test selector and compatibility comparator surface. SNOWDENSITY-10.3.16 authorizes `coe_open_sublimation_stage_a_v1` only as an opt-in diagnostic candidate. SNOWDENSITY-10.3.20 authorizes `coe_open_sublimation_stage_b_v1` only as an opt-in diagnostic candidate. `coe_shortwave_albedo_v1` and `coe_winter_thaw_state_loss_v1` remain non-default historical/diagnostic candidates unless separately re-ratified. | runtime configuration / winter column | CoE melt-term dispatch and diagnostics |
| `snow_sublimation` | `m water equivalent` | Opt-in Stage A vapor mass-loss sink from the snowpack. It is separate from routed melt/liquid, bounded by available snowpack SWE, published only in internal trace/conservation ledgers, and must not alter public WAT/HBP/PASS schema without later authority. | `coe_open_sublimation_stage_a_v1` candidate | open-surface ablation diagnostics and snow-state conservation |
| `snow_sublimation_surface_temperature_c` | `degC` | Stage B opt-in surface-layer temperature used to evaluate the saturation vapor-pressure surface for sublimation. It is bounded at or below freezing and derived from direct-production hourly forcing and SNOBAL active-layer authority, not from observed snow residuals. | `coe_open_sublimation_stage_b_v1` candidate | surface-layer sublimation diagnostics |
| `snow_sublimation_surface_layer_depth_m` | `m` | Stage B active surface-layer depth for sublimation diagnosis, bounded by snow depth and the Marks/SNOBAL active-layer ceiling (`0.25 m`). | `coe_open_sublimation_stage_b_v1` candidate | surface-layer cold-content diagnostics |
| `snow_sublimation_surface_layer_cold_content_j_m2` | `J m^-2` | Stage B diagnostic cold content of the active surface layer; non-positive in the SNOBAL convention and used to document the surface-temperature gate. | `coe_open_sublimation_stage_b_v1` candidate | surface-layer cold-content diagnostics |
| `snow_albedo` | `fraction` | Opt-in snow-surface albedo state consumed by the future `coe_shortwave_albedo_v1` shortwave term; accepted domain is `0 <= snow_albedo <= 0.85` under `brock2000_temperature_age_v1`. | SNOWDENSITY-05C albedo state update | `coe_shortwave_albedo_v1` melt path diagnostics |
| `snow_albedo_accumulated_positive_temperature_c_day` | `degC day` | Accumulated positive-temperature age index (`Ta`) since the latest fresh-snow reset for `brock2000_temperature_age_v1`. | SNOWDENSITY-05C albedo state update | albedo decay diagnostics |
| `snow_albedo_fresh_snow_reset_water_equiv_m` | `m water equivalent` | Fresh-snow water-equivalent increment threshold that resets `Ta` and returns albedo toward the fresh-snow cap; default core threshold is `0.001 m` water equivalent. | SNOWDENSITY-05C albedo state update | albedo reset diagnostics |
| `snow_albedo_model_id` | `enum/string` | Albedo-state provenance/model identifier; accepted opt-in value is `brock2000_temperature_age_v1`. | SNOWDENSITY-05C albedo state update | opt-in melt diagnostics and rollback evidence |
| `snow_melt_shortwave_absorbed_fraction` | `fraction` | Opt-in shortwave absorption operand for `coe_shortwave_albedo_v1`; equals `1 - snow_albedo` and is applied only to the CoE `amelt` term after the existing radiation source and canopy attenuation. | SNOWDENSITY-05D melt dispatch | raw melt reconstruction and rollback evidence |
| `winter_shortwave_source_provenance` | `enum/string` | Candidate provenance ledger naming the upstream gridded/provider source when known, the normalized climate `rad`/`radly` acceptance seam, units, slope/aspect and hourly transformation lineage, and proof that snowmelt does not receive a snow-only radiation scalar. | orchestration provenance plus climate runtime seam | `SC-CLIMATE-001#INV-CLIMATE-013` anti-alias evidence |
| `snow_wet_compaction_liquid_input` | `m water equivalent` | Exact daily liquid H2O presented to the Anderson/SNOBAL wet-compaction operator while snow cover is active at interval start, equal to the sum of positive hourly generated CoE melt before retention/routing plus all snow-contact rain (`rain_retained + rain_released`). Signed negative melt contributes zero. A mixed event that first creates a pack from zero start-of-hour cover does not enter H2O compaction in that interval, matching PySnobal's timestep `snowcover` gate. This is a process driver, not a storage or routed-liquid ledger. | active CoE snow coupling finalization | bulk and multilayer density wet-compaction input |
| `snow_density_process_diagnostics` | typed diagnostic record | Behavior-neutral daily density ledger carrying applicability; initial/final bulk density; exact initial snow load, wet-compaction liquid mass, and dry-compaction temperature drivers; snow-input mass/depth; directly evaluated fresh-snow density; signed bulk-density increments for fresh mixing, wet compaction, destructive metamorphism, overburden compaction, structural projection, climate fallback, internal/runtime caps, and downstream Stage-3 adjustment; an explicit climate-fallback-used Boolean; and a closure residual. Density values are `kg m^-3`, mass/load is `kg m^-2`, temperature is `degC`, and depth is `m`. | selected snow-density runtime | internal direct-production JSONL trace and diagnostic analysis only |
| `snow_accumulation_melt_diagnostics` | optional typed diagnostic record | Behavior-neutral hourly ledger carrying active precipitation water depth, rain depth, physical snowfall depth, snowfall SWE (`snowfall_depth * 0.1`), rain/snow fractions, phase-model identity, optional hydrometeor temperature, the four exact CoE empirical melt-depth contributions converted from legacy inches to `m` water equivalent, their uncapped sum, cap adjustment, applied raw melt, and explicit modeled wind-redistribution status/contribution. Depth terms are `m`; fractions are dimensionless; temperature is `degC`. The record is assembled only for an existing selected internal trace row and never controls physics. | SIMIMPL28 forcing plus SIMIMPL29 CoE melt evaluation | opt-in internal direct-production JSONL trace and diagnostic analysis only |
| `snow_signed_hour_diagnostics` | typed diagnostic record | Behavior-neutral extension of `snow_accumulation_melt_diagnostics` carrying exact hourly air temperature, radiation, cloud fraction, raw/applied/routed melt, liquid capacity/store/release, rain release, sublimation, and pack depth/density before and after, plus exact daily wind, dewpoint, and canopy forcing. It also carries existing duration-weighted Stage-3 hourly active/lower mass, depth, temperature, and cold content plus lower-volume present fraction; lower values retain full-hour weighting and are interpreted with that fraction. Units remain explicit at the JSON field boundary. | existing direct-runtime forcing, CoE snow coupling, and Stage-3 hourly diagnostics | additive internal direct-production JSONL trace v4 and diagnostic analysis only |
| `snow_solid_to_liquid_ledger` | compact typed production ledger | Exact daily raw signed melt, redistributed positive melt, authoritative bounded snowpack SWE loss, released rain, and routed-liquid handoff. All mass depths are `m` water equivalent. The independent identity is `liquid_handoff - snowpack_swe_loss - rain_released = 0`; raw and redistributed melt are context and cannot replace authoritative state loss. | exact existing CoE finalization and snow-partition outcomes | production/runtime handoff plus independent compact-ledger and JSONL reconstruction |
| `snow_liquid_disposition_ledger` | compact typed production ledger | Exact Stage-3 daily incoming liquid, routed liquid, signed retained-liquid change, refrozen liquid, and guarded producer residual. All values are `m` water equivalent; retained liquid means the Stage-3 layer-store delta, not the CoE retained store. Stage-3 incoming is the exact linked `snow_solid_to_liquid_ledger.liquid_handoff` argument whenever the solve is enabled. | exact existing Stage-3 routing outcome | production closure plus additive internal JSONL schema v4 and independent reconstruction |
| `snow_stage3_outcome` | compact typed production outcome | Stage-3 enabled state, total sublimation mass depth, and optional typed meltwater-flux temperature required by production snow state and downstream water-temperature handoff. | exact existing Stage-3 solve | production runtime/state projection on both trace-disabled and trace-enabled paths |
| `snow_verbose_diagnostic_capture` | optional boxed diagnostic record | Exact existing accumulation/melt, energy, cold-content, and 24-hour research fields assembled only when the existing nonempty JSONL path plus day/lane filter selects the row. Disabled, absent, empty, and filtered-out capture cannot remove closure guards or change any equation, state, selector, threshold, or ordinary output. | runner-derived typed capture request and existing producer values | real schema-v4 JSONL writer only |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SNOWFREEZE-001 | Melt bound and non-negativity branch semantics: post-branch exported melt satisfies `0 <= hrmelt <= Dsavail`, where `Dsavail` is the pre-hour available snow-depth state used by Eq. [3.6.1] branch logic. | hard-fail | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-MELT-ASSUMP, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-002 | Snow-density melt gate for legacy/CoE-default behavior: liquid melt export to infiltration/runoff is not allowed until post-update snow density reaches `ρsnew >= 350 kg m^-3`; below this threshold melt remains in-pack and increases density. The only currently authorized exceptions are explicitly opt-in candidates governed by `INV-SNOWFREEZE-066` and `INV-SNOWFREEZE-067`, which must preserve `legacy_coe` default behavior and publish separate evidence that positive thaw liquid leaves the snowpack through an auditable state-loss/drainage ledger rather than silently disappearing into density-only compaction. | hard-fail | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-SNOWDENS, INV-SNOWFREEZE-066, INV-SNOWFREEZE-067 | `[DIRECT][Static]` |
| INV-SNOWFREEZE-003 | Snow depth-density domain bounds: `Dsold >= 0`, `Dsnew >= 0`, `ρsold >= 0`, `ρsnew >= 0`, and `ρsnew <= 522 kg m^-3`; when `Dsnew = 0`, `ρsnew = 0`. | hard-fail | REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH3-SNOWDENS-LIM, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-004 | Active snow-update branch consistency: fresh snowfall contribution uses `100 kg m^-3` density and active depth/density updates follow Eq. [3.7.1]-[3.7.5] for settling, snowfall, melt, and melt+snowfall cases; drift-term equations remain governance-only while drift is inactive. | hard-fail | REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH3-DRIFT-INACTIVE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-005 | Rain/snow partition consistency: precipitation phase partition follows daily temperature logic (`Tmax < 0` => all snow; `Tmin > 0` => all rain; mixed day uses hourly occurrence/diurnal temperature function). | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-HRPRECIP, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static]` |
| INV-SNOWFREEZE-006 | Frost heat-flow formulation consistency: frost/thaw bookkeeping uses explicit layered heat-flow relations (`Qsrf`, `Quf`) and harmonic-mean layered thermal conductivity per Eq. [3.8.1]-[3.8.4]. Frost top heat flow must consume the legacy `hr_tmp`/`tmpadj` adjusted hourly surface temperature (`surtmp(hour)`), not raw hourly air temperature, with positive snow-covered surface temperatures capped to `0 degC`. Executable frost-depth progression must derive from hourly signed heat flow, latent-heat increments, and the active fine-layer frozen-depth/frozen-water state (`wb18_perc_frozen_depth_####`, `wb18_perc_frzw_####`). Freeze energy advances the front only by freezing layer water into the same store that `frwatc` later publishes; thaw energy retreats the active front by melting that same layer ice back into liquid storage. The surface heat path must be resisted by the current frozen-layer thickness (`Σ dz/k` through snow, residue, tilled frozen soil, and untilled frozen soil), with residue depth following the legacy `resdep` lineage, frozen tilled/untilled conductivity using the fixed legacy `kftill = 1.75` and `kfutil = 2.1 W m^-1 degC^-1` constants, and below-freezing shallow-front soil distance floored to the midpoint of the first fine layer (`dpfsfl = dg(1)/nfine(1)/2`, legacy fallback `0.005 m` only when fine-layer geometry is unavailable). The lower heat path remains a separate `Quf` term computed from the seasonal `tmpbl` curve and the `frostn.for:430-458` per-fine-layer unfrozen harmonic conductivity path over one metre below the frost front (`slsw`, `bdcons`, `ksoilf`; `kufz = 0.2` only when no positive conductivity terms exist), and freeze-active `frzng` execution must recompute surface resistance and `Qsrf` after each in-hour fine-layer front advance before consuming additional freezing time. Depth must be bounded by the physical soil profile and by the layer/fine-layer capacity exposed by that state; it must not use the retired `0.20 m * clamp(mean-temperature / 6 degC)` freeze-index proxy, a `0.20 m` model cap, post-hoc scalar depth projection into layer stores, a scalar `frdp * theta` frozen-water surrogate, raw hourly air temperature as the frost surface-temperature driver, zero-residue publication when management residue is present, per-soil frozen-path conductivity in place of fixed `kftill`/`kfutil`, or a constant lower-front conductivity when positive unfrozen layer terms are available. | hard-fail | REF-SNOWFREEZE-CH3-FROST, REF-SNOWFREEZE-LEGACY-TMPADJ, REF-SNOWFREEZE-LEGACY-FROSTN-QDRY, REF-SNOWFREEZE-LEGACY-WINTER-RESDEP, REF-SNOWFREEZE-LEGACY-FROZEN-PATH-KF | `[DIRECT][Static]` |
| INV-SNOWFREEZE-007 | Winter coupling payload completeness: hourly winter outputs required for downstream consumers are emitted with valid units/domains, including `hrmelt`, `Dfrost`, `Dthaw`, `Nft`, `Ws_frz`, and `InfCap_frz`, and daily snow-water term `S` is consistently reflected in water balance semantics. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-FROST, REF-SNOWFREEZE-CH4-COUPLING, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-008 | Snow drifting governance invariant: process claims requiring active drift transport equations are non-promotable until authority confirms an active drift implementation path for the target lineage. | governance-fail | REF-SNOWFREEZE-CH3-DRIFT-INACTIVE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-009 | Winter-routine activation branch is explicit: winter hourly processing is invoked when at least one trigger condition is true (existing snowpack, existing soil frost layer, or average daily temperature below `0 degC`), with no silent bypass. Activation depends on runtime state/forcing triggers, not snow-sidecar or frost-sidecar presence alone; parsed default snow/frost controls are valid controls when the missing-file branch has explicitly set defaults. For standard `ksflag` frost, `frost.options.frost_file_present` is provenance only and must not suppress the frozen-soil routine when `frost.options.wintRed=1` and thermal/runtime frost triggers are active. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, INV-SNOWFREEZE-012, INV-SNOWFREEZE-013 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-010 | CLIM05 parsed snow-control coupling invariant: when parsed `snow.options.*` controls are projected to runtime, coupling must enforce finite/valid control domains (`newsnw > 0`, `ssd > 0`, `newsnw <= ssd`), publish signed `S = melt - accumulation`, and maintain non-negative `snow.runtime_swe` without silent fallback/defaulting. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH5-COUPLING, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-011 | SIMIMPL18 day-key partition/publication closure: for active snow coupling and precipitation days where `Tmax <= rst`, liquid runoff-coupling input from direct rainfall/melt is zero for that day key (`RM = 0`), snow storage update remains explicit (`snow.runtime_swe(new) = snow.runtime_swe(old) + accumulation - melt`), and downstream published `Snow-Water`/hydout-equivalent snow storage values derive from runtime SWE state rather than static sidecar control `snow.options.ssd`. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-HRPRECIP, REF-SNOWFREEZE-CH5-COUPLING, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-012 | Frost routine-chain dispatch and handoff closure: active winter-hourly frost triggers dispatch `winter -> frostN`, `frostN` performs water-state handoff with `frwatc(1)` once at active-day hour-1 ingress (`frostn.for:335-337`, guarded by `hour.eq.1`) and `frwatc(0)` at day-end/thaw-complete exit, and freeze-active branches execute `frzng -> frznw` lineage without silent bypass. Implementations must not interpret `frwatc(1)` as an every-hour entry handoff because that would reapply the daily `st - yst` water-balance delta. When a computed frost outcome carries a material `frwatc` exchange, including a thaw-complete outcome with zero final frost, the original coarse layer basis must remain unchanged until the authoritative outcome projection and `soil_water_after_frwatc` scalar are applied together exactly once at frost ingress. A pre-ingress synthetic zero-frost clear/rebalance against the pre-exchange soil-water scalar is prohibited because it restores residual water over newly unfrozen thickness before the corresponding thaw handoff and can debit below the residual store. A stale coarse projection may be cleared separately only when no material `frwatc` exchange exists. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-FROST, REF-SNOWFREEZE-LEGACY-FRWATC-EGRESS, SC-WATBAL-001#INV-WATBAL-095 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-013 | Frozen-soil conductivity authority closure: when frost is present, saturated-conductivity coupling follows `frsoil` fine-layer aggregation with `getFreezeCond` land-use-dependent `kfactor` selection and remains explicitly bounded/typed at the runtime seam (`frost.runtime_infcap_frz`). | hard-fail | REF-SNOWFREEZE-CH3-FROST, REF-SNOWFREEZE-CH4-COUPLING, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-014 | HPHYS0268 spring snowpack lineage closure: material H1/H7/H39 seasonal `Ep` divergence claims must expose baseline-authoritative `winter -> snowd -> melt` lineage for runtime SWE/depth/density/settle carry state, hourly rain/snow/melt sums, signed `S`, WB13 `RM`, and WB13 `Snow-Water` before returning residual ownership to WB17 `Ep`. Active snowpack execution is governed by runtime snow/frost/thermal triggers and parsed/default snow controls; `snow.options.snow_file_present` may only select parsed-vs-default control provenance and must not gate whether snow processing runs. Non-agricultural HPHYS parity keeps frost disabled while snow remains active. | governance-hold | INV-SNOWFREEZE-009, INV-SNOWFREEZE-010, INV-SNOWFREEZE-011, REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-015 | HPHYS0269/HPHYS0303 baseline winter melt/snowpack invariant: openWEPP snowpack migration must preserve fixed `wepp_260430` daily negative-melt redistribution authority while preserving baseline `snowd.for`/`melt.for` signed hourly melt and rain-on-snow holding-capacity lineage. `melt.for` may emit negative hourly `hrmelt_raw`; only positive raw melt is bounded to available snow during the hourly melt branch. Corrected `winter.for` daily post-processing compares net daily melt (`pstvML + ngtvML`) and, when positive, scales positive hourly melt by `1 + ngtvML/pstvML` before daily routed melt is summed; the archived original `pstvML <= ngtvML` and `1 - ngtvML/pstvML` branch is rejected as bug-compatible archaeology, not active comparator behavior or target physics. **This net-melt-scaling describes the legacy `winter.for` porting reference (an ungated melt model); for the openWEPP production routing when positive and negative hourly melt COEXIST, `INV-SNOWFREEZE-019` (SNOWSCI-S1 single-source depth/density store) SUPERSEDES the net-scaling — routed melt equals the authoritative storage loss (`old SWE + snowfall + retained rain − new SWE`), i.e. the positive water-equivalent loss already applied to the store, and negative raw melt must not create a second SWE debit. Under the `INV-SNOWFREEZE-002` density gate, sub-350 kg m⁻³ hours retain positive melt IN-pack (never exported) so a below-gate refreeze is of in-pack liquid and must NOT be netted against above-gate exported melt; positive-parts routing is therefore the correct openWEPP behavior and is equivalent to the net-algebra on the density-gate-consistent reachable domain (no openWEPP production OFE-day has been observed to reach a mixed exported-melt-plus-refreeze branch). If that branch becomes reachable, the divergence must be re-adjudicated under `INV-SNOWFREEZE-019`, not silently shipped.** `snowd.for` rain-on-snow storage consumes hourly rain into snowpack density while `ρsnew < 350 kg m^-3`; retained rain increases runtime SWE and contributes negative daily `S` just like snowfall accumulation. Residual rain-on-snow that remains in `hrrain` after holding-capacity accounting must be handed to daily `winter.for` post-processing and added into `hrmlt`/`wmelt` before downstream liquid forcing. | hard-fail | INV-SNOWFREEZE-019, INV-SNOWFREEZE-002, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-LEGACY-SNOWD-RAINSTORE, REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| INV-SNOWFREEZE-047 | Frost-depth observation-validation correspondence invariant (`GAP-SNOWFREEZE-002`; accepted): modeled frost-front depth `frdp` is validated against historic site observations as an external authority (ADR-0017 - observation agreement is the acceptance target; legacy/compatibility frost output is only a flag). The observation-to-`frdp` correspondence is fixed by measurement method and must not be conflated: (a) frost-tube depth (the frozen/unfrozen free-water boundary) is the magnitude authority and compares to `frdp` directly within `TOL-SNOWFREEZE-007`; (b) soil-temperature `0 degC`-isotherm depth is a timing authority and an upper bound on `frdp`, validated for onset/thaw timing and frozen duration within `TOL-SNOWFREEZE-008` and for magnitude only as `frdp <= isotherm_depth + TOL-SNOWFREEZE-007`; (c) penetrometer/mechanical-resistance depth is method-dependent and secondary/non-authoritative for magnitude. `frdp` is the bottom extent of the modeled frozen domain, not necessarily the surface thaw state: branch-3 top thaw may grow `thdp` (a surface-thawed cap) while `frdp` remains fixed. Therefore late-thaw residuals that persist only because `frdp > 0` while `thdp` advances must be tagged as a measurement-correspondence / bounded residual before being treated as frost physics. A frost-depth divergence may be classified `OPENWEPP-DEFECTIVE` only when all hold: (1) modeled snow depth is controlled under `INV-SNOWFREEZE-048`/`TOL-SNOWFREEZE-009` or routed as forcing-limited under `INV-SNOWFREEZE-050`; (2) the comparison is like-for-like by method per (a)-(c); (3) censoring is honored; and (4) the divergence exceeds the accepted tier tolerance over a defined aggregation. Divergences failing (1)-(3) are `HARNESS-SURFACE-MISMATCH`, `FORCING-LIMITED`, or `UNRESOLVED`, never silently a model defect. This invariant validates fidelity only and does not relax the `INV-SNOWFREEZE-006` heat-flow formulation authority. | hard-fail | REF-SNOWFREEZE-FROST-OBS, REF-SNOWFREEZE-FROST-RATIFICATION-20260629, INV-SNOWFREEZE-006, INV-SNOWFREEZE-012, ADR-0017, TOL-SNOWFREEZE-007, TOL-SNOWFREEZE-008, TOL-SNOWFREEZE-009 | `[DIRECT][Ran] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-048 | Snow-depth observation correspondence and anti-alias invariant (`GAP-SNOWFREEZE-002`; accepted): the snow-control operand for `INV-SNOWFREEZE-047` is physical snowpack depth, not snow-water equivalent. Modeled depth must be WAT `Snow-Depth` (`mm`) converted from `snow.runtime_depth_m` and may not be substituted with WAT `Snow-Water`, `snow.runtime_swe`, snowfall depth, melt, or any water-storage surface. A snow-control failure may be routed as a snow-depth fidelity issue only after the harness proves all of the following for the paired rows: (1) observed source field semantics are physical snowpack depth with units normalized to `m`; (2) modeled and observed dates represent the same daily stage or any stage difference is explicitly classified; (3) signed residual direction, magnitude, and over/under counts are published, not only absolute residuals; (4) depth-vs-SWE anti-alias evidence shows the failure is not better explained by comparing observed depth to a water-equivalent alias; and (5) missing paired snow rows remain `INCONCLUSIVE`, not a snow or frost defect. When these proofs pass and `TOL-SNOWFREEZE-009` fails, frost-depth attribution is routed through `INV-SNOWFREEZE-050`: magnitude-only snow residuals may be forcing-limited/report-only, while forcing-robust timing/regime/slope snow defects block frost attribution. The Step 1 `>0.25` systematic-timing-fraction cutoff is not ratified as an invariant; it remains a diagnostic-script aggregation rule, and future fraction cutoffs must be package-declared and sensitivity-reported rather than silently inherited. | hard-fail | REF-SNOWFREEZE-FROST-OBS, REF-SNOWFREEZE-FROST-RATIFICATION-20260629, INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, TOL-SNOWFREEZE-008, TOL-SNOWFREEZE-009, ADR-0017 | `[DIRECT][Ran] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-049 | SNOTEL observed-density correspondence invariant (`GAP-SNOWFREEZE-002`; draft): paired SNOTEL `WTEQ` and `SNWD` rows provide an external-authority snowpack bulk-density surface for snow-depth fidelity adjudication. `WTEQ` is snow-water equivalent in inches and must be normalized to millimeters water equivalent; `SNWD` is physical snowpack depth in inches and must be normalized to meters. Observed bulk density is computed as `observed_density_kg_m3 = observed_swe_mm / observed_snow_depth_m` only when both SWE and physical depth are positive and like-for-like for the same date. Rows with absent depth, absent SWE, zero/trace depth, or impossible/non-finite density are excluded from density verdicts rather than repaired. The `snow.txt` SSD comparison arm may use an observed climatological density only when it is derived before residual comparison from peak-SWE-period SNOTEL density and documented with the derivation; choosing SSD by minimizing modeled-vs-observed depth residual is invalid. Legacy WEPP and PySnobal remain diagnostic flags under ADR-0017; observed SNOTEL SWE/depth/density plus this correspondence are the adjudication authority for the over-accumulation vs low-density fork. Density and SWE tolerances remain provisional under `TOL-SNOWFREEZE-010`; failure routes to snow-depth/density fidelity characterization, not directly to frost or production physics edits. | governance-hold | INV-SNOWFREEZE-003, INV-SNOWFREEZE-048, TOL-SNOWFREEZE-009, TOL-SNOWFREEZE-010, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-050 | Snow/frost fidelity evaluation-rubric invariant (`GAP-SNOWFREEZE-002`; accepted): model fidelity to snow/frost observations is assessed by the signature-based, multi-timescale rubric in the GAP-SNOWFREEZE-002 Snow/Frost Fidelity Evaluation Rubric Addendum, not by any single residual tolerance. The rubric decomposes the comparison into process-diagnostic signatures across long-term, seasonal, and event timescales (accumulation, densification/settling, peak magnitude and timing, ablation, rain-on-snow, regime ordering, conservation; for frost: onset, deepening, thaw, frozen duration). Each signature is tagged forcing-robust (`R`) or forcing-limited (`L`): `R` signatures carry model verdicts, while `L` signatures are reported but may not by themselves produce an `OPENWEPP-DEFECTIVE` verdict because they are dominated by forcing and representativeness uncertainty. Time-series cells score by Kling-Gupta-Efficiency decomposition; magnitude cells score by median signed bias and IQR; timing cells by date offset. Cell pass-levels use the accepted noise-floor bands `TOL-SNOWFREEZE-007/008/009/010/011` as evaluation bands, not calibration targets. The output is a per-model, per-site, per-cell profile; the ADR-0017 verdict taxonomy is applied per cell; legacy WEPP and PySnobal are scored on the same rubric as diagnostic flag profiles, never as targets. For frost default activation, this invariant adopts the observed-data Policy B analog: activation may proceed when residuals are attributed and bounded and full-surface no-regression/conservation gates pass; zero paired observation failures are not required. The accepted bounded residual state includes forcing-limited/sparse snow-buried thaw persistence, two deferred snow-free wet-heat/Qwet cells, two H1b `frdp` metric-edge cells plus `15` true-stall days out of `570` branch-3 warm/material days, two early-onset cells, Morris snow-control blocking, and Mandan/Reynolds no-paired-snow inconclusive routing. This invariant governs evaluation and activation method only; it changes no physics and supersedes use of `TOL-SNOWFREEZE-009` as a standalone snow-model acceptance band. | hard-fail | INV-SNOWFREEZE-047, INV-SNOWFREEZE-048, INV-SNOWFREEZE-049, REF-SNOWFREEZE-FROST-OBS, REF-SNOWFREEZE-FROST-RATIFICATION-20260629, ADR-0011, ADR-0017, TOL-SNOWFREEZE-011 | `[DIRECT][Ran] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-051 | SNOWDENSITY-02 opt-in `physics_bulk` snow-model envelope: openWEPP may develop a deliberate-legacy-divergence snow-density candidate only behind `snow_model = legacy_wepp | physics_bulk`, with `legacy_wepp` remaining the default and `physics_bulk` explicitly opt-in until a later activation package passes this contract's gates. `physics_bulk` is a candidate physics-bulk snowpack model, not an authorized production formula in this amendment. Its candidate state envelope includes SWE, physical depth, bulk density, retained liquid water, snow cold content or bulk temperature, and snow-cover age/metamorphism clock. Its candidate process envelope may include temperature/wind-dependent fresh-snow density, Anderson-1976/SNOBAL-style metamorphism, overburden compaction, wet-snow compaction, liquid retention/release/refreeze, and internal mass/energy closure, but exact equations/constants require SNOWDENSITY-03/04 evidence and hydrology-reviewer ratification before runtime promotion. Site-specific calibration is prohibited: SNOTEL, observed snow depth, legacy WEPP, and PySnobal may classify profile cells and diagnose failure modes, but they may not fit per-site constants, choose SSD by residual minimization, or tune `physics_bulk` separately for a site. `ssd` remains a legacy control-state/provenance input, not a `physics_bulk` density or settlement parameter. PySnobal/SNOBAL and legacy WEPP remain diagnostic flag profiles under ADR-0017, never acceptance targets. Frost physics attribution remains blocked until snow-control failures are passable or bounded by the v74/v75 rubric. | governance-hold | INV-SNOWFREEZE-048, INV-SNOWFREEZE-049, INV-SNOWFREEZE-050, REF-SNOWFREEZE-SNOWDENSITY01, REF-SNOWFREEZE-ANDERSON1976-CANDIDATE, REF-SNOWFREEZE-SNOBAL-CANDIDATE, ADR-0017, ADR-0026, ADR-0027 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-052 | SNOWDENSITY-05A CoE melt-modernization contract and sign-convention envelope: openWEPP may develop a deliberate-legacy-divergence melt candidate only behind `snow_melt_model = legacy_coe | coe_shortwave_albedo_v1`, with `legacy_coe` remaining the default, compatibility comparator surface, and rollback path. `coe_shortwave_albedo_v1` is opt-in only and is not production-authorized until SNOWDENSITY-05B/05C/05D bind the shortwave source, albedo state, implementation, and conservation evidence. The modernized path must preserve the WEPP Chapter 3 / Corps-of-Engineers melt-term family (`amelt`, `bmelt`, `cmelt`, `dmelt`), current canopy attenuation `(1 - cancov)`, signed raw melt, corrected daily redistribution, density gate, and routed-melt lineage. Sign convention is fixed before code: WEPP Chapter 3 prose writes this term as `- bmelt`, while openWEPP trace field `melt_bmelt_in` stores the signed `bmelt` contribution, so the runtime raw-melt identity is `hrmelt_raw = 0.0254 * (amelt + melt_bmelt_in + cmelt + dmelt)` as guarded by `tests/integration/clim05_snow_runtime_kernel_contract.rs`; any silent sign flip or double subtraction is invalid without a contract amendment. `dense_slow_melt_v1` remains a negative benchmark, not a promotable production melt path, because its profile improvement reduced a degree-day melt factor to mask a density gap. Shared radiation forcing must not be tuned or rescaled for snowmelt because `SC-CLIMATE-001#INV-CLIMATE-013` owns the winter radiation unit/provenance seam and the same forcing drives other hydrology/ET consumers. | governance-hold | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-MELT-ASSUMP, SC-CLIMATE-001#INV-CLIMATE-013, REF-SNOWFREEZE-MELT-OHMURA2001, REF-SNOWFREEZE-MELT-PELLICCIOTTI2005, REF-SNOWFREEZE-MELT-CARENZO2009, REF-SNOWFREEZE-MELT-BROCK2000, REF-SNOWFREEZE-MELT-WALTER2005, REF-SNOWFREEZE-MELT-GUPTA2023, INV-SNOWFREEZE-015, INV-SNOWFREEZE-050, INV-SNOWFREEZE-051 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-053 | SNOWDENSITY-05B shortwave source/provenance binding: the canonical openWEPP shortwave source is the daily climate `rad`/`radly` field in `Ly d^-1` at the climate parser/runtime seam. Upstream gridded radiation products may feed openWEPP only by being normalized by orchestration into this field with external provenance; openWEPP must not fetch, select, spatialize, or tune gridded shortwave products or maintain a snow-only radiation source. Runtime transformation is exactly the `SC-CLIMATE-001#INV-CLIMATE-013` lineage: preserve `radly` as `Ly d^-1`, perform the single conversion `radmj = radly * 0.04184`, apply slope/aspect transformation through `sunmap`, distribute hourly through `radcur`/`hr_tmp` including the near-isothermal `radmj/24` branch, and publish `winter.hourly.rad_mj_m2_####` in `MJ m^-2 h^-1`. ET and snowmelt consume the same daily radiation authority (`rad`/`RA`/`radiation_ly`); any fitted radiation scalar, already-MJ double conversion, Langley-scale value under an MJ label, silent clipping, or snow-only radiation scalar is invalid. | governance-hold | SC-CLIMATE-001#INV-CLIMATE-013, SC-EVAP-001#INV-EVAP-021, REF-SNOWFREEZE-CH3-HRRAD, REF-SNOWFREEZE-CH3-MELT, INV-SNOWFREEZE-052 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-054 | SNOWDENSITY-05C albedo-state core: the accepted opt-in albedo model for future `coe_shortwave_albedo_v1` implementation is `brock2000_temperature_age_v1`, a temperature-age snow-albedo state derived from Brock et al. (2000) and not fitted to openWEPP SNOTEL or frost-site observations. The state carries `snow_albedo`, `snow_albedo_model_id`, and `snow_albedo_accumulated_positive_temperature_c_day` (`Ta`) and must reset `Ta` when fresh snowfall meets `snow_albedo_fresh_snow_reset_water_equiv_m`. For active snow cover, compute deep-snow albedo as `0.713 - 0.112 * log10(Ta)`, shallow-snow albedo as `underlying_albedo + 0.442 * exp(-0.058 * Ta)`, and combine them with `alpha = (1 - exp(-d / d_star)) * alpha_deep + exp(-d / d_star) * alpha_shallow`, where `d` is snow water-equivalent depth and `d_star = 0.024 m water equivalent`. Computed albedo is bounded to `0 <= snow_albedo <= 0.85`; the upper cap is Brock's reported fresh-snow bound and the lower bound is physical. `legacy_coe` must not consume or require this state. `coe_shortwave_albedo_v1` on active snow must fail closed on missing opt-in albedo state, missing model id, non-finite/negative `Ta`, non-finite/out-of-range albedo, or model-id mismatch; it must not silently synthesize a hidden default. This invariant ratifies the albedo state core only and does not authorize routed-melt wiring, default activation, radiation-source changes, or coefficient fitting. | hard-fail | REF-SNOWFREEZE-MELT-BROCK2000, INV-SNOWFREEZE-052, INV-SNOWFREEZE-053 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-055 | SNOWDENSITY-05D opt-in CoE melt implementation: `legacy_coe` remains the default and must preserve current routed-melt behavior. When `snow_melt_model = coe_shortwave_albedo_v1`, the only authorized production melt-term change is the CoE shortwave operand `amelt = 0.0607 * hrrad * (1 - snow_albedo) * (1 - cancov)`, where `hrrad` is the existing `winter.hourly.rad_mj_m2_####` source from `INV-SNOWFREEZE-053`, `snow_albedo` is the typed `brock2000_temperature_age_v1` state from `INV-SNOWFREEZE-054`, and the existing canopy factor remains unchanged. `bmelt`, `cmelt`, `dmelt`, signed raw melt, positive-melt depth cap, density gate, rain retention/release, corrected negative-melt redistribution, runtime SWE/depth/density mutation, WB12 `S`, and WB13 `RM`/liquid forcing lineage must remain the same algorithmic path. The opt-in path must carry the updated albedo state in typed runtime state and fail closed on missing/invalid active-snow state rather than falling back to `legacy_coe`. Acceptance requires independent typed-operand reconstruction of hourly raw melt, redistributed melt, routed `wmelt`, snowpack SWE loss, WB12 signed liquid forcing, and WB13 routed liquid forcing. | hard-fail | INV-SNOWFREEZE-052, INV-SNOWFREEZE-053, INV-SNOWFREEZE-054, INV-SNOWFREEZE-015, INV-SNOWFREEZE-022, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-MELT-BROCK2000 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-056 | SNOWDENSITY-05F melt closure / density handoff: SNOWDENSITY-05F closes the melt-modernization ladder without default activation. `legacy_coe` remains the default and rollback path; `coe_shortwave_albedo_v1` remains opt-in only and is accepted only as a density-facing interface for later `physics_bulk` density work. The density-facing interface is the selector `snow_melt_model`, the 05B hourly shortwave source `winter.hourly.rad_mj_m2_####`, 05C albedo state/model/age/reset operands, 05D absorbed-shortwave/raw-melt/redistributed-melt/routed-`wmelt`/SWE-loss/WB12/WB13 liquid-forcing operands, and runtime SWE/depth/density after-state. The activation evidence baseline requires both diagnostic replay and H as-built context: 05E's diagnostic legacy improvement (`robust_fail_count 13 -> 10`, `robust_ordinal_score 61 -> 84`) is insufficient by itself because H as-built context remained `robust_fail_count=9`, `robust_ordinal_score=84`. Those 05E diagnostic replay deltas are regime-limited because the diagnostic harness used `cancov = 0.0` and PySnobal-bridge radiation rather than the configured coniferous forest winter canopy cover of about `0.9` and the native/proven 05B shortwave source. Same-day future snowfall is an explicit cold-start albedo continuity case: when the opt-in path has active same-day snowfall after earlier snow-free hours, the producer must preserve typed albedo continuity through fresh-snow reset, carry a valid previous opt-in state, or fail closed rather than silently clearing albedo only because the morning state was snow-free. SNOWDENSITY-06 may consume the opt-in melt boundary without retuning melt, radiation, or coefficients, but its entry gate must first repair or prove harness fidelity for real per-day canopy cover, with configured coniferous forest winter `cancov` expected near `0.9`, and native/proven shortwave radiation; density packages must not use melt changes as density compensation and must not promote parser/runfile/CLI selectors, output schemas, or default activation without a later ratified activation package. | governance-hold | INV-SNOWFREEZE-050, INV-SNOWFREEZE-052, INV-SNOWFREEZE-053, INV-SNOWFREEZE-054, INV-SNOWFREEZE-055, INV-SNOWFREEZE-051, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-057 | SNOWDENSITY-05G harness-fidelity rerun: diagnostic `coe-melt` SNOTEL adjudication may supersede the 05E regime-limited replay only when the replay consumes configured openWEPP canopy state instead of a `cancov = 0.0` harness constant and publishes the shortwave lineage used for `hrrad`. For the configured coniferous validation fixtures, accepted representative-regime evidence must demonstrate `canopy_cover_fraction` near `0.9` for the winter replay, with the value sourced from the generated openWEPP runtime surface or an explicitly equivalent per-day growth-state series. Shortwave evidence must either consume the native `winter.hourly.rad_mj_m2_####` source from `INV-SNOWFREEZE-053` or prove the PySnobal bridge inversion is like-for-like by recording `net_solar_Wm-2 = native_shortwave_MJ_m-2_h-1 * 1_000_000 / 3600 * 0.8` and `hrrad = net_solar_Wm-2 * 3600 / 1_000_000 / 0.8`, with no fitted radiation scalar. The representative 05G rerun supersedes the 05E promotion-candidate context with a `NON-PROMOTION` disposition for default activation: `legacy_coe` and `coe_shortwave_albedo_v1` both have `robust_fail_count=9`, while the opt-in ordinal score rises only from `84` to `86`. The 05G rerun does not authorize default activation, parser/runfile/CLI selectors, output schemas, coefficient retuning, density-physics changes, or frost attribution. | hard-fail | INV-SNOWFREEZE-050, INV-SNOWFREEZE-053, INV-SNOWFREEZE-055, INV-SNOWFREEZE-056, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-058 | SNOWDENSITY-06 density-only Anderson/SNOBAL compaction candidate: after SNOWDENSITY-05G, density work may add offline `physics_bulk` variants that change only snowpack density/compaction constants while preserving fixed melt, albedo, canopy, and shared-radiation boundaries. The first authorized variant is `density_compaction_v1`, an offline snowbench candidate with baseline candidate melt constants unchanged and named SNOBAL-lineage compaction constants for destructive temperature metamorphism (`ptm_rate_per_hour = 0.01`, `ptm_density_threshold = 100 kg m^-3`, `ptm_density_decay = 0.046 kg^-1 m^3`, `ptm_temperature_decay = 0.04 degC^-1`), overburden compaction (`poc_rate_per_hour = 0.026`, `poc_temperature_decay = 0.08 degC^-1`, `poc_density_decay = 21.0`, `swe_max = 2000 kg m^-2`, `rate_cos_amplitude = 23.5`, `rate_offset = 24.5`, `max_density = 550 kg m^-3`), and liquid-water compaction (`wet_half_saturation_ratio = 0.4`, `wet_max_density = 550 kg m^-3`). The variant may adjust only fresh-snow-density and compaction-strength constants within the SNOWDENSITY-02 candidate envelope; any melt coefficient, albedo constant, canopy series, radiation scalar, or site-specific parameter change invalidates its evidence. Evaluation must publish both whole-rubric context and a density/densification robust-cell profile covering `long_term_cold_season_bulk_density`, `seasonal_densification_trajectory`, `seasonal_depth_swe_slope`, and `cross_cutting_bias_sign_consistency`. A package may close as non-promotion if finite evidence shows those density cells do not beat legacy/as-built; it must not reinterpret the failure as a need to retune melt or default-activate `coe_shortwave_albedo_v1`. | governance-hold | INV-SNOWFREEZE-050, INV-SNOWFREEZE-051, INV-SNOWFREEZE-056, INV-SNOWFREEZE-057, REF-SNOWFREEZE-ANDERSON1976-CANDIDATE, REF-SNOWFREEZE-SNOBAL-CANDIDATE, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-059 | SNOWDENSITY-06B CoE-bound density replay: after SNOWDENSITY-06 proves `density_compaction_v1` can improve density cells only under the old degree-day `physics_bulk` melt surrogate, the next authorized evidence surface is an offline `coe_bound_density_compaction_v1` replay that consumes fixed CoE daily snowpack SWE/liquid boundaries from `legacy_coe` and optionally `coe_shortwave_albedo_v1`. The replay must preserve CoE `snow_water_m` identity for every daily row and may change only depth/density through the `density_compaction_v1` fresh-snow density and compaction update. It must not reuse `physics_bulk` degree-day melt, retune any melt/albedo/radiation/canopy constants, fit site-specific density parameters, alter production runtime selectors, or claim frost attribution. Evidence must publish the CoE boundary model, daily CoE SWE identity residual, routed-melt/SWE-loss boundary totals, whole-rubric context, density/densification robust-cell profile, and comparator context before SNOWDENSITY-07 runtime opt-in can be scaffolded. Finite non-promotion evidence must name the next blocker rather than route to mixed/deciduous canopy work inside this package. | governance-hold | INV-SNOWFREEZE-050, INV-SNOWFREEZE-056, INV-SNOWFREEZE-057, INV-SNOWFREEZE-058, REF-SNOWFREEZE-ANDERSON1976-CANDIDATE, REF-SNOWFREEZE-SNOBAL-CANDIDATE, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-060 | SNOWDENSITY-07 runtime opt-in: the accepted CoE-bound density result may be coupled into the typed winter-column/direct snow-coupling runtime only behind `snow_density_model = physics_bulk_density_compaction_v1`, with `snow_density_model = legacy_wepp` as the default, compatibility surface, and rollback path. The opt-in producer must preserve CoE SWE/liquid authority: `snow.runtime_swe`, signed `S`, raw melt, redistributed melt, routed `wmelt`, post-winter rain, snowpack SWE loss, albedo state, and downstream WB12/WB13 liquid-forcing operands remain those produced by the selected CoE melt boundary (`legacy_coe` or `coe_shortwave_albedo_v1`). The density model may mutate only physical `snow.runtime_depth_m` and `snow.runtime_density_kg_m3` by applying the SNOWDENSITY-06B `density_compaction_v1` fresh-snow, dry-compaction, and wet-compaction update, then force-normalizing mass back to the CoE runtime SWE. To prevent aliasing, the runtime must carry a separate CoE boundary depth/density/settle-count state for the next CoE melt calculation whenever opt-in density depth/density differ from the boundary state; feeding opt-in density/depth back into the CoE melt boundary is invalid unless a later contract amendment explicitly retires the boundary split. Acceptance requires default-disabled isolation, independent SWE/depth-density anti-alias checks, direct R4G state mutation/downstream/shadow/runtime-carry evidence, no site-specific constants, no output-schema/parser/runfile/default activation, and full workspace gates. | hard-fail | INV-SNOWFREEZE-050, INV-SNOWFREEZE-055, INV-SNOWFREEZE-056, INV-SNOWFREEZE-058, INV-SNOWFREEZE-059, REF-SNOWFREEZE-ANDERSON1976-CANDIDATE, REF-SNOWFREEZE-SNOBAL-CANDIDATE, ADR-0026, ADR-0027 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-061 | SNOWDENSITY-08 snow/frost gate rerun: after the typed runtime opt-in exists, the next evidence package must rerun the SNOTEL snow-density rubric for the accepted `physics_bulk_density_compaction_v1` lineage and the non-SNOTEL frost-site snow-control/frost rubric before any frost-attribution work resumes. SNOTEL evidence may use the SNOWDENSITY-06B CoE-bound replay when it proves the same density update, fixed CoE SWE/liquid boundaries, no site constants, and daily SWE identity. Non-SNOTEL frost attribution may be marked unblocked only when an authorized coupled WAT/publication run has applied the opt-in density state to the same snow-depth surface used by frost and WAT `Snow-Depth`; default-path WAT failures or offline snow-only substitutions cannot clear this gate. The rerun report must publish `frost_attribution_authorized`, SNOTEL robust/density-cell deltas, non-SNOTEL snow-control status counts, whether a coupled opt-in WAT path was available, CoE boundary anti-alias evidence, and the next blocker. It must not tune coefficients, canopy, radiation, albedo, melt, frost physics, parser/runfile/CLI selectors, output schemas, or defaults. | hard-fail | INV-SNOWFREEZE-047, INV-SNOWFREEZE-048, INV-SNOWFREEZE-050, INV-SNOWFREEZE-059, INV-SNOWFREEZE-060, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-062 | SNOWDENSITY-09 diagnostic coupled WAT rerun: a non-production diagnostic path may run the direct-production executor with an explicitly ratified opt-in `snow_density_model` for the non-SNOTEL frost fixtures only when the selector is package-bound, explicit, and absent by default. The authorized selector is the environment variable `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL` with accepted values `legacy_wepp`, `physics_bulk_density_compaction_v1`, and, when governed by `INV-SNOWFREEZE-068`, `physics_bulk_spring_densification_v1`; any other non-empty value must fail closed. This selector is diagnostic harness plumbing, not a parser/runfile/user CLI activation surface, and it must not affect compatibility runtime, default-candidate rollback, output schema, coefficients, canopy, radiation, albedo, melt, density constants, or frost physics. Acceptance requires paired default-vs-opt-in WAT reports, trace evidence that the direct-production snow partition selected the opt-in model, proof that WAT `Snow-Depth` remains sourced from `snow.runtime_depth_m`, no WAT rewriting/offline snow-only substitution, no site-specific constants, and a decision report that keeps frost attribution blocked unless the coupled opt-in snow-control gate passes and the SNOWDENSITY-08 SNOTEL density gate remains cleared. The coupled opt-in snow-control gate is evaluated only over fixtures with observed snow-depth rows; fixtures without observed snow-depth rows remain diagnostic frost/isotherm evidence and must be reported separately as out-of-gate, not counted as pass, fail, or blocker for the snow-depth control gate. | hard-fail | INV-SNOWFREEZE-047, INV-SNOWFREEZE-048, INV-SNOWFREEZE-050, INV-SNOWFREEZE-060, INV-SNOWFREEZE-061, INV-SNOWFREEZE-068, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-063 | SNOWDENSITY-10.3.1a per-day `cancov` direct-runtime bridge: snowbench and CoE melt diagnostic replay may no longer use a repeated scalar `cancov` runtime-surface value as canopy authority when direct-production day inputs can provide a per-day canopy trajectory. The daily series must be generated by the same direct production growth-state path that computes `growth_state_for_publication.canopy_cover_fraction` before snow liquid partition, must carry one finite `[0, 1]` value per simulation day, and must be date-aligned with the forcing rows consumed by replay. The legacy scalar `primary_canopy_cover_fraction` may remain as a backward-compatible summary/initial-state diagnostic, but it is not low-canopy or seasonal-canopy evidence once `cancov_daily_series` is available. CoE replay must fail closed on missing, duplicated, non-finite, out-of-range, or length-mismatched daily canopy rows. This amendment does not authorize canopy tuning, melt coefficient changes, density changes, radiation/albedo changes, default activation, parser/runfile/user CLI selectors, production output schema changes, fixture edits, or compatibility-runtime deletion. | hard-fail | INV-SNOWFREEZE-050, INV-SNOWFREEZE-056, INV-SNOWFREEZE-057, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-064 | SNOWDENSITY-10.3.5a openWEPP meteorology candidate crate: `crates/openwepp-meteorology` may implement pure psychrometric primitives and the Harder-Pomeroy hydrometeor-temperature precipitation-phase candidate only as a reusable library. The crate must expose typed Celsius, unit-interval humidity, vapor-pressure/density, latent-heat, air-diffusivity, air-conductivity, hydrometeor-temperature, and rain/snow fraction APIs with finite-domain guards and typed errors. Harder-Pomeroy rain and snow fractions must remain bounded in `[0, 1]`, close to one within roundoff, and be monotonic with hydrometeor temperature for each ratified coefficient set. This amendment does not authorize replacement of production `RST`, changes to `stmtim`/daily-hourly WEPP partition behavior, parser/runfile/user selectors, output schema, fixture edits, default activation, or compatibility-runtime changes. Production crates must not depend on or call `openwepp-meteorology` until a later contract amendment and work package explicitly authorizes an adjudication or activation seam. | hard-fail | REF-SNOWFREEZE-HARDER-POMEROY-2013, INV-SNOWFREEZE-005, INV-SNOWFREEZE-050, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-065 | SNOWDENSITY-10.3.5b opt-in hourly Harder-Pomeroy partition: production direct-runtime hourly winter partition may depend on `openwepp-meteorology` only behind an explicit package-bound selector with accepted values `legacy_rst` and `harder_pomeroy_hourly`; `legacy_rst` remains the default and must preserve the existing `stmtim` `RST` threshold branch exactly. The opt-in path must compute hourly Harder-Pomeroy fractions from the synthesized hourly air temperature and finite unit-interval relative humidity derived from the daily dew point or supplied RH, using the hourly coefficient set. Dewpoint-derived supersaturation may be normalized only to exact saturation (`RH=1.0`) with evidence; non-finite, negative, zero-saturation, or otherwise out-of-domain humidity inputs fail closed. For each active precipitation hour, `hrrain + hrsnow / 10` must reconstruct the active hourly precipitation depth within roundoff, preserving the legacy snowfall-depth scale while allowing fractional rain/snow coexistence only in the opt-in path. The real direct snow consumer must receive the selected hourly partition; producer-only symbol evidence is insufficient. This amendment does not authorize default activation, parser/runfile/user CLI selectors, fixture edits, public WAT/HBP/PASS schema changes, snow density/melt/canopy/radiation/frost changes, compatibility-runtime behavior changes, or site-calibrated phase coefficients. Jennings et al. observed-phase validation is required as adjudication evidence, not as a tuning target. | hard-fail | REF-SNOWFREEZE-HARDER-POMEROY-2013, INV-SNOWFREEZE-005, INV-SNOWFREEZE-050, INV-SNOWFREEZE-064, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-066 | SNOWDENSITY-10.3.7 opt-in winter-thaw melt-response correction: production direct-runtime CoE melt may add `snow_melt_model = coe_winter_thaw_state_loss_v1` only as an explicit opt-in candidate with `legacy_coe` remaining the default, comparator surface, and rollback path. The candidate must preserve CoE raw-melt terms (`amelt`, `bmelt`, `cmelt`, `dmelt`), signed raw melt, corrected negative-melt redistribution, radiation source, canopy attenuation, phase partition, density constants, rain retention/release, frost physics, and public output schemas. Its only authorized algorithmic delta is the positive-thaw application branch when the legacy density gate would keep `wmelt > 0` in-pack below `350 kg m^-3`: the candidate may route that positive `wmelt` to snowpack SWE state loss, routed melt, and downstream liquid forcing while preserving proportional depth/SWE loss and bounded non-negative snow state. `coe_shortwave_albedo_v1` behavior and albedo fail-closed rules are unchanged; `coe_winter_thaw_state_loss_v1` must not require or consume albedo state. A package-bound diagnostic direct-production selector, `OPENWEPP_SNOWDENSITY1037_MELT_MODEL`, may accept only `legacy_coe` or `coe_winter_thaw_state_loss_v1`; absent/empty values must preserve `legacy_coe`, and unknown values must fail closed. Acceptance requires default-identity tests, independent reconstruction of raw melt, redistributed melt, routed melt, retained/released rain, SWE loss, depth loss, final snow-state closure, and downstream WAT/liquid-routing evidence from produced artifacts. The candidate is only an opt-in improvement, not a fix, unless it both reduces paired Sleepers/Harvard event-window under-ablation and aggregate depth-loss deficit and does not worsen the coupled direct-production WAT snow-control gate relative to `legacy_coe`; if conservation or coupled WAT evidence is missing, failing, or worse, the package must close with `HOLD` rather than activate or widen scope. | hard-fail | INV-SNOWFREEZE-002, INV-SNOWFREEZE-015, INV-SNOWFREEZE-050, INV-SNOWFREEZE-055, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-067 | SNOWDENSITY-10.3.8 opt-in liquid holding-capacity drainage correction: production direct-runtime CoE melt may add `snow_melt_model = coe_liquid_holding_capacity_v1` only as an explicit opt-in candidate with `legacy_coe` remaining the default, comparator surface, and rollback path. The candidate must preserve CoE raw-melt terms (`amelt`, `bmelt`, `cmelt`, `dmelt`), signed raw melt, corrected negative-melt redistribution, radiation source, canopy attenuation, precipitation phase partition, density constants, frost physics, public output schemas, and `coe_shortwave_albedo_v1` albedo behavior. Its only authorized algorithmic delta is the positive-liquid application branch when the legacy density gate would retain positive melt or rain as density-only compaction below `350 kg m^-3`: the candidate may retain liquid only up to a non-fitted holding capacity derived from in-repo authority (`max_liquid_water_volume_fraction = 0.01`) and must route excess liquid as snowpack SWE state loss/released rain into downstream liquid forcing. Retained liquid must be a bounded, non-negative persistent snow-lane store, release must never exceed incoming liquid plus previously retained liquid above current capacity, and final snow SWE/depth/density/liquid-state closure must be independently reconstructable from produced operands. A package-bound diagnostic direct-production selector, `OPENWEPP_SNOWDENSITY1038_MELT_MODEL`, may accept only `legacy_coe` or `coe_liquid_holding_capacity_v1`; absent/empty values must preserve `legacy_coe`, and unknown values must fail closed. Acceptance requires default-identity tests, capacity-bound and drain-down tests, independent reconstruction of raw melt, redistributed melt, routed melt, retained/released rain, retained/released snow liquid, SWE loss, depth loss, final snow-state closure, and downstream WAT/liquid-routing evidence from produced artifacts. The candidate is only an opt-in improvement unless it both reduces paired Sleepers/Harvard event-window under-ablation and aggregate depth-loss deficit and does not worsen the coupled direct-production WAT snow-control gate relative to `legacy_coe`; if conservation/routing, persistent-store, or coupled WAT evidence is missing, failing, or worse, the package must close with `HOLD`. | hard-fail | INV-SNOWFREEZE-002, INV-SNOWFREEZE-015, INV-SNOWFREEZE-050, INV-SNOWFREEZE-055, REF-SNOWFREEZE-MARKS1998-LIQUID-CAPACITY, REF-SNOWFREEZE-ANDERSON1976-LIQUID, REF-SNOWFREEZE-SNOW17-PLWHC, REF-SNOWFREEZE-SNOBAL-LIQUID-CAPACITY, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-068 | SNOWDENSITY-10.3.11 opt-in spring compaction/densification candidate: production direct-runtime density may add `snow_density_model = physics_bulk_spring_densification_v1` only as an explicit opt-in candidate with `legacy_wepp` remaining the default, compatibility/comparator surface, and rollback path. The candidate must preserve the SNOWDENSITY-07 CoE SWE/liquid boundary split and may mutate only `snow.runtime_depth_m` and `snow.runtime_density_kg_m3`; `snow.runtime_swe`, signed `S`, raw melt, redistributed melt, routed `wmelt`, post-winter rain, snowpack SWE loss, retained/released liquid, albedo state, phase partition, canopy, radiation, rain heat, frost, public output schemas, and CoE boundary carry remain governed by the selected CoE melt/liquid boundary. Its only authorized algorithmic delta from `physics_bulk_density_compaction_v1` is wet-snow compaction realization: when the selected CoE boundary supplies positive same-day liquid for density compaction, the candidate must apply the same total liquid once to the Anderson/SNOBAL liquid-compaction term and may let wet conditions accelerate the daily time-compaction substeps. It must not change Anderson/SNOBAL dry/wet compaction multipliers, fresh-snow-density constants, melt/liquid constants, or the current `522 kg m^-3` density cap. It must not consume observed snow depth, observed density, fixture identity, site metadata, residual row class, or tolerance when computing runtime snow state. Acceptance requires default identity, fail-closed selector handling, SWE identity with the CoE runtime boundary, final density `<= 522 kg m^-3`, no site constants, no observed-depth fitting, trace evidence naming `physics_bulk_spring_densification_v1`, and a coupled direct-production WAT comparison against the `coe_liquid_holding_capacity_v1` plus `physics_bulk_density_compaction_v1` baseline. The candidate is only an opt-in improvement unless it reduces paired March/April compaction-feasible failures and does not worsen any observed-snow-depth paired surface or under-persistence guardrail; if the coupled WAT gate is missing, failing, or worse, the package must close with `HOLD` or non-promotion and keep frost attribution blocked. | hard-fail | INV-SNOWFREEZE-003, INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, INV-SNOWFREEZE-060, INV-SNOWFREEZE-062, INV-SNOWFREEZE-067, REF-SNOWFREEZE-CH3-SNOWDENS-LIM, REF-SNOWFREEZE-ANDERSON1976-CANDIDATE, REF-SNOWFREEZE-SNOBAL-CANDIDATE, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-069 | SNOWDENSITY-10.3.12 combined opt-in bundle activation adjudication: `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1` may be evaluated as a named bundle only through explicit package-bound diagnostic selectors, with `legacy_coe` and `legacy_wepp` remaining the default, comparator, and rollback members until a later activation package changes them. This bundle does not add a new formula, selector, coefficient, output schema, parser/runfile/user surface, fixture input, or compatibility-runtime path; it composes the already-ratified opt-in liquid-capacity boundary from `INV-SNOWFREEZE-067` with the already-ratified density compaction boundary from `INV-SNOWFREEZE-060`/`INV-SNOWFREEZE-062`. Activation Policy B supersedes any zero-paired-snow-failure activation rule: default activation eligibility requires real direct-production WAT evidence proving both selected members reached the direct snow partition, strictly better observed snow-depth performance than the current default over gate-eligible paired-snow surfaces, comparison against default, holding-capacity-only, and 10.3.11 spring-densification evidence, residual classification for any remaining failures, and a workspace-suite no-regression gate under the bundle selectors plus composite snow-state conservation closure. Downstream snow-affected output deltas are conserved-by-construction and not separately diffed by Policy B; runoff, erosion, water-balance, and watershed outputs may change when the improved conserved snow/liquid input changes. Missing or failed workspace-suite/conservation evidence must close `HOLD-OPT-IN-BUNDLE` with the missing scope named. Remaining paired observed snow-depth failures do not by themselves prohibit activation under Policy B, but they keep frost attribution separately blocked until snow control is good enough to isolate frost residuals. Observation-blocked surfaces remain diagnostic-only and must not be counted as pass/fail/blocker inputs for the snow-control gate. | hard-fail | INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, INV-SNOWFREEZE-060, INV-SNOWFREEZE-062, INV-SNOWFREEZE-067, INV-SNOWFREEZE-068, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-070 | SNOWDENSITY-10.3.13 residual-tail and Policy-B diagnostic: after the combined bundle adjudication, diagnostic tooling may consume committed real direct-production WAT reports and paired observations to classify date-level residual transitions across current default, `coe_liquid_holding_capacity_v1`, `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`, and the rejected `physics_bulk_spring_densification_v1` candidate. The diagnostic must report whether bundle under-persistence rows were newly introduced by the density arm, persisted from earlier arms, or came from opposite-sign over-persistence/pass states; must classify remaining over-persistence into cap-limited, patchy, compaction-feasible, or unresolved rows using the active `522 kg m^-3` cap; and must define the missing Policy-B workspace-suite/conservation evidence separately from frost-attribution snow-control residuals. This diagnostic is evidence only: it does not authorize default activation, cap changes, coefficient changes, new compaction-rate variants, open-surface ablation, parser/runfile/user selector changes, fixture edits, output-schema changes, frost attribution, Qwet/frzftp, or compatibility-runtime changes. If the SNOBAL `550 kg m^-3` cap re-anchor is pursued, it requires a separate contract-first package and Policy-B evidence; it must not be smuggled into this diagnostic. | hard-fail | INV-SNOWFREEZE-003, INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, INV-SNOWFREEZE-060, INV-SNOWFREEZE-067, INV-SNOWFREEZE-068, INV-SNOWFREEZE-069, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-071 | SNOWDENSITY-10.3.14 Policy-B no-regression and cap-authority diagnostic: before default activation of the current best bundle, package evidence must run the workspace-suite no-regression gate under the existing package-bound selectors `OPENWEPP_SNOWDENSITY1038_MELT_MODEL=coe_liquid_holding_capacity_v1` and `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_density_compaction_v1`, and must independently verify the composed direct trace state remains density-cap bounded and closes `runtime_swe = runtime_depth_m * runtime_density_kg_m3 / 1000` within tolerance. The gate means the existing suite passes under the bundle selectors plus composite snow-state conservation closure; downstream snow-affected output deltas are conserved-by-construction and not separately diffed by this diagnostic. This amendment does not itself change the default model or activate parser/runfile/user configuration. The active runtime density cap remains `522 kg m^-3` under `INV-SNOWFREEZE-003`; `550 kg m^-3` SNOBAL cap evidence may be evaluated only as a same-SWE, cap-pinned projection unless a later contract amendment authorizes a real dynamic cap implementation and rerun. The diagnostic must report whether the active-cap bundle is ready for a separate default-activation package, and must keep any cap re-anchor, shallow-pack compaction guard, open-surface ablation, or other residual physics as follow-up unless dynamic evidence exists. It must not change production physics, density caps, output schemas, fixture inputs, compatibility runtime, Qwet/frzftp, frost attribution, or add new selectors. | hard-fail | INV-SNOWFREEZE-003, INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, INV-SNOWFREEZE-060, INV-SNOWFREEZE-067, INV-SNOWFREEZE-069, INV-SNOWFREEZE-070, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-072 | SNOWDENSITY-10.3.15 default activation under active cap: the direct-production default snow bundle is `snow_melt_model = coe_liquid_holding_capacity_v1` plus `snow_density_model = physics_bulk_density_compaction_v1` when the package-bound selector environment variables are absent. This supersedes only the older default-selection posture of `INV-SNOWFREEZE-060` and `INV-SNOWFREEZE-067`; their conservation, rollback, boundary-split, no-site-tuning, and fail-closed requirements remain binding. `OPENWEPP_SNOWDENSITY1038_MELT_MODEL=legacy_coe` and `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=legacy_wepp` remain explicit rollback/test selectors, not parser/runfile/user CLI controls. Empty selector values select the activated defaults; unknown or unsupported values must fail closed. The rejected `physics_bulk_spring_densification_v1`, `coe_winter_thaw_state_loss_v1`, and `coe_shortwave_albedo_v1` candidates are not accepted by the active default selector path unless a later package re-ratifies them. Acceptance requires real downstream direct-production evidence that no-env WAT/trace rows select the activated bundle, explicit rollback evidence, unchanged output schema and user-facing configuration surfaces, active `522 kg m^-3` cap preservation, workspace gates on the no-env default path, and carry-forward release notes that downstream snow-affected output deltas are expected conserved consequences, `498/1415` paired snow-depth failures remain, and frost attribution remains blocked by `SNOW-CONTROL-RESIDUALS-REMAIN`. | hard-fail | INV-SNOWFREEZE-003, INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, INV-SNOWFREEZE-060, INV-SNOWFREEZE-067, INV-SNOWFREEZE-069, INV-SNOWFREEZE-071, REF-SNOWFREEZE-SNOWDENSITY1015, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-073 | SNOWDENSITY-10.3.16 open-surface ablation Stage A: production direct-runtime CoE melt may add `snow_melt_model = coe_open_sublimation_stage_a_v1` only as an explicit package-bound opt-in candidate with the activated default `coe_liquid_holding_capacity_v1` and rollback `legacy_coe` retained. Its only authorized algorithmic delta from `coe_liquid_holding_capacity_v1` is a finite, non-negative, bounded turbulent latent mass-loss sink `snow_sublimation` that subtracts snowpack SWE as vapor from the runtime snow state. Sublimated mass must never be routed as melt, rain, runoff, infiltration, liquid holding-capacity release, or density-only compaction; it must be tracked in internal trace/conservation ledgers and included in snow-state closure as vapor export. The Stage A candidate must preserve CoE raw melt terms, signed melt redistribution, liquid holding-capacity semantics, phase partition, canopy/radiation/longwave/albedo/frost behavior, density cap, density model default, output schemas, fixture inputs, compatibility runtime, parser/runfile/user surfaces, and Qwet/frzftp posture. Acceptance requires real coupled direct-production WAT/trace evidence that the opt-in candidate reached the snow partition, reduces the open-surface cap-limited over-persistence tail, does not worsen under-persistence, keeps sublimation magnitude in a literature-defensible range without fixture tuning, and closes whole-model snow-state conservation. If any gate is missing, worse, or fails, the package must close `HOLD` or non-promotion and must not activate the candidate. | hard-fail | INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, INV-SNOWFREEZE-067, INV-SNOWFREEZE-072, REF-SNOWFREEZE-MARKS1998-TURBULENT, REF-SNOWFREEZE-MARKS1999-SUBLIMATION, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-074 | SNOWDENSITY-10.3.17 shallow-pack compaction guard: production direct-runtime density may add `snow_density_model = physics_bulk_shallow_guard_v1` only as an explicit package-bound opt-in candidate with the activated default `physics_bulk_density_compaction_v1` and rollback `legacy_wepp` retained. Its only authorized algorithmic delta from `physics_bulk_density_compaction_v1` is reduced density-compaction aggressiveness when the pre-compaction physical snow depth is below the authority-derived `snow_shallow_compaction_guard_depth_threshold = 0.25 m`. The candidate must preserve the Anderson/SNOBAL density constants, fresh-snow-density equation, dry/wet compaction multipliers, wet liquid-compaction formula, daily substep count, active `522 kg m^-3` cap, CoE SWE/liquid boundary split, `snow.runtime_swe`, signed `S`, raw melt, redistributed melt, routed `wmelt`, post-winter rain, snowpack SWE loss, retained/released liquid, sublimation, albedo state, phase partition, canopy, radiation, rain heat, frost, public output schemas, fixture inputs, parser/runfile/user surfaces, compatibility runtime, and Qwet/frzftp posture. It must not consume observed snow depth, observed density, fixture identity, site metadata, residual row class, or snow-control tolerance. Acceptance requires real coupled direct-production WAT/trace evidence that the opt-in candidate reached the snow partition, cuts the density-arm-induced under-persistence tail with explicit `harvard_hardwood` reporting, does not worsen over-persistence, keeps the shallow threshold authority-derived rather than fixture-tuned, preserves SWE/depth-density conservation and the density cap, and leaves protected boundaries unchanged. If any gate is missing, worse, or fails, the package must close `HOLD` or non-promotion and must not activate the candidate. | hard-fail | INV-SNOWFREEZE-003, INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, INV-SNOWFREEZE-060, INV-SNOWFREEZE-067, INV-SNOWFREEZE-070, INV-SNOWFREEZE-072, REF-SNOWFREEZE-SNOWDENSITY1017, REF-SNOWFREEZE-MARKS-SHALLOW-LAYER, REF-SNOWFREEZE-ANDERSON1976-CANDIDATE, REF-SNOWFREEZE-SNOBAL-CANDIDATE, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-075 | SNOWDENSITY-10.3.19 Harder-Pomeroy direct-production phase default: the direct-production no-env snow bundle is `snow_melt_model = coe_liquid_holding_capacity_v1`, `snow_density_model = physics_bulk_density_compaction_v1`, and `snow_phase_partition_model = harder_pomeroy_hourly`. This supersedes only the older direct-production phase-default posture in `INV-SNOWFREEZE-065`; its finite humidity guards, active-hour precipitation reconstruction, clean-room Harder-Pomeroy authority, no-site-tuning rule, and parser/runfile/user CLI prohibitions remain binding. `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=legacy_rst` remains an explicit rollback/test selector, and explicit `harder_pomeroy_hourly`, absent, or empty selector values select the activated phase default. Unknown or unsupported selector values must fail closed. Activation Policy B for this phase-default change is the cross-SNOTEL `INV-SNOWFREEZE-050` forcing-robust rubric as the primary gate: a real coupled direct-production WAT rerun must show the new no-env default is at least as good as the prior activated bundle with `legacy_rst` phase on robust fail count and robust ordinal score. The 10.3.18 baseline profile is `17` robust fails and `172` robust score for the activated bundle, with the Harder-Pomeroy profile at `15` robust fails and `179` robust score; 10.3.19 must reconfirm this relationship after the default change. Acceptance also requires workspace-suite no-regression under the new no-env default, partition mass conservation (`hrrain + hrsnow / 10` reconstructs active hourly precipitation depth), unchanged public output schemas, fixture inputs, density cap, frost behavior, parser/runfile/user surfaces, and release notes carrying forward that humid-New-England depth regression is a non-representative roadmap item while the cross-SNOTEL density bias rise to about `+23.6 kg m^-3` is tracked separately. The `.run` disable option is not authorized by this amendment. If the cross-SNOTEL rubric or conservation gate fails, the package must close `HOLD` and must not activate the phase default. | hard-fail | INV-SNOWFREEZE-050, INV-SNOWFREEZE-065, INV-SNOWFREEZE-069, INV-SNOWFREEZE-072, REF-SNOWFREEZE-HARDER-POMEROY-2013, REF-SNOWFREEZE-SNOWDENSITY1019, ADR-0017 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-076 | SNOWDENSITY-10.3.20 sublimation diagnosis, composition, and Stage B unlock: production direct-runtime CoE melt may add `snow_melt_model = coe_open_sublimation_stage_b_v1` only as an explicit package-bound opt-in candidate with the current no-env default (`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1 + harder_pomeroy_hourly`) and rollback selectors retained. The package must diagnose why `coe_open_sublimation_stage_a_v1` worsens the cross-SNOTEL forcing-robust rubric, decomposing degradation by site, signature, residual component, and sublimation magnitude; score the partition+sublimation composition on the same rubric; and score Stage B against the current default. Stage B's only authorized algorithmic delta from Stage A is replacing the fixed freezing-point sublimation surface with a SNOBAL/Marks active surface-layer temperature/cold-content gate: surface vapor pressure is evaluated at a bounded surface-layer temperature, and the active layer depth is derived from the `0.25 m` Marks/SNOBAL surface-layer ceiling and current physical snow depth, not fitted to fixtures. Sublimated mass remains finite, non-negative, bounded by available snowpack SWE, tracked as vapor, and excluded from routed liquid. Stage B must preserve CoE raw melt terms, signed melt redistribution, liquid holding-capacity semantics, phase partition default, canopy/radiation/longwave/albedo/frost behavior, density cap, density model default, output schemas, fixture inputs, compatibility runtime, parser/runfile/user surfaces, `.run` controls, and Qwet/frzftp posture. Promotion is allowed only if a real cross-SNOTEL direct-production WAT/trace run shows the candidate beats the current default on the `INV-SNOWFREEZE-050` forcing-robust primary rubric (robust fail count no worse and robust ordinal score higher), satisfies the bidirectional guardrail, and closes sublimation and phase-partition conservation. If composition or Stage B does not beat the current default, it must remain opt-in/non-promoted; if conservation or protected boundaries fail, close `HOLD`. | hard-fail | INV-SNOWFREEZE-050, INV-SNOWFREEZE-073, INV-SNOWFREEZE-075, REF-SNOWFREEZE-SNOWDENSITY1020, REF-SNOWFREEZE-MARKS1999-SUBLIMATION, REF-SNOWFREEZE-MARKS-SHALLOW-LAYER, REF-SNOWFREEZE-LIBSNOBAL-CC0, ADR-0017, ADR-0028 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-077 | SNOWDENSITY-10.3.22 comprehensive climate-class density specialization candidate: production direct-runtime density may add `snow_density_model = physics_bulk_climate_class_density_v1` only as an explicit package-bound opt-in candidate with the current no-env default (`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1 + harder_pomeroy_hourly`) and rollback selectors retained. The candidate must represent all six Sturm 1995 class labels (`tundra`, `taiga`, `alpine`, `maritime`, `prairie`, `ephemeral`) and must assign class from the run's own wind, precipitation, and air-temperature climate under Sturm 1995 decision-tree authority; geographic lookup, site identity, observed residuals, SNOTEL fixture names, and NSIDC raster lookup are invalid runtime assignment inputs. The 1995 tree thresholds are `Tc=10 degC`, `CDM < 30 degC-month -> ephemeral`, `30 <= CDM < 125 degC-month -> high-temperature seasonal`, `CDM >= 125 degC-month -> low-temperature seasonal`, and `SPR >= 2 mm d^-1 -> high precipitation`; actual wind forcing is classified as low only at `<=0.5 m s^-1`, high only at `>=2.0 m s^-1`, and must fail closed between those values because the source brackets but does not choose a single wind-speed cutoff. Rare deep-tundra/deep-taiga tree branches must fail closed unless later authority ratifies their reduction to a six-class label. NSIDC-0768 and Sturm/Liston 2021 may be used only as independent class-system cross-checks; 2021's `61 degC-month`, `4 mm d^-1`, Boreal Forest, and Montane Forest updates must not be silently substituted for the 1995 thresholds/names paired with Sturm 2010. Class-specific density parameters must come from Sturm 2010 Table 4 and Equation 6 or later explicit literature authority, translated into the Anderson/SNOBAL compaction-coefficient form when the process model can reproduce the published trajectory. If a class cannot be reproduced by coefficient translation, a raw Sturm per-class density-form fallback must be explicitly flagged in runtime diagnostics and package evidence. Ephemeral has no Sturm 2010 parameter row and must retain the existing fresh-snow/Anderson compaction behavior as an explicitly documented fallback rather than fabricated parameters. Acceptance requires a real cross-SNOTEL direct-production WAT/trace run showing the opt-in candidate beats the current default `15/179` forcing-robust profile, fixes the cluster-1 split-sign densification trajectory in both directions (less densification at humid/continental forest and more at deep mountain), creates no new bidirectional persistence tail, preserves SWE/depth-density conservation and the active `522 kg m^-3` cap, and leaves public output schemas, fixtures, defaults, frost behavior, parser/runfile/user surfaces, `.run` controls, Qwet/frzftp, compatibility runtime, canopy, radiation, melt, and phase partition unchanged. Classes absent from the corpus may be covered only by reference authority, not claimed as rubric-validated. Boundary discontinuities from discrete classes must be monitored and smoothed only within the Sturm framework if later evidence shows class flips. If authoritative class thresholds, class parameters/fallbacks, the cross-SNOTEL gate, the bidirectional flip, or conservation evidence are missing or worse, the package must close `HOLD` or non-promotion and must not activate the candidate. | hard-fail | INV-SNOWFREEZE-003, INV-SNOWFREEZE-050, INV-SNOWFREEZE-060, INV-SNOWFREEZE-075, REF-SNOWFREEZE-SNOWDENSITY1022, REF-SNOWFREEZE-STURM2010-DENSITY, REF-SNOWFREEZE-STURM1995-CLASSIFICATION, REF-SNOWFREEZE-STURM2021-CLASSIFICATION-CROSSCHECK, REF-SNOWFREEZE-NSIDC0768, ADR-0011, ADR-0025, ADR-0026, ADR-0027, ADR-0028 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-078 | Paradigm 2 Stage 1 layered snow-density candidate: production direct-runtime density may add `snow_density_model = physics_bulk_multilayer_density_v1` only as an explicit package-bound opt-in candidate with the current no-env default (`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1 + harder_pomeroy_hourly`) and `legacy_wepp` rollback retained. The candidate must store a persistent top-to-bottom `snow_layers` vector in the ADR-0026 winter column using the same variable-length-Vec precedent as frost; layer state must not be projected into fixed coarse slots. Stage 1 is density-only: it may add/split/merge layer density state and apply the existing Anderson/SNOBAL fresh-snow, destructive metamorphism, overburden compaction, and wet-compaction constants per layer, but the only authorized physics delta relative to `physics_bulk_density_compaction_v1` is replacing bulk total-pack overburden with per-layer local overburden equal to the sum of overlying layer mass. It must preserve the active `522 kg m^-3` cap, CoE SWE/liquid/melt/routed boundaries, Harder-Pomeroy phase default, canopy/radiation/albedo/frost behavior, output schemas, fixtures, parser/runfile/user selector surfaces, `.run` controls, Qwet/frzftp, compatibility runtime, and site-calibration prohibition. Layer mass and thickness aggregates must reconstruct public aggregate SWE/depth/density and whole-model snow-state conservation within tolerance; invalid or inconsistent layers fail closed rather than being silently repaired. A real cross-SNOTEL+cancov direct-production `INV-SNOWFREEZE-050` run is the primary gate: promotion requires beating the current default `15/179` forcing-robust profile, proving the split-sign densification trajectory improves in both directions (less densification at humid/continental forest and more at deep mountain), creating no new bidirectional persistence tail, closing conservation, and meeting ADR-0025 performance evidence. Missing layer-consumer proof, missing rubric evidence, worse robust guardrail results, conservation failure, cap drift, protected-boundary drift, or fixture/site fitting must close `HOLD` or non-promotion without activation. | hard-fail | INV-SNOWFREEZE-003, INV-SNOWFREEZE-050, INV-SNOWFREEZE-060, INV-SNOWFREEZE-075, INV-SNOWFREEZE-077, REF-SNOWFREEZE-PARADIGM2-STAGE1, REF-SNOWFREEZE-ANDERSON1976-CANDIDATE, REF-SNOWFREEZE-SNOBAL-CANDIDATE, ADR-0011, ADR-0025, ADR-0026, ADR-0028, ADR-0029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-079 | Paradigm 2 Stage 2 snow-to-frost insulation-profile candidate: production direct-runtime frost may add `snow_frost_insulation_model = layered_resistance_v1` only as an explicit package-bound opt-in candidate composed with `snow_density_model = physics_bulk_multilayer_density_v1`; absent or empty selector values must retain the existing bulk `snow_depth_m`/`snow_density_kg_m3` handoff. Before the frost candidate may run, real direct-production trace evidence must prove the Stage 1 layer stack develops a physically meaningful density gradient on multi-layer snowpack rows, with basal density greater than surface density. The candidate must compute snow insulation as thermal resistance over the prior-day snow layer stack, `R_snow = sum(depth_i / k_i)`, using the Sturm et al. 1997 density-to-conductivity relation already mirrored by the WEPP frost heat-path implementation (`k = 0.023 + 0.234*rho` for `rho < 0.156 g cm^-3`; otherwise `k = 0.138 - 1.01*rho + 3.233*rho^2`, multiplied by `ksnowf`, with `rho` in `g cm^-3` and `k` in `W m^-1 K^-1`). The frost solver may consume this resistance only through an equivalent bulk density whose existing `snow_depth / k(rho)` reproduces the layer resistance; that equivalent density is internal frost-insulation state, must remain finite and within the active `522 kg m^-3` snow-density cap, and must not replace public aggregate snow density. The candidate preserves ADR-0026 prior-day snow-to-frost timing decoupling and may not read same-day post-frost snow changes. It must leave CoE SWE/liquid/melt/routed boundaries, phase partition, albedo, canopy, radiation, public output schemas, fixtures, parser/runfile/user surfaces, `.run` controls, Qwet/frzftp, compatibility runtime, and default behavior unchanged. Acceptance is frost-primary: forcing-robust frost-depth observation signatures (`INV-SNOWFREEZE-050`; onset, deepening, thaw, frozen duration) must improve versus the bulk handoff, the cross-SNOTEL snow rubric must not worsen, snow/frost conservation must close, and ADR-0025 H2637 performance evidence must pass. Missing or failed gradient evidence, unsupported selector fallback, non-finite or out-of-domain resistance/effective density, worse frost fidelity, snow-rubric regression, conservation failure, performance failure, or any Stage 3 per-layer thermal/melt/liquid scope creep must close `HOLD` or non-promotion without activation. | hard-fail | INV-SNOWFREEZE-006, INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, INV-SNOWFREEZE-078, REF-SNOWFREEZE-PARADIGM2-STAGE2, ADR-0011, ADR-0025, ADR-0026, ADR-0028, ADR-0029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-080 | Paradigm 2 Stage 3 per-layer liquid-routing and meltwater-temperature candidate: production direct-runtime snow may add `snow_stage3_liquid_routing_model = layered_thermal_liquid_v1` only as an explicit package-bound opt-in diagnostic candidate composed with `snow_density_model = physics_bulk_multilayer_density_v1`; absent or empty selector values must retain the disabled path. The candidate may use Stage 0 surface-energy primitives, inter-layer conduction, per-layer cold content, the Stage 1 layer stack, and the 10.3.8 per-layer liquid holding-capacity authority to route the already-computed CoE daily melt/rain liquid through layers, refreeze liquid in cold layers, retain bounded liquid, and assign a typed intensive meltwater-flux temperature. The existing CoE melt/rain mass path remains authoritative for snow SWE, `snow_routed_melt`, downstream liquid forcing, and public water-balance surfaces; Stage 3 must not replace CoE melt with an energy-balance melt, retune melt coefficients, or alter public aggregate snow depth/density/SWE schemas. Layer temperatures must remain finite and `<= 0 degC`; meltwater flux temperature must be finite and physically bounded when routed liquid is positive; mass, liquid-storage, and energy ledgers must close from produced operands, including latent heat of refreeze and cold-content change. The HBP flux-temperature boundary is diagnostic-first: typed meltwater temperature may be carried as internal trace/state but must not activate full in-stream temperature routing or public output-schema changes. Acceptance requires mass, liquid, and energy conservation; physically reasonable meltwater temperature; forcing-robust melt-timing no-worse evidence; cross-SNOTEL snow rubric no-regression as a guardrail only; and ADR-0025 H2637 performance evidence. Default activation, parser/runfile/user selectors, `.run` controls, fixture/schema/cap/frost/default changes, full stream-temperature routing, untyped temperature surfaces, unphysical temperatures, liquid/energy non-closure, or performance failure must close `HOLD` or non-promotion without activation. | hard-fail | INV-SNOWFREEZE-050, INV-SNOWFREEZE-067, INV-SNOWFREEZE-075, INV-SNOWFREEZE-078, REF-SNOWFREEZE-PARADIGM2-STAGE3, REF-SNOWFREEZE-LIBSNOBAL-CC0, REF-SNOWFREEZE-MARKS1998-LIQUID-CAPACITY, REF-SNOWFREEZE-MARKS1998-TURBULENT, ADR-0011, ADR-0025, ADR-0026, ADR-0028, ADR-0029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-081 | Paradigm 2 Stage 3-Decouple snow-neutral water-temperature arm: production direct-runtime snow may run `snow_stage3_liquid_routing_model = layered_thermal_liquid_v1` as an explicit package-bound opt-in capability with the current no-env density default `snow_density_model = physics_bulk_density_compaction_v1`; absent or empty selector values must retain the disabled path. This amendment supersedes only the Stage 3 requirement in `INV-SNOWFREEZE-080` to compose with `physics_bulk_multilayer_density_v1` for the water-temperature arm. When Stage 3 is selected under a bulk density model, the runtime must synthesize and persist a top-to-bottom layer geometry whose aggregate SWE, depth, and density exactly reconstruct the selected bulk density outcome and whose per-layer densities all equal the selected bulk aggregate density. The decoupled arm may carry per-layer temperature, cold content, retained liquid, and refreeze; solve inter-layer thermal exchange using Stage 0 surface-energy primitives; route the already-computed CoE daily melt/rain liquid through the layer geometry; and assign a typed intensive meltwater-flux temperature. It must not invoke Stage 1 local-overburden densification, require `physics_bulk_multilayer_density_v1`, replace CoE melt/rain mass, alter public aggregate snow SWE/depth/density, alter frost handoff behavior, or claim snow-density promotion. Acceptance requires the real cross-SNOTEL/cancov forcing-robust snow rubric to exactly match the current bulk default profile (`15` robust fails / `179` score) with no worse robust cells, mass/liquid/energy conservation to close from produced operands, physically reasonable typed meltwater temperature, forcing-robust runoff/melt-timing no-worse evidence, and ADR-0025 H2637 performance evidence. Default activation, parser/runfile/user selectors, `.run` controls, fixture/schema/cap/frost/default changes, full stream-temperature routing, Stage 1 density dependence, untyped temperature surfaces, unphysical temperatures, public snow-rubric regression, liquid/energy non-closure, or performance failure must close `HOLD` without activation. | hard-fail | INV-SNOWFREEZE-050, INV-SNOWFREEZE-075, INV-SNOWFREEZE-080, REF-SNOWFREEZE-PARADIGM2-STAGE3, REF-SNOWFREEZE-PARADIGM2-STAGE3-DECOUPLE, REF-SNOWFREEZE-LIBSNOBAL-CC0, REF-SNOWFREEZE-MARKS1998-LIQUID-CAPACITY, REF-SNOWFREEZE-MARKS1998-TURBULENT, ADR-0011, ADR-0025, ADR-0026, ADR-0028, ADR-0029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-082 | Paradigm 2 multilayer promotion: `snow_stage3_liquid_routing_model = layered_thermal_liquid_v1` is a production-supported opt-in water-temperature capability when composed with the current no-env bulk snow default. Absent, empty, or `disabled` selector values must retain the bulk default and disabled Stage 3 path; unknown values must fail closed. The selector remains supported-internal only: parser/runfile/user CLI/WEPPpy wiring is deferred until the stream-temperature program has a downstream consumer. The promoted arm must publish nullable hillslope WAT parquet `MeltwaterTemperature` in `degC` from `snow_meltwater_flux_temperature` when routed meltwater temperature exists, with field metadata and unit-registry validation; null is required when the arm is disabled or no routed meltwater temperature is produced. HBP binary/watershed serialization and full in-stream routing are not authorized by this promotion. Acceptance requires a real cross-SNOTEL/cancov direct-production rerun showing the opt-in arm exactly matches the current bulk default (`15` robust fails / `179` score) with no worse robust cells, mass/liquid/energy conservation closure, physically reasonable finite meltwater temperatures `<= 0 degC`, ADR-0025 H2637 performance within budget, default-path public-schema regression proof, release notes, and rollback evidence. Default activation, Stage 1 density dependence, CoE melt/rain mass replacement, density/frost changes, public snow-rubric regression, fixture/schema changes outside the supported WAT temperature field, HBP/watershed serialization, full stream-temperature routing, untyped or unbounded temperature publication, liquid/energy non-closure, or performance failure must close `HOLD`. | hard-fail | INV-SNOWFREEZE-050, INV-SNOWFREEZE-075, INV-SNOWFREEZE-081, REF-SNOWFREEZE-PARADIGM2-PROMOTION, REF-SNOWFREEZE-PARADIGM2-STAGE3-DECOUPLE, ADR-0011, ADR-0025, ADR-0026, ADR-0028, ADR-0029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-083 | Dynamic frost residue-cover coupling: when frost thermal inputs are constructed from a production residue/decomposition path that publishes dynamic `residue_depth_m`, the frost surface heat path must consume that current dynamic depth as the residue resistance term in `residue_depth_m / residue_conductivity_w_m_k`. The t0 `frost.runtime_residue_depth_m` seed remains valid only as the initial residue-depth value and conversion provenance; it must not be silently reused for later frost days after surface-residue mass changes. Missing, negative, non-finite, or stale dynamic residue-depth payloads are typed hard failures. The amendment changes only the residue-cover boundary consumed by frost; it does not authorize snow-model changes, Qwet changes, canopy leaf-on/leaf-off changes, frost default activation, fixture fitting, public schema changes, or legacy-comparator targeting. Acceptance requires a real downstream frost trace showing `DirectFrostThermalInputs.residue_depth_m` follows current residue mass, an inert/no-senescence identity-stability check, residue mass/depth conservation, and the Sleepers A-vs-B timing re-score requested by the residue-cover work package. | hard-fail | INV-SNOWFREEZE-006, INV-SNOWFREEZE-047, INV-SNOWFREEZE-050, REF-SNOWFREEZE-LEGACY-WINTER-RESDEP, REF-SNOWFREEZE-FROST-RESIDUE-COVER-20260629, SC-RESIDUE-001#INV-RESIDUE-019, ADR-0011, ADR-0017, ADR-0028 | `[DIRECT][Ran] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-084 | Frost direct-production default activation and direct cutover: the no-env hillslope `DefaultCandidate` runtime policy must select `DirectProductionExecutor` after `INV-SNOWFREEZE-047/048/050` ratification and the `GAP-SNOWFREEZE-002` attributed/bounded disposition. Multi-OFE/Wave-2 public-output runs and legacy sidecar-discovery runs are not allowed to fall back to the compatibility runtime as a default behavior; a direct-runtime failure on those surfaces is a named blocker to fix or hold, not a compatibility escape hatch. Explicit `DirectProductionExecutor` remains available and fail-closed for callers that deliberately request direct execution. Explicit `Compatibility` selection, if still present in code during staged deletion, is a deprecated comparator/deletion seam only and must not be used to satisfy default-activation gates. Unknown or conflicting runtime selector values must fail closed. Activation is authorized by the frost Policy B analog in `INV-SNOWFREEZE-050`: bounded residuals plus full-surface no-regression and conservation evidence, not zero paired observation failures. The activation does not authorize Qwet, new frost physics, observation-fixture edits, public output-schema changes, or reinterpretation of legacy as an acceptance target. Acceptance requires current full gates (`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` or ratified nextest equivalent, `cargo deny check`), source-level anti-evasion guards when authority-suite posture changes, and runtime evidence that no-env runs use direct production without compatibility fallback. | hard-fail | INV-SNOWFREEZE-047, INV-SNOWFREEZE-048, INV-SNOWFREEZE-050, REF-SNOWFREEZE-FROST-RATIFICATION-20260629, REF-SNOWFREEZE-FROST-DIRECT-CUTOVER-CORRECTION-20260629, ADR-0011, ADR-0017 | `[DIRECT][Ran] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-085 | SNOW-SURFACE-EB-03 shared thermal and energy composition: `layered_thermal_liquid_v1` top-layer temperature and cold content are the sole candidate snow thermal provider for the internal `B/L/S/LS` campaign cells. `snow_surface_longwave_model` and `snow_sublimation_model` are orthogonal held diagnostic/reproduction selectors that default to disabled and do not alter `snow_melt_model`, phase, density, albedo, or liquid-routing selection. Longwave consumes `SC-SNOWENERGY-001` v2. Sublimation uses the same pre-exchange surface temperature to produce one bounded signed vapor exchange and derives latent heat from that exchange using temperature-appropriate latent heat of sublimation. Vapor loss mutates layer and aggregate ice storage only and never enters routed melt, retained/released liquid, or refreeze. Enabling either mechanism without Stage 3, combining the new sublimation selector with legacy Stage A/B melt variants, unavailable polar-night cloud state, invalid derived emissivity, or nonclosing vapor/latent/cold-content ledgers fails closed. The CoE melt/rain mass path remains authoritative; positive surface-energy excess is not converted to energy-balance melt. Acceptance requires real direct-production `B/L/S/LS` consumer proof, independent longwave/mass/energy reconstruction, unchanged absent-selector default behavior, and no default/user/public-schema/frost/fixture activation. EB-03 did not satisfy that acceptance: S/LS reached the absolute-zero provider bound with material SWE remaining, so the selectors are not supported runtime science and EB-04 is held. | hard-fail | INV-SNOWFREEZE-073, INV-SNOWFREEZE-076, INV-SNOWFREEZE-080, INV-SNOWFREEZE-082, SC-SNOWENERGY-001#INV-SNOWENERGY-015, SC-SNOWENERGY-001#INV-SNOWENERGY-017, REF-SNOWFREEZE-EB03, ADR-0011, ADR-0026 | `[DIRECT][Static] + [INFERENCE][Ran]` |
| INV-SNOWFREEZE-086 | SNOW-SURFACE-EB-03A active-layer thermal correction: the `layered_thermal_liquid_v1` radiating/turbulent state must be constructed over the upper `min(total snow depth, 0.25 m)` independently of snowfall-event boundaries. When lower snow exists, the active and lower control volumes retain distinct shared temperatures across substeps and exchange the same harmonic conductive flux `G_0` with equal-and-opposite energy operands inside every selected stability substep. Conductivity must use the libsnobal Yen/Anderson `KTS+efcon` formulation with elevation-derived atmospheric pressure, not the Sturm frost-insulation relation. The provider must use the fixed Marks/SNOBAL `60/10/1 kg m^-2` thresholds and `60/15/1 minute` levels, conservatively reconstruct persistent-layer mass/depth/cold-content/liquid state, and reevaluate radiation, vapor/latent exchange, and conduction from the current substep state. The CoE path remains authoritative for snow existence and melt, so the SNOBAL `1 kg m^-2` no-snow collapse is not imported and positive energy excess is not converted to melt. Temperature clamps, air-temperature replacement, cold-content taxes, fitted limiters, new user coefficients, density/phase/frost/default changes, and nonclosing projection or energy ledgers are prohibited. Acceptance requires physical real S/LS completion, published production active/lower/`G_0`/cadence operands, active/lower and whole-pack reconstruction, exact vapor/latent identity, protected default/B/L behavior, and no public activation. | hard-fail | INV-SNOWFREEZE-080, INV-SNOWFREEZE-082, INV-SNOWFREEZE-085, SC-SNOWENERGY-001#INV-SNOWENERGY-020, SC-SNOWENERGY-001#INV-SNOWENERGY-022, SC-SNOWENERGY-001#INV-SNOWENERGY-023, SC-SNOWENERGY-001#INV-SNOWENERGY-025, REF-SNOWFREEZE-EB03A, ADR-0011, ADR-0026 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-087 | SNOW-SURFACE-EB-04V density-process observability: every non-legacy direct snow-density update must produce a typed, behavior-neutral daily ledger through the real JSONL consumer. It must evaluate fresh-snow density before mixing; distinguish snow-input mass/depth, fresh mixing, wet compaction, destructive metamorphism, overburden compaction, structural boundary/merge projection, climate fallback and whether that fallback was used, internal/runtime caps, and downstream Stage-3 adjustment; and close final minus initial bulk density to their signed sum within `TOL-SNOWFREEZE-012`. For multilayer density, destructive and overburden attribution may allocate the exact uncapped bulk-density increment in proportion to their same-state uncapped local density tendencies while recording the exact capped-minus-uncapped bulk correction; this is a diagnostic contribution, not a counterfactual causal effect. Legacy and snow-free inapplicability must be explicit. Total before/after density, post-compaction layer differences as fresh density, or observation-fit residual allocation are prohibited aliases. The ledger may not change arithmetic order, equations, coefficients, selectors, defaults, caps, state mutation, public WAT/PASS/HBP schemas, or promotion status; EB-04V evidence remains diagnostic-only under ADR-0042. | hard-fail | INV-SNOWFREEZE-058, INV-SNOWFREEZE-060, INV-SNOWFREEZE-072, INV-SNOWFREEZE-078, ADR-0042 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-088 | SNOW-SURFACE-EB-04W accumulation and melt-driver observability: active direct snow coupling must preserve the SIMIMPL28 hourly phase result through the real JSONL consumer as rain depth, physical snowfall depth, snowfall SWE, rain/snow fractions, phase-model identity, and optional hydrometeor temperature. Active precipitation must satisfy `rain_m + snowfall_swe_m = active_precipitation_m` and daily `accumulation_m = sum(hourly snowfall_swe_m)` within `TOL-SNOWFREEZE-013`. Each active-pack CoE melt evaluation must publish the exact empirical `amelt`, `bmelt`, `cmelt`, and `dmelt` contributions in common `m` water-equivalent units, their uncapped sum, a separate pack-cap adjustment, and applied raw melt, with `applied = sum(components) + cap_adjustment`. These are empirical melt-depth formula contributions, not separately identifiable physical energy fluxes; in particular `bmelt` and `cmelt` may not be relabeled as pure sensible heat. The modeled wind-redistribution contribution is explicitly zero because active drifting is not implemented, but physical site redistribution remains unknown external forcing and cannot be set to zero or inferred from the residual. Diagnostics must not change arithmetic order, phase/melt/state equations, constants, selectors, defaults, caps, forcing, public schemas, calibration, or promotion; EB-04W observations/results remain diagnostic-only under ADR-0042. | hard-fail | INV-SNOWFREEZE-004, INV-SNOWFREEZE-005, INV-SNOWFREEZE-008, INV-SNOWFREEZE-015, INV-SNOWFREEZE-075, REF-SNOWFREEZE-EB04W, ADR-0042 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-089 | SNOW-SURFACE-EB-04W2B typed-snow activation and daily storage closure: production direct runtime must resolve its existing typed hourly phase provider before deciding snow inactivity whenever daily precipitation exceeds `TOL-SNOWFREEZE-014`; this availability rule does not alter the provider's phase equations or decisions. After any authorized provider has populated the typed hourly boundary, finite physical snowfall depth greater than `TOL-SNOWFREEZE-014` is sufficient to enter active snow coupling even when prior runtime SWE is zero and daily mean air temperature is nonnegative. The consumer must preserve provider identity without re-partitioning or changing Harder-Pomeroy, legacy-RST, or externally supplied phase decisions. It must independently reconstruct snowfall SWE as `sum(hourly snowfall_m * 0.1)` and require `SWE_before + typed_snowfall_SWE + rain_retained - snowpack_state_loss - sublimation - SWE_after = 0` within `TOL-SNOWFREEZE-006`; using reported `accumulation_m`, a zero-output inactive alias, or a downstream runoff residual as the independent snowfall operand is invalid. A material residual is a typed domain/closure failure, never a silent clamp. Warm all-rain, zero-pack input remains inactive after typed phase resolution; warm dry, zero-pack input may bypass the provider. | hard-fail | INV-SNOWFREEZE-004, INV-SNOWFREEZE-005, INV-SNOWFREEZE-019, INV-SNOWFREEZE-020, INV-SNOWFREEZE-075, INV-SNOWFREEZE-088, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-090 | SNOW-STAGE3-LIQUID-SIGNED-HOUR-TRACE-CLOSURE behavior-neutral observability: the real direct-production internal snow JSONL consumer must publish additive schema v4 with exact daily Stage-3 `incoming_liquid_m`, `routed_liquid_m`, signed `retained_liquid_delta_m`, `refrozen_liquid_m`, and `liquid_closure_residual_m`. The independent identity is `incoming - routed - retained_delta - refrozen = residual` within `TOL-SNOWFREEZE-015`; `retained_delta` is the Stage-3 layer liquid-store change returned by the routing solve, not the CoE retained store. The same trace must publish the exact existing signed-hour context: hourly air temperature, radiation, cloud, raw/applied/routed melt, liquid holding capacity, retained liquid before/after, released liquid, rain release, sublimation, pack depth/density before/after, and duration-weighted Stage-3 active/lower mass, depth, temperature, and cold content plus lower-volume present fraction, together with daily wind, dewpoint, and canopy. Lower-state values retain full-hour weighting and must be interpreted with the present fraction rather than mislabeled as end-of-hour state. Tests and evidence must reject omission of retained change, top-level CoE routed melt substituted for Stage-3 routed liquid, the CoE retained store substituted for Stage-3 retained change, double-counted refreeze, and acceptance of the producer residual without operand reconstruction. Publication must preserve production arithmetic, branches, selectors, defaults, state mutation, WAT/HBP/PASS schemas and values, and all pre-v4 trace field values; it cannot authorize a physics correction, calibration, promotion, fixture edit, or observation reinterpretation. | hard-fail | INV-SNOWFREEZE-080, INV-SNOWFREEZE-086, INV-SNOWFREEZE-088, INV-SNOWFREEZE-089, REF-SNOWFREEZE-STAGE3-TRACE-CLOSURE, ADR-0042 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-091 | SNOW-MASS-TRANSITION-LEDGER-PERSISTENCE behavior-neutral architecture: production snow must retain one authoritative calculation/state-mutation path and two exact linked compact ledgers. The solid-to-liquid ledger owns exact daily raw signed melt, redistributed positive melt, bounded authoritative snowpack SWE loss, released rain, and the existing routed-liquid handoff; its independent identity is `liquid_handoff - snowpack_swe_loss - rain_released = 0` within `TOL-SNOWFREEZE-016`, and raw/redistributed melt cannot replace state loss. The liquid-disposition ledger owns exact Stage-3 incoming, routed, signed retained change, refrozen, and residual operands governed by `TOL-SNOWFREEZE-015`; enabled Stage-3 incoming is the exact upstream handoff argument, not a recomputation. Production-required Stage-3 enabled state, sublimation, and typed meltwater temperature must remain in a compact outcome on every capture path. The hourly accumulation/melt and Stage-3 energy/cold-content arrays are an optional boxed verbose payload assembled only when the existing nonempty internal trace path and day/lane filters select the row. Absent, empty, disabled, or filtered-out capture must retain all physical calculations and closure guards while assembling and carrying no verbose post-solve payload. Opt-in capture must preserve schema-v4 rows, ordered keys, values, nulls, strings, and ordinary WAT/HBP/PASS outputs exactly and cannot select or change physics, arithmetic order, state, tolerances, selectors, defaults, fixtures, observations, calibration, or promotion. An undeclared or production-readable second mass state, duplicate independently mutable production ledger operands, inferred adjacent aliases, producer-only consumer proof, unauthorized external API break, or confirmed trace-disabled runtime/RSS regression above the frozen package bound is closure-blocking. The sole exception is an evaluation-only object satisfying every custody and claim guard in `INV-SNOWFREEZE-094`; that exception does not weaken the one-authoritative-production-state rule. | hard-fail | INV-SNOWFREEZE-019, INV-SNOWFREEZE-080, INV-SNOWFREEZE-088, INV-SNOWFREEZE-090, INV-SNOWFREEZE-094, REF-SNOWFREEZE-MASS-TRANSITION-LEDGER-PERSISTENCE, ADR-0042 | `[DIRECT][Static] + [INFERENCE][Ran]` |
| INV-SNOWFREEZE-092 | SNOW-WET-COMPACTION-OPERAND-AUTHORITY: each active-pack density update must supply the Anderson/SNOBAL wet-compaction operator with `snow_wet_compaction_liquid_input = sum(max(snow.hourly.melt_raw_m, 0)) + rain_retained + rain_released` exactly once. The melt term is gross positive liquid generated before in-pack retention, bounded storage loss, redistribution, or runoff; the rain term is all rain presented while snow cover was active at interval start, whether retained or released. A mixed event that first creates a pack from zero start-of-hour cover does not enter H2O compaction in that interval, matching PySnobal's timestep `snowcover` gate. Signed negative melt contributes zero. This driver is evaluated after precipitation and melt generation and before runoff disposition, matching PySnobal 0.2.3 `mass_bal`/`h2o_compact` chronology. It is not a conserved store or export: `snowpack_state_loss + routed_melt`, routed liquid alone, state loss plus rain, raw daily rain, retained-liquid store level/change, and any sum that counts state loss or released rain twice are prohibited aliases. The exact driver must reach both bulk and multilayer density paths, remain finite and nonnegative, and reconstruct from independent hourly generated-melt and rain operands within `TOL-SNOWFREEZE-017`. For wet-liquid operand meaning, this invariant supersedes the under-specified same-day-liquid wording in `INV-SNOWFREEZE-060`, `INV-SNOWFREEZE-068`, and `INV-SNOWFREEZE-078`; their model, boundary, and promotion constraints otherwise remain binding. This correction by itself must preserve the currently selected melt generator, snow SWE, signed `S`, retained/released liquid, routed handoff, mass-transition ledgers, Stage-3 mass closure, phase, energy, canopy, radiation, frost, selectors/defaults, density constants/cap, and public schemas. After an atomic `INV-SNOWFREEZE-093` cutover, the same driver must consume exact Stage 3 generated melt at the identical pre-retention chronology point without changing its anti-alias rules. | hard-fail | INV-SNOWFREEZE-058, INV-SNOWFREEZE-060, INV-SNOWFREEZE-067, INV-SNOWFREEZE-068, INV-SNOWFREEZE-078, INV-SNOWFREEZE-087, INV-SNOWFREEZE-088, INV-SNOWFREEZE-089, INV-SNOWFREEZE-091, REF-SNOWFREEZE-SNOBAL-CANDIDATE, REF-SNOWFREEZE-WET-COMPACTION-OPERAND-CLOSURE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-093 | SNOW-COE-STAGE3-MELT-OWNER-AUTHORITY-RECONCILIATION: Stage 3 is the sole admitted future melt owner. In each resolved substep, complete net radiation, sensible heat, latent heat, ground/interlayer conduction, and precipitation-advected heat satisfy cold content first; bounded positive excess converts available ice to liquid with the fixed latent heat of fusion. The already evaluated bounded sublimation demand is reserved from post-precipitation ice before melt availability is calculated, so melt and later vapor mutation cannot overdraw the same solid mass; later deposition cannot enlarge melt availability. The exact solid debit and liquid credit enter Stage 3 refreeze, retention, and routing in that same substep before state repartition. Energy, solid-to-liquid, and liquid-disposition ledgers must reconstruct independently, and CoE plus Stage 3 may never generate melt simultaneously. Positive energy left when the available-ice bound saturates has no admitted recipient or next-state chronology and blocks cutover alongside residual-snow phase disposition; it may not be discarded or carried by proxy. The post-2007 CoE `A/B/C/D`, `C_canopy`, daily midpoint gate, embedded albedo, and rain-heat formulation failed the frozen specific-validation and enforceable-envelope predicates; it remains only unchanged compatibility runtime behavior during `IMPLEMENTATION_HOLD`, not target authority. This invariant prospectively supersedes every earlier invariant, addendum, boundary row, and package constraint only where it assigns lasting target melt ownership to CoE, including the melt-owner clauses of `INV-SNOWFREEZE-080`, `INV-SNOWFREEZE-085`, and `INV-SNOWFREEZE-086`; their current-runtime descriptions and all other guards remain binding. No cutover is authorized until complete flux wiring, exact `m_s <= 1 kg m^-2` residual-snow phase disposition, terminal-energy disposition, selectors/defaults/rollback, real-consumer proof, and linked closure gates pass atomically. | hard-fail | INV-SNOWFREEZE-080, INV-SNOWFREEZE-085, INV-SNOWFREEZE-086, INV-SNOWFREEZE-091, SC-SNOWENERGY-001#INV-SNOWENERGY-029, SC-SNOWENERGY-001#INV-SNOWENERGY-030, REF-SNOWFREEZE-21N-MELT-OWNER, ADR-0011, ADR-0042 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-094 | SNOW-STAGE3-EVALUATION-SHADOW-AUTHORITY: production retains exactly one snow-mass/state authority and the linked ledgers of `INV-SNOWFREEZE-091`; an evaluation shadow is admitted only when a typed, default-off request selects exactly one operator and a mandatory non-authoritative tag names its source snapshot, support/interval, cadence, carrier, coverage, unresolved boundaries, and claim class. `same_state_paired_carrier_v1` evaluates exactly two stable named/versioned carrier arms against one identical immutable authoritative snapshot without state evolution. One pairing ID binds identical forcing, geometry including `z_T/z_q/z_u/z_0,aero`, cadence, coverage, and every non-formulation operand; each arm declares component IDs and explicit not-applicable components and independently reconstructs its component total. It admits carrier/component comparison only. `sequential_resolved_shadow_v1` clones one named snapshot and may evolve that clone only inside one declared continuous interval and the resolved Stage 3 domain. A physical within-interval chronology claim additionally requires every state-changing forcing/process on that interval, same-substep phase generation and liquid disposition under `SC-SNOWENERGY-001#INV-SNOWENERGY-030`, and independent initial-plus-flux-to-final mass, enthalpy, and liquid ledger closure; otherwise the result is only a bounded response experiment. Absent later authority, the clone terminates at the operator boundary and cannot seed another shadow or production interval. The evaluator has exclusive custody; the clone and its ledgers must not alias, mutate, replace, route into, publish as, calibrate, select, or otherwise reach production state, physics, defaults, fixtures, observations, public outputs, or downstream consumers. Untagged state, ambiguous operator/support, production consumer reachability, cross-boundary carry, incomplete chronology/closure, or evidence that exceeds the admitted operator claim is a governance `HOLD`; runtime errors in an enabled future evaluator must fail the evaluation request without changing authoritative execution. Daily or independently reinitialized shadows cannot support accumulation-season, peak-SWE, peak-date, meltout, restart, reappearance, terminal/post-terminal, efficacy, promotion, or cutover claims. Persistence and terminal/receiving-surface authority require separate amendments; this invariant by itself authorizes no runtime implementation or CoE ownership change. | evaluation hard-fail + governance hold | INV-SNOWFREEZE-091, INV-SNOWFREEZE-093, SC-SNOWENERGY-001#INV-SNOWENERGY-029, SC-SNOWENERGY-001#INV-SNOWENERGY-030, REF-SNOWFREEZE-STAGE3-EVALUATION-SHADOW-AUTHORITY, ADR-0042 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-095 | SNOW-STAGE3-SHADOW-SOLVER-EXTRACTION-AND-OBSERVABILITY realization: the two `INV-SNOWFREEZE-094` operators may execute only through an absent-by-default typed request and complete typed tag. Disabled, absent, filtered-out, invalid, or conflicting requests allocate/evolve no evaluation clone or verbose evaluation payload and preserve exact schema-v4 rows plus authoritative calculations, arithmetic order, state, ledgers, routing, errors, defaults, WAT/HBP/PASS outputs, and production performance posture. Enabled evaluation emits internal schema v5 only. `same_state_paired_carrier_v1` uses the immutable initial post-CoE daily snapshot, pairing ID `stage3_carrier_pair_v1`, exactly arms `stage3_surface_energy_v1` and `stage3_complete_carrier_v1`, identical forcing and CLIGEN `z_T/z_q/z_u/z_0,aero` geometry, exact support/cadence, and deterministic stable source, forcing/geometry, and combined non-formulation fingerprints; each arm exposes explicit component applicability and independently reconstructs its total without evolving the snapshot. `sequential_resolved_shadow_v1` may retain the current within-call clone evolution but must label its claim `bounded_response_experiment`, terminate at the call boundary, and publish evaluated versus requested seconds; it may not claim physical chronology because this realization does not supply cross-day persistence or complete same-substep phase/liquid chronology. Both operators publish shadow-specific shortwave, longwave, sensible, latent, precipitation-advection, internal active/lower conduction, vapor exchange, cold-content export where state evolves, available ice where applicable, total, terminal unallocated energy, residual, support, and coverage. Internal conduction may not be relabeled snow-ground flux; the absent snow-ground boundary remains unresolved. The real internal JSONL consumer must read every new operand and independently reject production/adjacent aliases. Turbulent primitive failures preserve their typed meteorology source and evaluation context. Solver extraction must preserve exported production API and move `runoff_reconciliation.rs` below 3,000 lines without changing formulas, thresholds, arithmetic order, production ownership, fixtures, observations, calibration, promotion, or public outputs. | evaluation hard-fail + governance hold | INV-SNOWFREEZE-091, INV-SNOWFREEZE-093, INV-SNOWFREEZE-094, OBL-SNOWFREEZE-P-067, OBL-SNOWFREEZE-C-009, REF-SNOWFREEZE-STAGE3-SHADOW-OBSERVABILITY, ADR-0042 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-096 | SNOW-STAGE3-TURBULENT-CARRIER-LINEAGE-AND-OPERATOR-RECONCILIATION observability: an enabled `INV-SNOWFREEZE-094/095` request may add one boxed, enabled-only reconciliation companion and internal schema-v6 row without changing the existing exported `TurbulentFluxes`, `DirectSnowStage3EvaluationDiagnostics`, `DirectSnowStage3EvaluationHourDiagnostics`, or `DirectSnowStage3EvaluationResult` shapes or call signatures. The existing public turbulent function and the diagnostic path must share one private Monin-Obukhov solver; duplicated formulas, coefficients, convergence logic, or arithmetic order are prohibited. Schema v6 publishes ordered duration-tagged tuples: one `3600 s` tuple per applicable same-state hour and every existing dynamic sequential substep, with typed applicability instead of numeric-zero N/A. Each tuple binds site/day/lane/operator/hour/substep identity; elapsed/requested/evaluated support; raw-source, forcing, geometry, effective-input, and frozen-projection custody; exact control-volume membership and active/total mass, depth, density, temperature, and cold content before/after; air/dewpoint/surface temperature, vapor pressures, wind, pressure, `z_T/z_q/z_u/z_0,aero`; solver options/status, stability length/corrections/log factors, exchange velocities, air density/potential temperature, specific humidities, and latent heat; shortwave, explicit albedo source/model/state, longwave/canopy/sky-view, precipitation-advection, vapor/melt/sublimation/deposition/cold-export, active/lower cold-content change, and residual operands. Historical schema-v5 meaning remains unchanged and disabled/default rows remain exact schema v4 with no companion allocation. The independent version-aware consumer must fail closed on unknown/malformed versions, reconstruct primitive fluxes and duration integrals without producer helpers or producer totals, preserve the legacy identity `mixed sequential complete = comparable external subset + active internal conduction`, join only exact common-support identities, and separate immutable whole-column `S`, frozen active-volume projection `F`, and sequential clone `Q` as `F-S` projection and `Q-F` evolution effects. Support censoring, unmatched rows, and the Snowbird right-censored year remain explicit. The output is operator-mechanics evidence only: no result may change equations, constants, thresholds, defaults, ownership, state, ledgers, fixtures, observations, or public outputs, or claim seasonal physical chronology, terminal closure, validation, promotion, CoE retirement, or cutover. | evaluation hard-fail + governance hold | INV-SNOWFREEZE-091, INV-SNOWFREEZE-093, INV-SNOWFREEZE-094, INV-SNOWFREEZE-095, OBL-SNOWFREEZE-P-068, OBL-SNOWFREEZE-C-010, REF-SNOWFREEZE-STAGE3-OPERATOR-RECONCILIATION, ADR-0042 | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| `INV-SNOWFREEZE-083` | runtime | Frost thermal input construction from residue/decomposition producer depth | Typed hard error on missing/non-finite/negative/stale dynamic residue-depth payload or seed-depth reuse after residue mass changes | Frost residue-cover implementation gate | `[DIRECT][Ran] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-007` | runtime | Winter payload boundary validator (hourly + daily coupling fields) | Typed hard error on missing/invalid required payload fields | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-008` | governance | Review/disposition/verification promotion check | Promotion `HOLD` when drift-active claims appear without authority-backed implementation status update | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-009` | runtime | Winter-routine trigger-condition branch gate | Typed hard error on silent skip when trigger condition is true | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-010` | runtime | CLIM05 snow-control adapter + hydrology snow-coupling branch | Typed hard error on missing/non-finite/domain-invalid `snow.options.*` controls or invalid `S`/`snow.runtime_swe` closure | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-011` | runtime | CLIM05 partition branch + WB13/hydout publication mapper | Typed hard error on cold-day liquid-partition mismatch (`RM > 0` when `Tmax <= rst`) or publication of `Snow-Water` from static `snow.options.ssd` instead of runtime SWE state | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-012` | runtime | Winter/frost dispatch gate and routine-chain sequencing validator (`winter`, `frostN`, `frwatc`, `frzng`, `frznw`), including material thaw-complete projection ownership | Typed hard error on active-branch dispatch mismatch, missing handoff call sequencing, silent bypass, or pre-ingress synthetic clearing/rebalancing of a material `frwatc` outcome | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| `INV-SNOWFREEZE-092` | runtime + governance | active-snow hourly generated-melt/rain finalization, private daily density handoff, bulk/multilayer density consumers, and CoE-bound snowbench mirror | Typed hard error on non-finite/negative driver or reconstruction failure; closure-blocking on current duplicate, routed-only, state-loss-plus-rain, retained-store, or other adjacent alias | SNOW-WETCOMPACT-DUP-001 defect-closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-093` | runtime + test + governance | future complete Stage 3 energy carrier, bounded phase conversion, same-substep liquid pipeline, linked ledgers, and real downstream consumer | Hard `IMPLEMENTATION_HOLD` on incomplete fluxes, unresolved residual-snow phase authority, partial/dual activation, missing real-consumer proof, or nonclosing energy/mass/liquid ledgers; typed closure failure after cutover | SNOW-COE-STAGE3-MELT-OWNER-AUTHORITY-RECONCILIATION and future implementation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-094` | test + governance; runtime only after a separate implementation package | typed evaluation request/tag, exact paired-arm fingerprints and reconstruction, operator-specific custody, complete sequential forcing/closure, negative production-consumer proof, support/coverage metadata, and claim audit | Evaluation request hard-fails on invalid enabled metadata or state aliasing; governance `HOLD` on untagged state, paired-arm inequality, incomplete chronology/closure, production reachability, cross-boundary carry, unsupported claims, or any runtime/default/public mutation | SNOW-STAGE3-EVALUATION-SHADOW-AUTHORITY contract gate and future operator implementation/noninterference gates | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-095` | typed request/tag resolver + extracted evaluator + internal JSONL consumer | exact disabled schema-v4 identity; enabled schema-v5 operator/tag/arm IDs; stable fingerprint equality; per-arm and sequential component reconstruction; requested/evaluated support and coverage; typed turbulent source preservation; production-state/ledger/output noninterference; line-count/API parity | Enabled evaluation hard-fails on request conflict, invalid metadata, fingerprint/coverage/component nonclosure, or typed primitive failure without changing authoritative execution; governance `HOLD` on producer-only proof, production reachability, overbroad claim, schema-v4 drift, unlabeled internal conduction, missing snow-ground boundary disclosure, cross-boundary carry, or any production/default/public mutation | SNOW-STAGE3-SHADOW-SOLVER-EXTRACTION-AND-OBSERVABILITY focused contract, module parity, runtime noninterference, anti-alias reconstruction, real schema-v5 consumer, error taxonomy, and exact-v4 gates | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-096` | shared private turbulent solver + enabled-only reconciliation companion + internal schema-v6 producer + independent version-aware consumer | exact v4 disabled identity and v5 historical semantics; no disabled allocation or protected API drift; tuple identity/order/duration/applicability; primitive turbulent/radiative/advected reconstruction; source/effective/frozen-projection custody; active/total state and mass/cold closure; legacy mixed-to-external bridge; exact common-support `S/F/Q` decomposition; unmatched/censored inventory; production noninterference | Enabled evaluation hard-fails on typed solver error, malformed or incomplete tuple, duplicate/non-contiguous identity, invalid applicability/null state, fingerprint mismatch, reconstruction/closure failure, or unknown schema without authoritative mutation; governance `HOLD` on duplicated physics, v4/v5 drift, producer-helper dependence, external/internal aliasing, support laundering, or any seasonal/terminal/production/cutover claim | turbulent-solver parity, schema-v6 writer/reader, primitive anti-tautology, endpoint custody, exact-support reconciliation, predecessor-bridge, noninterference, and four-site result gates | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| `hydrometeor_temperature` | `openwepp_meteorology::phase::HydrometeorTemperatureSolution::temperature` | candidate crate API only; not a production runtime surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `harder_pomeroy_rain_fraction`, `harder_pomeroy_snow_fraction` | `openwepp_meteorology::phase::PrecipitationPhaseFractions::{rain_fraction,snow_fraction}` | candidate crate API only; not a production runtime surface | fractions remain in `[0, 1]` and close to one | `[DIRECT][Static] + [INFERENCE][Static]` |
| `relative_humidity`, `dew_point_temperature`, `air_vapor_density`, `hydrometeor_saturation_vapor_density` | `openwepp_meteorology::psychrometrics::*` typed primitives | candidate crate API only; not a production runtime surface | dimensionless fraction, `degC`, and `kg m^-3` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrain_store` | `snow.hourly.rain_retained_m` | hourly rain retained in snowpack holding capacity | `m` -> `m`, finite and non-negative | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrain_release` | `snow.hourly.rain_released_m` | residual rain-on-snow added into final `hrmelt`/`wmelt` after holding-capacity accounting | `m` -> `m`, finite and non-negative | `[DIRECT][Static] + [INFERENCE][Static]` |
| `rain(iplane)` after winter processing | `snow.post_winter_rain_m` | daily direct-rain depth remaining after winter clearing/restoration, rain retention, and rain-on-snow promotion into routed `wmelt` | `m` -> `m`, finite and non-negative | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Thr` | `winter.hourly.air_temp_c` | hourly thermal forcing surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Thra`/`surtmp(hour)` | `frost.hourly.surface_temp_c_{idx4}` | hourly adjusted surface-temperature forcing consumed by frost top heat flow after legacy `hr_tmp`/`tmpadj` synthesis | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `frost_hourly_surface_temp` | `frost.hourly.surface_temp_c_{idx4}` | registry canonical name for the hourly frost top-boundary temperature | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Tmax`, `Tmin` | `tmax`, `tmin` (`HillslopeProductionStateSymbol::{Wb14Tmax,Wb14Tmin}`) | daily thermal forcing surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrad` | `winter.hourly.rad_mj_m2`, `winter.hourly.rad_mj_m2_{idx4}` | hourly radiation surface | `MJ m^-2 h^-1` preserved at registry boundary; legacy term-level conversion remains explicit | `[DIRECT][Static] + [INFERENCE][Static]` |
| `cancov`, `clouds` | `cancov` (`HillslopeProductionStateSymbol::Wb15PlantCancov`), `winter.hourly.cloud_fraction` | melt and surface-temperature modifiers | `fraction` -> `fraction` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Qsrf`, `Quf`, `Ksrf` | `frost.hourly.qsrf_w_m2`, `frost.hourly.quf_w_m2`, `frost.hourly.ksrf_w_m_k` | frost heat-flow bookkeeping surface | `W m^-2` / `W m^-1 degC^-1` unchanged | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Snowd`, `Resd`, `Tilld`, `Utilld` | `frost.hourly.snow_depth_m`, `frost.hourly.residue_depth_m`, `frost.hourly.tilled_frozen_depth_m`, `frost.hourly.untilled_frozen_depth_m` | layered conductivity state inputs | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Dfrost`, `Dthaw` | `frost.runtime_dfrost`, `frost.runtime_dthaw` (`HillslopeProductionStateSymbol::{Wb14FrostRuntimeDfrost,Wb14FrostRuntimeDthaw}`) | hourly frost/thaw depth boundary outputs | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `frdp` | `frdp`, `hillslope_wat.frdp`, `hillslope_wat.frdp:mm` | active frost-front depth runtime state and WAT publication surface | runtime `m`; WAT publication `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `S` | `S` (`HillslopeProductionFluxSymbol::Wb12SnowCouplingS`) | daily snow-water term in water-balance closure | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Snow-Water` (WB13/hydout publication surface) | `hillslope_wat.Snow-Water`, `hillslope_wat.Snow-Water:mm`; derived alias from `snow.runtime_swe` at publication boundary | replay/output storage-state publication | runtime SWE (`m`) is converted to published snow-water storage units `mm` at boundary without sidecar-control substitution | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Snow-Depth` | `Snow-Depth`, `hillslope_wat.Snow-Depth`, `hillslope_wat.Snow-Depth:mm` | physical snow-depth WAT publication surface | `mm` -> `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `MeltwaterTemperature` | `MeltwaterTemperature`, `hillslope_wat.MeltwaterTemperature`, `snow_meltwater_flux_temperature`, `hillslope_wat.MeltwaterTemperature:degC` | nullable layered-snow meltwater-temperature publication surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow.options.rst`, `snow.options.newsnw`, `snow.options.ssd`, `snow.options.snow_file_present` | identity (`HillslopeProductionStateSymbol::{Wb14SnowRst,Wb14SnowNewsnw,Wb14SnowSsd,Wb14SnowFilePresent}`) | parsed snow sidecar controls projected to runtime seam | scalar controls preserved; `snow_file_present` in `{0,1}` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_runtime_swe` | `snow.runtime_swe` identity (`HillslopeProductionStateSymbol::Wb14SnowRuntimeSwe`) | runtime snow-water-equivalent storage state | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_routed_melt`, `snow_post_winter_rain` | `snow.routed_melt_m`, `snow.solid_to_liquid_ledger.liquid_handoff_m`, `snow.post_winter_rain_m` | daily snow/liquid coupling publication surfaces | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_runtime_depth`, `snow_runtime_density`, `snow_runtime_settle_day_count` | `snow.runtime_depth_m`, `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count` | runtime snow carry-state surfaces | `m`, `kg m^-3`, and `count` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_wet_compaction_liquid_input` | private `SnowCouplingOutcome::wet_compaction_liquid_input_m`, then `SnowDensityRuntimeInputs::liquid_for_compaction_m` and `density_process_liquid_for_compaction_mass_kg_m2` | daily active-pack process driver carried from exact CoE hourly generation/rain lineage to both density consumers | `m water equivalent` becomes `kg m^-2` only at the density boundary using `1000 kg m^-3`; finite/nonnegative and exact-one | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_density_process_diagnostics` | `DirectSnowLiquidPartition::density_process_diagnostics` and `density_process_*` fields in `openwepp-r7h-direct-production-snow-trace-v3` | internal daily direct-production diagnostic ledger | density state/increments `kg m^-3`; initial snow, compaction-liquid, and snowfall load `kg m^-2`; snowfall depth `m`; compaction temperature `degC`; applicability, fresh-density availability, and fallback-use Booleans | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_solid_to_liquid_ledger` | `DirectSnowLiquidPartition::solid_to_liquid_ledger` and existing schema-v4 `raw_melt_m`, `snowpack_swe_loss_m`, `rain_released_m`, and `routed_melt_m` fields | compact daily production ledger and internal trace projection | `m` water equivalent preserved; raw signed melt remains distinct from authoritative state loss | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_liquid_disposition_ledger` | `DirectSnowLiquidPartition::liquid_disposition_ledger` and existing schema-v4 `stage3_*liquid*` fields | compact daily Stage-3 production ledger and internal trace projection | `m` water equivalent preserved; retained delta remains signed | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_accumulation_melt_diagnostics` | `DirectSnowLiquidPartition::verbose_diagnostics` and `phase_*`, `snowfall_swe_m`, `coe_melt_*`, and `modeled_wind_redistribution_*` fields in `openwepp-r7h-direct-production-snow-trace-v4` | optional internal hourly direct-production diagnostic ledger | rain/snowfall/melt contributions `m`; fractions dimensionless; hydrometeor temperature `degC`; phase/model status enumerated | `[DIRECT][Static] + [INFERENCE][Static]` |
| `winter_air_temp_hourly`, `winter_dewpoint_hourly`, `winter_wind_hourly`, `winter_cloud_fraction_hourly` | `winter.hourly.air_temp_c_{idx4}`, `winter.hourly.dewpoint_c_{idx4}`, `winter.hourly.wind_m_s_{idx4}`, `winter.hourly.cloud_fraction_{idx4}` | registry-owned hourly winter forcing surfaces | `degC`, `degC`, `m s^-1`, and `dimensionless` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_rain`, `snow_hourly_rain_retained`, `snow_hourly_snowfall` | `snow.hourly.rain_m_{idx4}`, `snow.hourly.rain_retained_m_{idx4}`, `snow.hourly.snowfall_m_{idx4}` | registry-owned hourly precipitation/snowpack forcing surfaces | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_stmtim_control_surfaces` | `snow.hourly.stmtim.{rain_m,stmdur_s,wntdur_h,wnttim_h,hrtemp_c,rst_c,hrrain_m,hrsnow_m,active_interval,rain_branch,snow_branch}_{idx4}` | HPHYS0318 trace-grade OpenWEPP SIMIMPL28 precipitation partition control surfaces | depths `m`, durations `s`/`h`, temperatures `degC`, flags `0/1` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_stmtim_phase_model` | `snow.hourly.stmtim.phase_model_{idx4}` | phase-selector trace surface | `dimensionless` -> `dimensionless` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_stmtim_rain_fraction` | `snow.hourly.stmtim.rain_fraction_{idx4}` | rain-fraction trace surface | `dimensionless` -> `dimensionless` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_stmtim_snow_fraction` | `snow.hourly.stmtim.snow_fraction_{idx4}` | snow-fraction trace surface | `dimensionless` -> `dimensionless` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_stmtim_relative_humidity` | `snow.hourly.stmtim.relative_humidity_{idx4}` | relative-humidity trace surface | `dimensionless` -> `dimensionless` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_stmtim_hydrometeor_temperature` | `snow.hourly.stmtim.hydrometeor_temperature_c_{idx4}` | hydrometeor-temperature trace surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_depth`, `snow_hourly_density` | `snow.hourly.depth_before_m_{idx4}`, `snow.hourly.depth_available_m_{idx4}`, `snow.hourly.depth_after_m_{idx4}`, `snow.hourly.density_before_kg_m3_{idx4}`, `snow.hourly.density_after_kg_m3_{idx4}` | registry-owned hourly snow state families | `m` and `kg m^-3` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_hourly_melt`, `snow_hourly_melt_raw`, `snow_hourly_melt_branch_active`, `snow_hourly_melt_terms` | `snow.hourly.melt_m_{idx4}`, `snow.hourly.melt_raw_m_{idx4}`, `snow.hourly.melt_branch_active_{idx4}`, `snow.hourly.melt_amelt_in_{idx4}`, `snow.hourly.melt_bmelt_in_{idx4}`, `snow.hourly.melt_cmelt_in_{idx4}`, `snow.hourly.melt_dmelt_in_{idx4}` | registry-owned hourly melt term/state families | `m`, signed `m`, `dimensionless`, and `in` preserved at diagnostic boundary | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0268/HPHYS0269 snowpack diagnostics | `snow_runtime_swe_m`, `snow_runtime_depth_m`, `snow_runtime_density_kg_m3`, `snow_runtime_settle_day_count`, `snow_s_m`, `snow_hourly_rain_sum_m`, `snow_hourly_rain_retained_sum_m`, `snow_hourly_snowfall_water_equiv_sum_m`, `snow_hourly_melt_raw_sum_m`, `snow_hourly_melt_sum_m`, `snow_runtime_swe_closure_error_m`, `wb13_rm_mm`, `wb13_snow_water_mm` | Opt-in run-trace evidence for classifying H1/H7/H39 spring snowpack/SWE/`RM` lineage and winter melt/rain-retention migration before assigning material `Ep` residual ownership | runtime state `m`/`kg m^-3`/`count`, daily coupling `m`, WB13 publication `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0270 daily snowpack state diagnostics | `snow_runtime_swe_before_m`, `snow_runtime_depth_before_m`, `snow_runtime_density_before_kg_m3`, `snow_runtime_settle_day_count_before`, `snow_runtime_swe_m`, `snow_runtime_depth_m`, `snow_runtime_density_kg_m3`, `snow_runtime_settle_day_count`, `snow_runtime_swe_delta_m`, `snow_runtime_depth_delta_m`, `snow_runtime_density_delta_kg_m3`, `snow_runtime_settle_day_count_delta` | Opt-in run-trace evidence for classifying daily snowpack carry-state residuals before assigning H1/H7/H39 spring divergence ownership to WB17 `Ep`, storage, WB13 publication, or another snowpack production seam | runtime state `m`/`kg m^-3`/`count` and daily deltas in same units | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0271/HPHYS0272 melt-forcing diagnostics | `snow.hourly.melt_raw_m_####`, `snow.hourly.melt_m_####`, `snow.hourly.melt_amelt_in_####`, `snow.hourly.melt_bmelt_in_####`, `snow.hourly.melt_cmelt_in_####`, `snow.hourly.melt_dmelt_in_####`, `snow.hourly.melt_hrtef_f_####`, `snow.hourly.melt_hrdtf_f_####`, `snow.hourly.melt_vwmph_####`, `snow.hourly.melt_rainin_####`, `snow.hourly.melt_wind_adjustment_####`, `snow.hourly.melt_branch_active_####`, `winter.hourly.rad_mj_m2_####`, `winter.hourly.dewpoint_c_####`, `winter.hourly.wind_m_s_####` | Opt-in run-trace evidence for classifying H1 day-36 spurious melt against `melt.for` term-level lineage, hourly forcing, and `SC-CLIMATE-001#INV-CLIMATE-013` radiation units before changing production physics | melt depths `m`; radiation `MJ m^-2 h^-1`; melt terms in inch-equivalent pre-`0.0254` conversion, temperatures `degF`/`degC`, wind `mph`/`m s^-1`, rain `in`, flags `0/1` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ws_frz`, `Nft` | `frost.runtime_ws_frz`, `frost.runtime_nft` (`HillslopeProductionStateSymbol::{Wb14FrostRuntimeWsFrz,Wb14FrostRuntimeNft}`) | frozen-soil coupling boundary outputs | units preserved as declared | `[DIRECT][Static] + [INFERENCE][Static]` |
| `InfCap_frz` | `frost.runtime_infcap_frz` (`HillslopeProductionStateSymbol::Wb14FrostRuntimeInfcapFrz`) | frozen-soil infiltration-capacity boundary output | `m s^-1` required at exported boundary | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow_stage3_operator_reconciliation_tuple` | enabled-only boxed `DirectSnowStage3OperatorReconciliationDiagnostics` / ordered `DirectSnowStage3OperatorReconciliationTuple` companion projected to internal schema v6 | Evaluation-only scalar/vector diagnostic exception: it never crosses a production kernel boundary, is not registered as a `HillslopeProductionStateSymbol` or `HillslopeProductionFluxSymbol`, and is consumer-forbidden outside the independent internal analyzer. Each exact field, unit, time/area basis, nullable rule, and equation is bound by the v129 reconciliation addendum. | Typed meteorology wrappers validate dimensional carrier inputs; Rust field suffixes and the canonical schema table preserve SI units; the only explicit conversions are hourly radiation `MJ m^-2 * 10^6 / 3600 -> W m^-2`, snowfall geometry `m * 0.1 * 1000 / 3600 -> kg m^-2 s^-1`, rain `m * 1000 / 3600 -> kg m^-2 s^-1`, and `flux * duration -> J m^-2` or `kg m^-2`. | `[DIRECT][Static] + [INFERENCE][Static]` |

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
- Default activation, production parser/runfile/CLI selector exposure, output-schema publication, or compatibility rollback removal for `coe_shortwave_albedo_v1` before a later activation package clears the `INV-SNOWFREEZE-056` evidence baseline. `[INFERENCE][Static]`
- SNOWDENSITY-06 or later density work consuming `coe_shortwave_albedo_v1` while retuning melt coefficients, radiation forcing, or albedo constants to improve density signatures. `[INFERENCE][Static]`
- SNOWDENSITY-06 `density_compaction_v1` evidence scored without a density/densification robust-cell profile, or claimed as successful because of melt/timing compensation rather than density-cell improvement. `[INFERENCE][Static]`
- SNOWDENSITY-06B CoE-bound density replay evidence that lets daily SWE drift from the selected CoE boundary, reuses the old degree-day `physics_bulk` melt surrogate, or treats mixed/deciduous canopy adjudication as a substitute for fixed-boundary density evidence. `[INFERENCE][Static]`
- Active same-day future snowfall under `coe_shortwave_albedo_v1` clearing opt-in albedo continuity solely because earlier same-day hours were snow-free, instead of applying fresh-snow reset, carrying a valid previous opt-in state, or failing closed. `[DIRECT][Static] + [INFERENCE][Static]`
- Any albedo constants or reset thresholds fitted to SNOTEL, frost-site observations, legacy residuals, or PySnobal residuals. `[INFERENCE][Static]`
- `melt_bmelt_in` sign semantics changed by silent sign flip or double subtraction without a new contract amendment and source-line proof. `[DIRECT][Static] + [INFERENCE][Static]`
- `openwepp-meteorology` selected or invoked by production winter precipitation partitioning, `RST` replacement, parser/runfile/user configuration, output publication, fixture mutation, compatibility-runtime behavior, or default activation outside the explicitly opt-in SNOWDENSITY-10.3.5b hourly partition seam. `[INFERENCE][Static]`
- Harder-Pomeroy candidate APIs returning non-finite hydrometeor temperature, vapor pressure, vapor density, latent heat, diffusivity, conductivity, or precipitation fractions; rainfall/snowfall fractions outside `[0, 1]`; or rainfall plus snowfall fraction not closing to one within numerical roundoff. `[DIRECT][Static] + [INFERENCE][Static]`
- `harder_pomeroy_hourly` producing active-hour precipitation partitions where `hrrain + hrsnow / 10` fails to reconstruct the active hourly precipitation depth within roundoff, or where the default `legacy_rst` branch differs from the pre-10.3.5b threshold behavior. `[DIRECT][Static] + [INFERENCE][Static]`
- Supersaturated dewpoint/RH inputs silently clamped without the `INV-SNOWFREEZE-065` exact-saturation normalization evidence, or any non-finite/negative/zero-saturation humidity input proceeding through the opt-in hourly partition. `[DIRECT][Static] + [INFERENCE][Static]`
- `coe_winter_thaw_state_loss_v1` selected by default, exposed through parser/runfile/user activation, consuming albedo state, altering CoE melt coefficients/forcing/canopy/phase/density constants, or reported as closure without paired thaw-window under-ablation and aggregate-deficit improvement evidence. `[DIRECT][Static] + [INFERENCE][Static]`
- `coe_open_sublimation_stage_a_v1` selected by default, exposed through parser/runfile/user activation, changing the activated density cap/default, reading PySnobal/libsnobal C source without confirmed non-GPL-family licensing, using fixture-fitted turbulent constants, routing sublimated vapor as melt/liquid, leaving sublimated mass out of the internal snow-state conservation ledger, changing public output schemas, worsening paired under-persistence, or claiming activation/frost attribution from Stage A evidence. `[DIRECT][Static] + [INFERENCE][Static]`
- `coe_open_sublimation_stage_b_v1` selected by default, exposed through parser/runfile/user or `.run` controls, mutating the current Harder-Pomeroy phase default or activated density/melt defaults, using fixture-tuned active-layer thresholds or turbulent constants, reading libsnobal C without recording CC0/non-GPL provenance, routing sublimated vapor as melt/liquid, omitting vapor from internal conservation ledgers, changing public output schemas, or claiming promotion without beating the current default on the cross-SNOTEL forcing-robust rubric. `[DIRECT][Static] + [INFERENCE][Static]`
- `physics_bulk_shallow_guard_v1` selected by default, exposed through parser/runfile/user activation, changing the activated density cap/default, reading PySnobal/libsnobal C source without confirmed non-GPL-family licensing, fitting the shallow threshold or guard factor to fixtures, changing SWE/melt/liquid/routed outputs, changing public output schemas, consuming observed depth/density/site/residual inputs at runtime, worsening paired over-persistence, or claiming activation/frost attribution from shallow-guard evidence. `[DIRECT][Static] + [INFERENCE][Static]`
- `physics_bulk_climate_class_density_v1` selected by default, exposed through parser/runfile/user activation, assigning snow class from geography/site identity/observed residuals/fixture names/NSIDC lookup, inventing Sturm 1995 decision thresholds, fitting class thresholds or parameters to SNOTEL/cancov fixtures, proceeding for a class without authority-backed parameters, changing density cap, melt/liquid/routed outputs, public output schemas, frost behavior, or claiming validation for classes absent from the observed corpus. `[DIRECT][Static] + [INFERENCE][Static]`
- Before an atomic `INV-SNOWFREEZE-093` cutover, `layered_thermal_liquid_v1`
  selected by default, exposed through parser/runfile/user or `.run` controls,
  running without valid layer state, or replacing the compatibility CoE melt
  path. At or after cutover, changing public snow/WAT schemas except for the
  ratified nullable `MeltwaterTemperature` WAT parquet field, producing layer
  temperatures above freezing, publishing untyped or unbounded meltwater
  temperature, silently discarding refrozen liquid, leaving mass/liquid/energy
  residuals materially nonzero, changing the bulk snow-density rubric under
  the Stage 3-Decouple arm, or serializing meltwater temperature through
  HBP/watershed paths before a later stream-temperature package authorizes that
  boundary remains invalid. `[DIRECT][Static] + [INFERENCE][Static]`
- Any Stage 3 melt-owner activation while a CoE generator remains live; while
  net radiation, sensible heat, latent heat, ground/interlayer conduction, or
  precipitation-advected heat is omitted or duplicated; before exact residual-
  snow phase disposition at `m_s <= 1 kg m^-2`; or without same-substep linked
  energy, solid-to-liquid, and liquid-disposition closure. `[DIRECT][Static] +
  [INFERENCE][Static]`

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
- OBL-SNOWFREEZE-P-030: Any `coe_shortwave_albedo_v1` routed-melt
  implementation must prove default `legacy_coe` identity, typed active-snow
  fail-closed behavior for missing/invalid opt-in albedo state, formula-level
  `amelt` reconstruction, signed raw-melt reconstruction, corrected
  negative-melt redistribution, routed `wmelt`, SWE storage loss, WB12 `S`, and
  WB13 liquid-forcing closure. It must carry the updated albedo state through
  typed runtime snow state and close `HOLD` if any accepted liquid-forcing
  operand can only be inferred from SWE/depth/density alias surfaces.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-031: Any density or activation producer consuming
  `coe_shortwave_albedo_v1` after SNOWDENSITY-05F must treat melt as a fixed
  opt-in boundary: preserve `legacy_coe` default/rollback, carry selector,
  hourly shortwave, albedo state/model/age/reset, absorbed-shortwave fraction,
  raw melt, redistributed melt, routed `wmelt`, SWE loss, WB12 `S`, WB13 liquid
  forcing, and runtime SWE/depth/density after-state as typed operands; do not
  retune melt, albedo, or radiation to improve density; and report both 05E
  diagnostic replay and H as-built context before any default-candidate claim.
  The 05E replay profile must be labeled regime-limited until the harness
  consumes real per-day canopy cover instead of `cancov = 0.0`; for the
  configured coniferous forest validation fixtures, winter `cancov` is expected
  near `0.9`. The harness must also use
  native/proven shortwave radiation instead of an unproven PySnobal-bridge
  radiation inversion.
  Producers must explicitly handle same-day future snowfall cold-start albedo
  continuity by fresh-snow reset, valid previous opt-in carry, or typed
  fail-closed disposition.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-032: Any diagnostic replay or density package using
  SNOWDENSITY-05G melt evidence must carry the representative-regime harness
  proof forward: configured coniferous validation fixtures must not be scored
  with `cancov = 0.0`; replay artifacts must publish the canopy source,
  representative canopy value or series summary, shortwave source, and
  bridge-inversion identity when the PySnobal forcing file is used as the
  transport surface. This proof is evidence for adjudication only and does not
  create a production activation selector.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-033: Any SNOWDENSITY-06 `density_compaction_v1` producer
  must remain offline/diagnostic, preserve baseline candidate melt coefficients,
  albedo constants, canopy handling, shared-radiation inputs, production
  defaults, and rollback paths, publish the named PTM/POC/liquid-water
  compaction constants in its report, prove SWE conservation and finite thermal
  residuals, reject site-specific constants, and score both whole-rubric context
  and density/densification robust cells before any promotion claim.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-034: Any SNOWDENSITY-06B CoE-bound density replay producer
  must run only as an offline diagnostic, name the fixed CoE melt boundary
  (`legacy_coe` or `coe_shortwave_albedo_v1`), preserve daily CoE
  `snow_water_m` identity within roundoff, publish boundary SWE-loss/routed
  liquid totals and identity residuals, apply only the ratified
  `density_compaction_v1` density update to depth/density, reject site-specific
  constants, and score both whole-rubric and density/densification robust cells
  before any SNOWDENSITY-07 runtime opt-in claim.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-035: Any SNOWDENSITY-07 runtime opt-in producer must keep
  `legacy_wepp` as the default snow-density model and must prove that the
  default/surface-driven compatibility path does not construct, select, or
  publish `physics_bulk_density_compaction_v1`. The opt-in path must preserve
  CoE SWE/liquid/routed-melt identity, publish the separate CoE boundary
  depth/density/settle-count carry used for future melt-boundary calculation,
  mutate only runtime physical depth and density, reject site-specific
  constants, and prove state mutation, downstream operands, shadow projection,
  runtime carry, and existing publication consumers read the opt-in depth/density
  rather than an adjacent SWE or CoE-boundary alias.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-036: Any SNOWDENSITY-08 gate-rerun producer must report
  SNOTEL density evidence and non-SNOTEL frost-site evidence separately. It may
  call the SNOTEL gate cleared only from a same-lineage CoE-bound replay with
  daily SWE identity and no site constants. It may call frost attribution
  unblocked only from a coupled non-SNOTEL WAT/publication run that applies
  `physics_bulk_density_compaction_v1` to the actual runtime snow-depth state
  consumed by frost and WAT `Snow-Depth`. If that coupled path is absent, the
  package must close with the blocker named and must not rewrite WAT, substitute
  offline snow-only depth for a coupled frost run, or treat default-path
  snow-control failures as opt-in evidence.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-037: Any SNOWDENSITY-09 diagnostic WAT-rerun producer must
  keep `legacy_wepp` as the default direct-production density model and must
  use `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL` only as a package-bound diagnostic
  environment selector. The producer must fail closed on unknown selector
  values, emit trace evidence naming the selected `snow_density_model`, run the
  non-SNOTEL frost fixtures through the real WAT publication path without WAT
  rewriting, and report default-vs-opt-in snow-control/frost rubric deltas. It
  must compute the snow-control gate from observed-snow-depth fixtures only,
  must report no-observed-snow fixtures separately as diagnostic-only
  out-of-gate evidence, and must not add parser/runfile/user CLI activation,
  output-schema changes, constants, tuning, or frost-physics edits.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-038: Any SNOWDENSITY-10.3 low-canopy/mixed/deciduous melt
  adjudication producer must consume the direct-production per-day
  `cancov_daily_series` when using snowbench or CoE melt replay. The producer
  must publish the series source, row count, date alignment, min/max/mean/first/
  last summary, and fail-closed validation for missing, duplicated, non-finite,
  out-of-range, or length-mismatched rows. A scalar runtime-surface `cancov`
  may be reported only as an initial-state or backward-compatible summary, not
  as seasonal canopy authority. This obligation does not authorize canopy
  tuning, coefficient tuning, production activation, output-schema changes, or
  fixture edits.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-039: Any SNOWDENSITY-10.3.5a meteorology-crate producer must
  implement only a pure reusable `openwepp-meteorology` crate and must publish
  contract-first evidence, clean-room provenance, and production non-wiring
  scans before closure. The crate must use typed unit-boundary inputs/outputs or
  locally typed wrappers for Celsius temperature, unit-interval humidity, vapor
  pressure, vapor density, latent heat, diffusivity, conductivity, hydrometeor
  temperature, and rain/snow fractions. Tests must cover finite-domain guards,
  water/ice saturation-vapor-pressure reference values, dewpoint/RH round trips,
  Harder-Pomeroy fixed-point convergence, saturated-air identity, rainfall-
  fraction monotonicity for coefficient sets, fraction closure, and explicit
  non-convergence behavior. The package must fail closed rather than wire this
  crate into production `RST`, `stmtim`, parser/runfile/user selectors, output
  schema, compatibility runtime, or default behavior.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-040: Any SNOWDENSITY-10.3.5b opt-in hourly partition
  producer must amend this contract before runtime code, add contract-derived
  tests for `INV-SNOWFREEZE-065`, preserve default `legacy_rst` identity, fail
  closed on unknown selector values, and prove the real direct-production snow
  consumer receives the selected hourly `hrrain`/`hrsnow` values. The producer
  must publish default-vs-opt-in near-freezing cases, active-hour precipitation
  reconstruction, exact-saturation normalization evidence for supersaturated
  dewpoint-derived RH, and Jennings et al. observed-phase validation artifacts
  with rows scored, stations scored, accuracy, confusion counts, per-station
  predicted 50% air-temperature thresholds, and humid/maritime-vs-dry/
  continental threshold contrast when metadata supports it. The package must
  not add parser/runfile/user CLI selectors, public output-schema changes,
  fixture edits, default activation, phase-coefficient tuning, density/melt/
  canopy/radiation/frost changes, or compatibility-runtime changes.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-041: Any SNOWDENSITY-10.3.7 winter-thaw melt-response
  producer must amend this contract before runtime code, add contract-derived
  tests for `INV-SNOWFREEZE-066`, preserve `legacy_coe` default identity and
  `coe_shortwave_albedo_v1` behavior, and expose `coe_winter_thaw_state_loss_v1`
  only as an explicit opt-in typed/snowbench diagnostic selector. Any direct-
  production WAT rerun for this package must use the package-bound diagnostic
  selector `OPENWEPP_SNOWDENSITY1037_MELT_MODEL`, preserve absent-selector
  `legacy_coe` behavior, reject unknown values, and avoid parser/runfile/user
  CLI activation. The producer
  must independently reconstruct raw CoE melt, redistributed melt, routed melt,
  retained/released rain, snowpack SWE loss, modeled depth loss, final snow-
  state closure, under-ablation counts, aggregate depth-loss deficit, and
  coupled direct-production WAT snow-control deltas from produced artifacts.
  It must prove that released snowpack water does not exceed available snowpack
  storage plus same-day snow/rain inputs, and that snowpack SWE state loss is
  routed into the downstream liquid/WAT balance rather than disappearing as an
  isolated depth decrement. It must improve both paired Sleepers/Harvard under-
  ablation count and aggregate depth-loss deficit relative to `legacy_coe`
  before closure, and it must report whether the coupled direct-production WAT
  snow-control gate improves, is neutral, worsens, or remains blocked. It must
  not add
  parser/runfile/user CLI activation, public output-schema changes, fixture
  edits, default activation, melt coefficient tuning, radiation/canopy/phase/
  density/frost changes, sub-canopy longwave, rain heat, Qwet/frzftp, site
  constants, or compatibility-runtime changes.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-042: Any SNOWDENSITY-10.3.8 liquid holding-capacity
  drainage producer must amend this contract before runtime code, add
  contract-derived tests for `INV-SNOWFREEZE-067`, preserve `legacy_coe`
  default identity, preserve `coe_shortwave_albedo_v1` and
  `coe_winter_thaw_state_loss_v1` behavior, and expose
  `coe_liquid_holding_capacity_v1` only as an explicit opt-in typed/snowbench
  diagnostic selector. Any direct-production WAT rerun for this package must
  use the package-bound diagnostic selector
  `OPENWEPP_SNOWDENSITY1038_MELT_MODEL`, preserve absent-selector
  `legacy_coe` behavior, reject unknown values, and avoid parser/runfile/user
  CLI activation. The producer must independently reconstruct raw CoE melt,
  redistributed melt, routed melt, retained/released rain, retained/released
  snow liquid, liquid holding capacity, persistent retained-liquid after-state,
  snowpack SWE loss, modeled depth loss, final snow-state closure, under-
  ablation counts, aggregate depth-loss deficit, and coupled direct-production
  WAT snow-control deltas from produced artifacts. It must prove that released
  snowpack water is bounded by incoming liquid plus previously retained liquid
  above current holding capacity, and that released liquid is routed into
  downstream liquid/WAT balance rather than disappearing as an isolated depth
  decrement. It must improve both paired Sleepers/Harvard under-ablation count
  and aggregate depth-loss deficit relative to `legacy_coe` before closure, and
  it must report whether the coupled direct-production WAT snow-control gate
  improves, is neutral, worsens, or remains blocked. It must not add
  parser/runfile/user CLI activation, public output-schema changes, fixture
  edits, default activation, melt coefficient tuning, radiation/canopy/phase/
  density/frost changes, sub-canopy longwave, rain heat, Qwet/frzftp, site
  constants, or compatibility-runtime changes.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-043: Any SNOWDENSITY-10.3.11 spring compaction/
  densification producer must amend this contract before runtime code, add
  contract-derived tests for `INV-SNOWFREEZE-068`, preserve `legacy_wepp`
  default identity, preserve `physics_bulk_density_compaction_v1` behavior,
  and expose `physics_bulk_spring_densification_v1` only as an explicit opt-in
  density model. Any direct-production WAT rerun for this package must use the
  package-bound diagnostic selector `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL`,
  preserve absent-selector `legacy_wepp` behavior, reject unknown values, and
  avoid parser/runfile/user CLI activation. The producer must prove SWE
  identity with the selected CoE boundary, final density bounded by
  `522 kg m^-3`, no observed-depth/density/fixture coupling, no site constants,
  and trace evidence that the selected density model reached the direct snow
  partition. It must compare the real coupled WAT snow-control gate against the
  `coe_liquid_holding_capacity_v1` plus `physics_bulk_density_compaction_v1`
  baseline, report March/April compaction-feasible row clearance, cap-limited
  residuals, under-persistence guardrails, and per-surface snow-control deltas,
  and it must not worsen any observed-snow-depth paired surface before any
  opt-in improvement claim. It must not add parser/runfile/user CLI activation,
  public output-schema changes, fixture edits, default activation, density-cap
  changes, observed-depth fitting, melt/radiation/canopy/phase/rain-heat/
  longwave/frost changes, Qwet/frzftp, or compatibility-runtime changes.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-044: Any SNOWDENSITY-10.3.12 combined bundle activation
  adjudication producer must preserve both existing opt-in boundaries and must
  not add new process physics. The package must run the real direct-production
  WAT path with `OPENWEPP_SNOWDENSITY1038_MELT_MODEL =
  coe_liquid_holding_capacity_v1` and `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL =
  physics_bulk_density_compaction_v1`, or close `HOLD` if current direct
  evidence cannot be produced. It must prove, from trace rows, that both
  selected members reached the direct snow partition; compare default,
  liquid-holding-only, combined-bundle, and spring-densification evidence;
  classify remaining observed-snow-depth failures by surface, cover, month,
  residual sign, and March/April cap class; and report observation-blocked
  surfaces separately. It must evaluate Activation Policy B by reporting whether
  the bundle is strictly better than the current default over gate-eligible
  paired-snow surfaces and whether the workspace-suite no-regression gate under
  the bundle selectors plus composite snow-state conservation closure is present.
  Downstream snow-affected output deltas are conserved-by-construction and are
  not separately diffed by this gate. A package that lacks workspace-suite/
  conservation evidence must close `HOLD-OPT-IN-BUNDLE` even if snow-depth
  residuals improve. Paired snow-depth residuals must keep frost attribution
  separately blocked until snow control is good enough to isolate frost. It must
  not add parser/runfile/user CLI activation, public output-schema changes,
  fixture edits, density-cap changes, observed-depth fitting, coefficient
  tuning, melt/radiation/canopy/phase/rain-heat/longwave/frost changes,
  Qwet/frzftp, or compatibility-runtime changes.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-045: Any SNOWDENSITY-10.3.13 residual-tail and Policy-B
  diagnostic producer must consume real direct-production WAT evidence from the
  accepted default/holding-only/bundle/spring-densification ladder and pair it
  to observations by date. It must report date-level state transitions, not only
  aggregate counts, for under-persistence and over-persistence failures; must
  distinguish activation blockers from frost-attribution blockers; and must
  publish a Policy-B workspace-suite/conservation evidence matrix naming every
  missing activation scope and noting that downstream snow-affected output deltas
  are not separately diffed by this diagnostic. It must keep `522 kg m^-3` as the active density cap,
  report any `550 kg m^-3` cap consideration as follow-up only, and must not add
  default activation, production physics, density-cap changes, selector
  surfaces, parser/runfile/user controls, fixture changes, output-schema
  changes, coefficient tuning, observed-depth fitting, Qwet/frzftp, frost
  attribution, or compatibility-runtime changes.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-046: Any SNOWDENSITY-10.3.14 Policy-B no-regression and
  cap-authority producer must consume the SNOWDENSITY-10.3.12 real direct-
  production bundle report and the SNOWDENSITY-10.3.13 residual diagnostic,
  verify composite trace state closure and active density-cap bounds, quantify
  cap-pinned rows under the active `522 kg m^-3` cap and a `550 kg m^-3`
  same-SWE depth projection, record the workspace-suite no-regression gate
  status under the existing bundle selectors plus composite snow-state
  conservation closure, explicitly note that downstream snow-affected output
  deltas are conserved-by-construction and not separately diffed, and distinguish
  active-cap activation-package readiness from dynamic `550 kg m^-3` cap
  re-anchor readiness. It must not change defaults, production physics, density caps,
  selectors, parser/runfile/user controls, fixture inputs, output schemas,
  Qwet/frzftp, frost attribution, or compatibility runtime.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-047: Any SNOWDENSITY-10.3.15 default-activation producer
  must amend this contract before production edits, then prove that the real
  direct-production no-env path selects `coe_liquid_holding_capacity_v1` and
  `physics_bulk_density_compaction_v1` in WAT/trace-consuming runs. It must
  retain explicit rollback/test selectors for `legacy_coe` and `legacy_wepp`,
  reject unsupported selector values fail-closed, and verify no parser,
  runfile, user CLI, output-schema, fixture, compatibility-runtime, Qwet,
  frzftp, density-cap, or frost-attribution surface changed. Package evidence
  must run workspace gates on the no-env default path, record active `522 kg m^-3`
  cap preservation, document downstream snow-affected output deltas as expected
  conserved consequences rather than separately diffed regressions, and carry
  forward the `498/1415` residual snow-control failure count as a frost-
  attribution blocker rather than a hidden activation pass.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-048: Any SNOWDENSITY-10.3.16 open-surface ablation Stage A
  producer must amend this contract before production edits, then implement
  `coe_open_sublimation_stage_a_v1` only behind an explicit opt-in selector
  while preserving the activated `coe_liquid_holding_capacity_v1` default and
  `legacy_coe` rollback. It must derive turbulent latent mass-loss constants
  from Marks/SNOBAL paper authority or physical constants, not fixture tuning;
  must fail closed on unsupported selector values; must bound sublimated mass by
  available snowpack SWE; must publish sublimation only in internal trace and
  conservation artifacts unless a later schema amendment authorizes public
  output; must prove that sublimation is not routed melt/liquid; and must run
  coupled direct-production WAT/trace gates for open-surface cap-limited
  over-persistence reduction, under-persistence non-worsening, magnitude range,
  and whole-model snow-state conservation. It must not change defaults, density
  caps, output schemas, parser/runfile/user controls, fixture inputs,
  compatibility runtime, frost attribution, Qwet/frzftp, or two-layer
  snow-surface structure.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-049: Any SNOWDENSITY-10.3.17 shallow-pack compaction-guard
  producer must amend this contract before production edits, then implement
  `physics_bulk_shallow_guard_v1` only behind an explicit opt-in density
  selector while preserving the activated `physics_bulk_density_compaction_v1`
  default and `legacy_wepp` rollback. It must derive the shallow guard threshold
  from Marks/SNOBAL active surface-layer depth authority (`0.25 m`) and not from
  fixture fitting; must fail closed on unsupported selector values; must preserve
  SWE identity with the selected CoE runtime boundary; must preserve the active
  `522 kg m^-3` cap; must prove the candidate changes only physical runtime
  depth/density and not melt/liquid/routed mass terms; and must run coupled
  direct-production WAT/trace gates for induced under-persistence reduction
  (with `harvard_hardwood` reported explicitly), over-persistence non-worsening,
  threshold authority, and whole-model snow-state conservation. It must not
  change defaults, density caps, output schemas, parser/runfile/user controls,
  fixture inputs, compatibility runtime, frost attribution, Qwet/frzftp,
  sublimation, two-layer snow-surface structure, or any melt/canopy/radiation/
  phase/rain-heat/longwave/frost behavior.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-050: Any SNOWDENSITY-10.3.19 Harder-Pomeroy phase-default
  producer must amend this contract before production edits, then prove that
  the real direct-production no-env path selects `harder_pomeroy_hourly`
  together with `coe_liquid_holding_capacity_v1` and
  `physics_bulk_density_compaction_v1` in coupled WAT/trace-consuming runs. It
  must retain `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=legacy_rst` as an explicit
  rollback/test selector, treat absent and empty selector values as the new
  phase default, reject unknown selector values fail-closed, and avoid adding
  parser/runfile/user CLI selectors, `.run` disable controls, fixture edits,
  public output-schema changes, density-cap changes, Qwet/frzftp, frost
  behavior, or compatibility-runtime changes. Package evidence must rerun the
  cross-SNOTEL `INV-SNOWFREEZE-050` forcing-robust rubric on a real
  direct-production no-env default and prove it is at least as good as the
  prior activated bundle with `legacy_rst` phase, run workspace-suite
  no-regression under the new no-env default, close active-hour partition mass
  conservation, and carry forward release notes for the non-representative
  humid-New-England depth regression roadmap item and the cross-SNOTEL density
  bias rise of about `+23.6 kg m^-3`.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-051: Any SNOWDENSITY-10.3.20 sublimation Stage B producer
  must amend this contract before production edits; prove PySnobal/libsnobal
  provenance if source equations are read; keep current no-env defaults and
  explicit rollback selectors intact; expose `coe_open_sublimation_stage_b_v1`
  only through the existing package-bound internal melt selector; and reject
  unsupported selector values fail-closed. Evidence must diagnose Stage A
  degradation by site, signature, residual component, and sublimation magnitude;
  score partition+sublimation composition and Stage B on the real cross-SNOTEL
  direct-production `INV-SNOWFREEZE-050` rubric; prove sublimation vapor closure
  and active-hour phase partition closure; and demonstrate no fixture, public
  output-schema, density-cap, frost, parser/runfile/user CLI, `.run` disable,
  Qwet/frzftp, compatibility-runtime, or site-calibration change. Promotion is
  authorized only when the candidate beats the current default on the
  forcing-robust primary rubric and passes conservation; otherwise it remains
  opt-in/non-promoted or the package closes `HOLD`.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-052: Any SNOWDENSITY-10.3.22 climate-class density
  specialization producer must amend this contract before production edits,
  expose `physics_bulk_climate_class_density_v1` only through the existing
  package-bound internal density selector, and keep the current no-env default
  plus `legacy_wepp` rollback intact. It must implement the full Sturm class
  label set and require runtime class assignment from the run's own wind,
  precipitation, and air-temperature climate under Sturm 1995 authority; it
  must not use geographic lookup, site identity, observed snow residuals, or
  fixture-specific constants. Evidence must record the verified Sturm 1995
  thresholds (`Tc=10 degC`, `CDM=30/125 degC-month`, `SPR=2 mm d^-1`, wind
  bracket `0.5-2.0 m s^-1`), the Sturm/Liston 2021 cross-check differences,
  Sturm 2010 parameter coverage, the documented ephemeral fresh-snow/Anderson
  fallback, any coefficient-translation residuals, any raw density-form
  fallback, class-boundary behavior, and honest validation coverage for classes
  absent from the current corpus. Promotion is authorized only after a real
  cross-SNOTEL direct-production `INV-SNOWFREEZE-050` run beats the current
  default, proves the bidirectional densification-trajectory flip, creates no
  new persistence tail, and closes conservation. Missing class thresholds,
  unresolved wind-ambiguous runtime classifications, missing class
  parameters/fallbacks, missing rubric evidence, or worse guardrail results
  must close `HOLD` or non-promotion without activation.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-053: Any Paradigm 2 Stage 1 layered snow-density producer
  must amend this contract before production edits, expose
  `physics_bulk_multilayer_density_v1` only through the existing package-bound
  internal density selector, and keep the current no-env default plus
  `legacy_wepp` rollback intact. Evidence must prove the real direct-production
  snow consumer reads and persists the layer stack through `DirectSnowLaneState`,
  `DirectSnowRuntimeCarry`, typed snow partition, R4G coupling, and the next-day
  winter-column state; aggregate WAT/public outputs must remain unchanged in
  schema and must derive from the aggregate layer outcome when the candidate is
  selected. The package must publish layer aggregate conservation, local-overburden
  unit tests, protected-boundary scans, no parser/runfile/user selector proof, the
  cross-SNOTEL+cancov `INV-SNOWFREEZE-050` rubric, bidirectional densification
  evidence, persistence-tail guardrail evidence, and ADR-0025 performance evidence.
  Promotion is authorized only if the candidate beats the current default on the
  primary rubric, fixes the split-sign densification trajectory both directions,
  creates no new persistence tail, and closes conservation; otherwise it remains
  opt-in/non-promoted or the package closes `HOLD`.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-054: Any Paradigm 2 Stage 2 snow-to-frost insulation-profile
  producer must amend this contract before production edits, expose
  `layered_resistance_v1` only through an internal package-bound
  `snow_frost_insulation_model` selector, and preserve the existing bulk
  `snow_depth_m`/`snow_density_kg_m3` handoff as the absent-selector rollback.
  Evidence must first prove the Stage 1 real direct-production layer stack forms
  a basal-denser-than-surface density gradient; if it does not, the package must
  close `HOLD` or repair Stage 1 before any frost coupling. The implementation
  must compute `sum(depth_i / k_sturm1997(rho_i))` over prior-day snow layers,
  project only an insulation-equivalent bulk density into the existing frost
  solver, prove finite/domain and density-cap bounds, and prove prior-day timing
  decoupling is preserved. Promotion is frost-primary and requires the frost
  observation-corpus forcing-robust signatures to improve versus the bulk handoff,
  the cross-SNOTEL snow rubric not to worsen, snow/frost conservation to close,
  and ADR-0025 H2637 performance evidence to pass. Missing gradient evidence,
  worse frost fidelity, snow-rubric regression, conservation/performance failure,
  or Stage 3 thermal/melt/liquid scope creep must close `HOLD` or non-promotion
  without activation.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-055: Any Paradigm 2 Stage 3 per-layer liquid-routing and
  meltwater-temperature producer must amend this contract before production
  edits, expose `layered_thermal_liquid_v1` only through an internal
  package-bound `snow_stage3_liquid_routing_model` selector, and preserve the
  disabled path as the absent-selector rollback. Evidence must prove Stage 3
  consumes the real Stage 1 layer stack, carries per-layer temperature,
  cold-content, liquid-water, and refreeze diagnostics, keeps existing CoE
  routed-melt mass authoritative, produces a typed meltwater-flux temperature,
  and closes mass, liquid, and energy ledgers from produced operands. The
  package must prove no default activation, no parser/runfile/user or `.run`
  control exposure, no fixture/schema/density-cap/frost/default change, no full
  in-stream temperature routing, no snow-rubric promotion claim, no untyped
  temperature publication, and ADR-0025 H2637 performance evidence. Missing
  layer consumer proof, unphysical meltwater temperature, liquid or energy
  non-closure, runoff-timing regression, snow-rubric regression, or performance
  failure must close `HOLD` or non-promotion without activation.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-056: Any Paradigm 2 Stage 3-Decouple producer must amend
  this contract before production edits, expose the snow-neutral
  water-temperature arm only through the internal package-bound
  `snow_stage3_liquid_routing_model = layered_thermal_liquid_v1` selector, and
  preserve the disabled path as the absent-selector rollback. The implementation
  must run with `snow_density_model = physics_bulk_density_compaction_v1`
  without requiring `physics_bulk_multilayer_density_v1`; when no Stage 1
  density layers are present, it must synthesize or carry a layer geometry whose
  per-layer densities all equal the selected bulk aggregate density and whose
  SWE/depth/density aggregates exactly reconstruct the bulk outcome. Evidence
  must prove the current bulk snow rubric is unchanged (`15` robust fails /
  `179` score), no robust cells or runoff/melt-timing signatures worsen, CoE
  melt/rain mass remains authoritative, typed meltwater-flux temperature is
  bounded, mass/liquid/energy ledgers close, no public schema/default/frost/cap/
  fixture/parser/runfile/user CLI/`.run`/full stream-temperature routing change
  occurred, and ADR-0025 H2637 performance evidence passes. Snow-rubric
  regression, Stage 1 density dependence, unphysical temperature, liquid or
  energy non-closure, protected-boundary drift, or performance failure must
  close `HOLD` without activation.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-057: Any Paradigm 2 multilayer promotion producer must amend
  this contract before production edits, keep the water-temperature arm
  default-off, and expose it only through the supported internal
  `snow_stage3_liquid_routing_model = layered_thermal_liquid_v1` selector until
  a later stream-temperature package authorizes user-facing/runfile/WEPPpy
  wiring. The implementation must publish nullable hillslope WAT parquet
  `MeltwaterTemperature` in `degC` from the typed Stage 3 meltwater flux
  temperature, with unit metadata and output-registry validation; disabled
  Stage 3 or absent routed meltwater temperature must publish null. Evidence
  must prove the real snow rubric remains exactly the current default (`15`
  robust fails / `179` score, `0` worse robust cells), rollback and fail-closed
  selector behavior remain intact, mass/liquid/energy ledgers close, finite
  produced meltwater temperatures are `<= 0 degC`, H2637 performance remains
  within ADR-0025 budget, no default-path public-schema regression occurs, no
  HBP/watershed serialization or full in-stream routing is added, and release
  notes describe the supported opt-in mode. Default activation, Stage 1 density
  dependence, unphysical temperature, liquid/energy non-closure, protected
  boundary drift, HBP/watershed serialization, user-facing selector exposure, or
  performance failure must close `HOLD`.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-058: Any frost residue-cover implementation must amend this
  contract and `SC-RESIDUE-001` before production edits, preserve the existing
  frost heat-flow formulation in `INV-SNOWFREEZE-006`, and replace only the
  residue-depth boundary consumed by `DirectFrostThermalInputs`. Evidence must
  show the residue producer publishes current dynamic `residue_depth_m` from
  surface-residue mass, frost consumes that value on the real downstream path,
  inert/no-senescence fixtures remain identity-stable against the t0 seed,
  residue mass/depth conservation closes, and the Sleepers seasonal-residue
  A-vs-B timing re-score is reported. Missing consumer proof, stale seed reuse,
  canopy leaf-on/off scope creep, Qwet changes, snow-model changes,
  public-schema drift, fixture fitting, or frost-default activation must close
  `HOLD`.
  `[DIRECT][Ran] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-059: Any SNOW-SURFACE-EB-03 producer must amend this
  contract and `SC-SNOWENERGY-001` before production edits; retain the
  supported internal Stage 3 selector as the shared thermal carrier; add
  independent default-off internal longwave and sublimation selectors; and
  preserve the existing melt, phase, density, albedo, liquid, frost, fixture,
  and public-output defaults. Evidence must reconstruct the direct-production
  `B/L/S/LS` cells, prove the same top-layer temperature/cold-content provider
  is consumed in every cell, prove signed vapor mass and latent energy derive
  from one bounded exchange, show vapor does not alias liquid/melt/refreeze,
  and close cold-content energy including exported cold content. Missing
  provider state, legacy/new sublimation double selection, polar-night cloud
  unavailability, nonclosing ledgers, or default/user/public-boundary drift
  must fail closed or keep the campaign in `HOLD`.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-060: Any SNOW-SURFACE-EB-03A producer must construct the
  upper-`0.25 m` active thermal volume independently of depositional
  boundaries, publish active/lower reconstruction and `G_0` operands, apply
  the exact libsnobal `KTS+efcon` conductivity with elevation-derived
  pressure, retain distinct active/lower temperatures across substeps, apply
  the fixed mass-dependent stability hierarchy, and prove real S/LS physical
  completion without a clamp, fitted limiter, new coefficient, CoE-melt
  replacement, or protected-path regression.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-061: Any SNOW-SURFACE-EB-04V producer must preserve the
  selected density model's state-mutation arithmetic while publishing
  `snow_density_process_diagnostics` through `SnowDensityRuntimeOutcome`,
  `DirectSnowLiquidPartition`, and the real direct-production JSONL consumer.
  Contract-derived tests must isolate fresh mixing, wet compaction,
  destructive metamorphism, overburden, structural projection, fallback, and
  cap terms; reject total-delta and post-merge fresh-density aliases; and
  reconstruct closure independently. The retained nine-lane B/L/S/LS study is
  diagnostic-only and may not fit parameters or change promotion posture.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-062: Any SNOW-SURFACE-EB-04W producer must preserve existing
  phase and CoE melt arithmetic while carrying typed hourly forcing and exact
  four-term melt diagnostics through `DirectSnowLiquidPartition` to the real
  direct-production JSONL consumer. Tests must isolate every term, distinguish
  physical snowfall depth from SWE, exercise zero/no-pack and capped melt,
  reject pure-energy and wind-residual aliases, independently reconstruct phase,
  accumulation, and melt closure, and prove prior outputs unchanged. The four
  unique lanes/16 cells and five retained chronology operators are
  diagnostic-only; they may not fit parameters, compensate forcing, or alter
  efficacy/promotion posture. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-063: Any Stage-3 liquid trace-closure producer must publish
  exact typed producer operands through additive internal JSONL schema v4 and
  carry the existing signed-hour forcing, CoE state/routing, pack state, and
  Stage-3 cold-content diagnostics without recomputation or inference. A
  package-independent parser must consume real release-CLI JSONL, reconstruct
  `incoming - routed - retained_delta - refrozen`, distinguish every named
  adjacent alias with deliberately non-equal operands, and prove protected WAT
  identity plus value identity for all pre-v4 trace fields. Producer-reported
  residuals and source-string checks are supporting evidence only. Any physics,
  selector, default, state-mutation, fixture, observation, WAT/HBP/PASS schema,
  or calibration/promotion delta is prohibited.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-064: Any durable snow mass-transition implementation must
  carry the exact upstream and downstream operands in linked compact typed
  ledgers over the single authoritative production result. It must separate
  Stage-3 enabled state, sublimation, and meltwater temperature into a compact
  production outcome; resolve the existing trace path/day/lane request before
  snow computation; and assemble the large hourly diagnostic payload only for
  selected rows. Contract-derived tests must prove both identities, exact
  handoff linkage, deliberately non-equal aliases, trace-disabled payload
  absence with closure guards retained, trace-enabled schema-v4 identity,
  ordinary output identity, migration-safe internal API adoption, bounded type
  footprint, and frozen paired runtime/RSS non-regression. A producer-only flag
  or counter cannot prove disabled-path absence or real-writer adoption.
  `[DIRECT][Static] + [INFERENCE][Ran]`
- OBL-SNOWFREEZE-P-065: Any active Anderson/SNOBAL wet-compaction producer must
  compute gross positive generated melt from the exact hourly pre-retention
  `melt_raw_m` values, add retained and released snow-contact rain once, validate
  the finite/nonnegative daily result, and carry it through one private typed
  handoff to both bulk and multilayer density consumers. Contract-derived tests
  must use deliberately non-equal melt, retained rain, released rain, state
  loss, routed handoff, and retained-store values; reconstruct the real density
  diagnostic input independently; and reject every alias named by
  `INV-SNOWFREEZE-092`. The offline CoE-bound density mirror must consume the
  same source columns rather than reconstruct the retired alias. No melt,
  retention, routing, SWE, energy, density constant/cap, selector/default,
  fixture-truth, calibration, or public-schema delta is authorized.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-066: A future melt-owner implementation must atomically
  retire CoE melt generation and make Stage 3 the real sole consumer/producer
  path. It must complete all energy components, satisfy cold content before
  bounded latent-fusion conversion, resolve the exact residual-snow phase
  boundary, define the physical recipient and next-state chronology for any
  positive energy remaining after available-ice exhaustion, route generated
  liquid through refreeze/retention/routing in the same substep, and
  independently close energy plus both linked mass ledgers.
  Selector, default, rollback, typed-failure, and real downstream-consumer
  evidence are mandatory; 21N authorizes no runtime change by itself.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-067: A future evaluation-shadow implementation must resolve
  a default-off typed request before allocating shadow state, select exactly
  one `INV-SNOWFREEZE-094` operator, attach the complete authority tag, and
  retain distinct typed custody for the clone and its ledgers. Disabled,
  absent, invalid, filtered-out, or failed evaluation must leave authoritative
  calculations, arithmetic order, state, routing, outputs, errors, defaults,
  and performance-sensitive payload allocation unchanged. Operator-specific
  tests must prove exactly two stable named/versioned paired arms, one pairing
  ID, identical state/forcing/geometry/cadence/coverage and non-formulation
  fingerprints, explicit per-arm component/NA semantics, and independent
  per-arm component-total reconstruction. A sequential physical-chronology
  claim additionally requires every state-changing forcing/process, exact
  same-substep phase/liquid chronology, and independently reconstructed
  initial-plus-flux-to-final mass, enthalpy, and liquid ledgers; without those
  proofs the evidence must be labeled only a bounded response experiment.
  Tests must also prove the bounded interval/domain, negative
  production-consumer reachability, and rejection of cross-boundary
  persistence or overbroad claims. This obligation authorizes no implementation
  by itself.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-068: The v128 evaluation realization must keep the typed
  request absent by default, construct the complete operator tag before clone
  allocation, and confine every clone and evaluation ledger to the current
  call. Same-state paired evaluation must use exactly the frozen pair/arm IDs,
  immutable initial snapshot, identical geometry/forcing/support/cadence and
  deterministic fingerprints, explicit component applicability, and
  independently reconstructable arm totals. Sequential evaluation must remain
  a bounded response experiment and publish exact truncation/coverage. Every
  new energy/mass/support field must retain units, interval/area basis, source,
  applicability, and rejected aliases through the real schema-v5 consumer.
  Typed turbulent errors retain the primitive source. Disabled schema-v4 and
  authoritative state/ledgers/outputs remain exact. `[DIRECT][Static] +
  [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-069: The v129 reconciliation producer must preserve the
  existing public API and route old and diagnostic turbulent results through
  one private solver. Only a valid enabled evaluation request may allocate the
  boxed companion or emit schema v6. It must emit every ordered applicable
  substep without sampling, truncation, or silent cap; the current resolved
  operators require at most `1,440` tuples per day. Every tuple must retain
  exact identity, interval and area basis, units, source/effective/projection
  fingerprints, explicit applicability, solver status, raw primitives,
  albedo lineage, external terms, internal active/lower conduction, and
  before/after mass and cold-content operands. Historical schema-v5 semantics
  and exact disabled schema-v4 rows remain unchanged. `[DIRECT][Static] +
  [INFERENCE][Static]`

## Consumer Obligations

- OBL-SNOWFREEZE-C-001: Infiltration/runoff consumers treat redistributed `hrmelt`/daily `wmelt` as event liquid forcing with the same rigor as rainfall forcing. WB12 must expose meltwater to Green-Ampt infiltration capacity before assigning the residual to runoff. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-002: Daily water-balance consumer treats melted snow as rainfall contribution in Eq. [5.1.1] accounting semantics. `[DIRECT][Static]`
- OBL-SNOWFREEZE-C-003: Soil/erosion-related consumers receiving frost outputs (`Dfrost`, `Dthaw`, `Nft`, `Ws_frz`, `InfCap_frz`) must fail explicitly on missing or invalid winter payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-004: Consumers propagate invariant-violation context as typed errors without silent clamping/defaulting. `[INFERENCE][Static]`
- OBL-SNOWFREEZE-C-005: Runoff and storage reconciliation consumers must apply signed `S` coupling semantics and reject active-coupling payloads missing required `snow.options.*` controls. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-006: Output/publication consumers must reject static-control substitution where snow-storage publication aliases (`Snow-Water`, hydout-equivalent snow-water surfaces) fail runtime SWE derivation checks. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-007: Consumer implementations must preserve the baseline `wmelt -> fin/smrate -> layer ingress -> infiltration/runoff residual` lineage from `watbal_hourly.for` and `grna.for`; melt-only runoff shortcuts or publication-only storage compensation are invalid. `[DIRECT][Static]`
- OBL-SNOWFREEZE-C-008: WB12/WB14/WB13 consumers must reject domain-invalid runtime snow state before treating snow coupling as inactive or publishing zero snow storage. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-009: Production snow, water balance, infiltration/runoff,
  frost, output, calibration, configuration, and assurance-promotion consumers
  must reject `snow_evaluation_shadow_state` and its ledgers as input or
  authority. Only the tagged evaluation evidence consumer may read them, and
  it must preserve operator, support, coverage, custody, unresolved-boundary,
  and claim-class metadata without relabeling shadow results as production.
  `[INFERENCE][Static]`
- OBL-SNOWFREEZE-C-010: The internal schema-v5 evaluation consumer must read
  every v128 operator/tag, fingerprint, component, mass, support, coverage, and
  residual field; independently reconstruct both paired totals and the bounded
  sequential energy identity; reject production aggregates, CoE melt, producer
  residuals, row count, nonzero-hour count, and ground-flux aliases; and retain
  the unresolved snow-ground and terminal boundaries. No production or public
  consumer may read schema-v5 evidence as state or authority. `[INFERENCE][Static]`
- OBL-SNOWFREEZE-C-011: The real internal reconciliation consumer must dispatch
  schema v5 and v6 explicitly and reject unknown or malformed versions. For v6
  it must independently reconstruct each turbulent, radiation,
  precipitation-advection, vapor, mass, cold-content, and duration-integrated
  term from primitive operands without importing producer calculation helpers
  or accepting producer totals as proof. It must enforce exact daily and
  hourly identities, non-overlapping ordered substeps, source/forcing/geometry
  custody, common-support integration, the legacy mixed/external/conduction
  bridge, and the frozen `S/F/Q` projection-versus-evolution decomposition;
  unmatched, truncated, non-applicable, and censored evidence stays explicit.
  No production or public consumer may read schema-v6 evidence as state or
  authority. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Melt bounds and density gate (`INV-SNOWFREEZE-001/002`) | melt post-processing and pre-routing checks | Explicit branch applies melt bounds; hard error only if post-branch state remains invalid or if density gate is violated | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Opt-in winter-thaw state-loss correction (`INV-SNOWFREEZE-066`) | typed CoE melt selector, low-density positive-thaw application branch, snowbench replay, and paired event-window adjudication | Hard error on default drift, invalid snow-state closure, albedo dependency, missing operand reconstruction, or closure without paired event-window improvement; otherwise opt-in candidate remains diagnostic until later activation | SNOWDENSITY-10.3.7 thaw-response gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Opt-in liquid holding-capacity drainage correction (`INV-SNOWFREEZE-067`) | typed CoE melt selector, retained-liquid state, capacity-bound low-density positive-liquid branch, snowbench replay, and paired/coupled adjudication | Hard error on default drift, invalid snow-state closure, unbounded liquid release, missing persistent retained-liquid evidence, missing operand reconstruction, or closure without paired event-window and coupled WAT evidence; otherwise opt-in candidate remains diagnostic until later activation | SNOWDENSITY-10.3.8 capacity-drainage gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Opt-in spring compaction/densification candidate (`INV-SNOWFREEZE-068`) | typed density selector, CoE-boundary SWE identity, wet-compaction substep realization, and coupled WAT adjudication | Hard error on default drift, SWE drift, density above `522 kg m^-3`, observed-depth fitting, missing trace proof, or any paired-surface snow-control worsening; otherwise candidate remains diagnostic until later activation | SNOWDENSITY-10.3.11 spring compaction gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Combined opt-in snow-depth bundle activation adjudication (`INV-SNOWFREEZE-069`) | explicit package-bound melt+density selector pair, direct-production WAT trace proof, residual classification, and Policy-B workspace-suite/conservation evidence | Hard error on missing direct trace proof, missing paired snow-control evidence, default activation without strict gate-eligible snow improvement, or default activation without workspace-suite/conservation evidence; remaining paired snow residuals block frost attribution separately and drive classified follow-ons | SNOWDENSITY-10.3.12 bundle activation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Residual-tail and Policy-B diagnostic (`INV-SNOWFREEZE-070`) | date-level residual transition report, cap-classification matrix, and Policy-B workspace-suite/conservation evidence matrix | Hard error on missing real WAT lineage, aggregate-only under-persistence attribution, hidden density-cap change, or activation/frost-attribution claim from diagnostic evidence alone; otherwise follow-ons target classified residuals and missing Policy-B scopes | SNOWDENSITY-10.3.13 residual diagnostic gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Policy-B no-regression and cap-authority diagnostic (`INV-SNOWFREEZE-071`) | package-bound bundle selectors, workspace-suite no-regression gate, composite trace state identity, active-cap bounds, and same-SWE cap-pinned projection | Hard error on hidden default/cap mutation, missing trace/WAT lineage, missing workspace-suite/conservation gate status, trace identity or cap-bound failure, claiming `550 kg m^-3` dynamic readiness from projection only, or claiming frost attribution; otherwise active-cap activation may proceed only through a separate package with default/rollback gates | SNOWDENSITY-10.3.14 Policy-B/cap gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Default activation under active cap (`INV-SNOWFREEZE-072`) | no-env direct-production selector path, rollback/test selector path, WAT trace proof, output/user-surface scan, workspace gates, and downstream snow-affected delta documentation | Hard error on no-env legacy fallback, unsupported candidate acceptance, missing rollback, hidden parser/runfile/user CLI exposure, output-schema drift, density-cap mutation, missing real downstream WAT/trace evidence, or frost-attribution claim while `498/1415` paired snow-depth failures remain; otherwise the active-cap bundle is the direct-production default with explicit rollback/test selectors | SNOWDENSITY-10.3.15 default-activation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Open-surface ablation Stage A (`INV-SNOWFREEZE-073`) | explicit opt-in melt selector, snow-sublimation vapor ledger, coupled WAT/trace residual classification, and snow-state conservation | Hard error on default activation, unsupported selector fallback, fixture-tuned constants, sublimation routed as liquid, missing vapor ledger, missing coupled WAT/trace evidence, open-surface tail non-improvement, under-persistence worsening, out-of-range sublimation magnitude, conservation failure, schema/fixture/default/cap/frost drift, or two-layer-surface scope creep; otherwise candidate remains opt-in diagnostic only | SNOWDENSITY-10.3.16 Stage A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Harder-Pomeroy phase default activation (`INV-SNOWFREEZE-075`) | no-env direct-production phase selector path, explicit `legacy_rst` rollback/test path, cross-SNOTEL forcing-robust rubric rerun, active-hour precipitation reconstruction, and workspace-suite/conservation gates | Hard error on no-env legacy phase fallback, missing rollback, unsupported selector acceptance, hidden parser/runfile/user CLI or `.run` control exposure, fixture/schema/cap/frost drift, cross-SNOTEL robust rubric worse than prior activated bundle, partition mass non-closure, missing workspace-suite evidence, or promotion based on humid-New-England depth alone; otherwise Harder-Pomeroy hourly phase is the direct-production default with explicit `legacy_rst` rollback | SNOWDENSITY-10.3.19 phase-default gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Sublimation diagnosis and Stage B unlock (`INV-SNOWFREEZE-076`) | Stage A degradation diagnosis, partition+sublimation composition score, Stage B opt-in selector, active surface-layer temperature/cold-content gate, cross-SNOTEL rubric, vapor and phase conservation | Hard error on default/rollback drift, hidden parser/runfile/user CLI or `.run` control exposure, fixture/schema/cap/frost drift, unsupported selector acceptance, fixture-tuned threshold/constant, unproven libsnobal license/provenance after reading C, sublimation routed as liquid, missing site/signature/magnitude diagnosis, missing real cross-SNOTEL WAT/trace evidence, promotion without beating the current default on robust score/fail count, bidirectional guardrail failure, or conservation failure; otherwise candidates remain opt-in/non-promoted unless the primary gate is won | SNOWDENSITY-10.3.20 sublimation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Climate-class density specialization (`INV-SNOWFREEZE-077`) | explicit opt-in density selector, Sturm 1995 forcing-derived class assignment, Sturm 2010 parameter coverage, fallback flags, cross-SNOTEL rubric, bidirectional densification flip, persistence guardrail, and conservation | Hard error on default/rollback drift, hidden parser/runfile/user CLI or `.run` control exposure, geographic/site/observed-residual class assignment, invented or fixture-fitted thresholds/parameters, unsupported class parameter use, missing fallback flag, missing real cross-SNOTEL WAT/trace evidence, failure to beat current default, missing bidirectional densification flip, new persistence tail, conservation failure, or validation claims for absent classes; otherwise candidate remains opt-in/non-promoted unless the primary gate is won | SNOWDENSITY-10.3.22 climate-class gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Paradigm 2 Stage 1 layered snow-density candidate (`INV-SNOWFREEZE-078`) | explicit opt-in density selector, persistent winter-column `snow_layers`, local-overburden compaction, aggregate layer closure, cross-SNOTEL rubric, bidirectional densification flip, persistence guardrail, conservation, and performance evidence | Hard error on default/rollback drift, hidden parser/runfile/user CLI or `.run` control exposure, fixed-slot layer projection, fixture/site fitting, density-cap drift, output-schema drift, frost/melt/liquid/phase/canopy/radiation drift, missing real consumer persistence proof, invalid layer aggregate closure, missing rubric evidence, failure to beat current default, missing bidirectional densification flip, new persistence tail, conservation failure, or missing performance evidence; otherwise candidate remains opt-in/non-promoted unless the primary gate is won | Paradigm 2 Stage 1 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Paradigm 2 Stage 2 snow-frost insulation-profile candidate (`INV-SNOWFREEZE-079`) | explicit opt-in frost-insulation selector, Stage 1 density-gradient entry proof, Sturm et al. density-to-conductivity relation, layer-stack resistance closure, equivalent-density projection, frost observation rubric, snow no-regress, conservation, and performance evidence | Hard error on default/bulk rollback drift, unsupported selector fallback, hidden parser/runfile/user CLI or `.run` control exposure, missing gradient proof, missing/non-finite/out-of-domain resistance or equivalent density, public schema drift, fixture/site fitting, density-cap drift, same-day snow-to-frost timing breach, Stage 3 thermal/melt/liquid scope creep, frost rubric non-improvement, snow-rubric regression, conservation failure, or missing performance evidence; otherwise candidate remains opt-in/non-promoted unless the frost-primary gate is won | Paradigm 2 Stage 2 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Paradigm 2 Stage 3 per-layer liquid-routing and meltwater-temperature candidate (`INV-SNOWFREEZE-080`) | explicit opt-in liquid-routing selector, Stage 1 layer-stack consumer proof, Stage 0 energy primitive use, per-layer cold-content/refreeze/liquid routing, typed meltwater-flux temperature, mass/liquid/energy closure, runoff-timing no-worse evidence, snow no-regress guardrail, and performance evidence | Hard error on default/disabled rollback drift, unsupported selector fallback, hidden parser/runfile/user CLI or `.run` control exposure, missing layer state with snow present, energy-balance melt replacing CoE routed melt, unphysical layer or meltwater temperatures, unbounded retained liquid/refreeze, public schema drift, fixture/site fitting, full stream-temperature routing, mass/liquid/energy non-closure, runoff-timing regression, snow-rubric regression, or missing performance evidence; otherwise candidate remains opt-in/non-promoted unless Stage 3 gates are won | Paradigm 2 Stage 3 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Snow depth/density domain (`INV-SNOWFREEZE-003/004`) | hourly snowpack state update | Hard error on domain/branch inconsistency | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Partition and activation branches (`INV-SNOWFREEZE-005/009`) | daily/hourly branch selection | Hard error on branch mismatch or silent bypass | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Frost heat-flow semantics (`INV-SNOWFREEZE-006`) | frost routine bookkeeping | Hard error on invalid equation domain; investigate hourly-heavy deltas per ADR confidence tiers | Tier-B investigation gate | `[DIRECT][Static]` |
| Dynamic frost residue-cover coupling (`INV-SNOWFREEZE-083`) | frost thermal input construction and downstream frost trace | Hard error on missing/non-finite/negative/stale dynamic residue-depth payload, seed-depth reuse after residue mass changes, missing residue conservation evidence, or missing real consumer proof; otherwise frost consumes the current residue depth while retaining existing heat-flow equations | Frost residue-cover implementation gate | `[DIRECT][Ran] + [INFERENCE][Static]` |
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
| Opt-in CoE melt implementation (`INV-SNOWFREEZE-055`) | Typed snowmelt selector, active-snow albedo carry, and routed liquid-forcing reconstruction | Hard error for missing active opt-in state; default path identity required; governance `HOLD` unless raw melt, redistributed melt, routed `wmelt`, SWE loss, WB12 `S`, and WB13 liquid forcing reconstruct from typed operands without storage aliasing | SNOWDENSITY-05D and successors | `[DIRECT][Static] + [INFERENCE][Static]` |
| Melt closure and density handoff (`INV-SNOWFREEZE-056`) | Contract, source scan, package closure, and future density/activation package review | Governance `HOLD` for default activation; density work may consume `coe_shortwave_albedo_v1` only as a fixed opt-in boundary with no melt/radiation retuning, preserved rollback, cold-start albedo continuity, 05E regime-limited evidence labeling, real-canopy/native-or-proven-radiation harness fidelity entry gate, and both 05E diagnostic replay plus H as-built context in activation evidence | SNOWDENSITY-05F and successors | `[DIRECT][Static] + [INFERENCE][Static]` |
| Drift activation governance (`INV-SNOWFREEZE-008`) | review/disposition/promotion gate | Governance `HOLD` until active-implementation authority is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Density-process observability (`INV-SNOWFREEZE-087`) | typed density runtime, direct snow partition, and real JSONL consumer | Typed hard error on non-finite fields or failed closure; governance `HOLD` on inapplicability ambiguity, total-delta aliasing, mutation drift, or calibration/promotion use | EB-04V diagnostic gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Accumulation/melt-driver observability (`INV-SNOWFREEZE-088`) | typed forcing/melt runtime, direct snow partition, and real JSONL consumer | Typed hard error on non-finite/out-of-domain fields or failed phase/component closure; governance `HOLD` on snowfall depth/SWE aliasing, pure-energy relabeling, physical-redistribution inference, mutation drift, or calibration/promotion use | EB-04W diagnostic gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Typed-snow activation and daily storage closure (`INV-SNOWFREEZE-089`) | shared typed snow partition and direct/snowbench consumers | Positive typed snowfall forces active coupling; independently reconstructed daily SWE residual above `TOL-SNOWFREEZE-006` returns the existing typed domain/closure error | EB-04W2B warm-snow, mixed-event, and all-rain vectors | `[DIRECT][Static] + [INFERENCE][Static]` |
| Stage-3 liquid and signed-hour trace closure (`INV-SNOWFREEZE-090`) | typed CoE/Stage-3 diagnostics, direct snow partition, schema-v4 JSONL writer, and independent real-file parser | Typed hard error on existing producer domain/closure violations; governance `HOLD-EVIDENCE` on omitted/aliased operands, consumer bypass, nonclosing independent identity, protected-output drift, or any behavior mutation | SNOW-STAGE3-LIQUID-SIGNED-HOUR-TRACE-CLOSURE real Snowbird trace gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Durable linked snow mass-transition ledgers (`INV-SNOWFREEZE-091`) | compact solid-to-liquid ledger, exact Stage-3 handoff, compact liquid-disposition ledger, compact Stage-3 outcome, typed verbose-capture request, real schema-v4 writer, and independent compact/file consumers | Existing typed physical/closure failures remain hard errors; governance `HOLD` on duplicate mass authority, inferred/aliased operands, lost production outcome, eager disabled-path payload, trace/output drift, unauthorized API break, confirmed package-bound performance regression, or any physics/state mutation | SNOW-MASS-TRANSITION-LEDGER-PERSISTENCE baseline/candidate compatibility, reconstruction, footprint, and performance gates | `[DIRECT][Static] + [INFERENCE][Ran]` |
| Wet-compaction operand authority (`INV-SNOWFREEZE-092`) | exact hourly generated-melt and rain finalization, private density handoff, bulk/multilayer consumer, density diagnostic, and CoE-bound offline mirror | Typed hard error on non-finite/negative input or failed reconstruction; defect closure is blocked if the current duplicate, routed-only, state-loss-plus-rain, retained-store, or any other adjacent alias reaches a real consumer | SNOW-WETCOMPACT-DUP-001 correction and anti-alias gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Stage 3 sole future melt ownership (`INV-SNOWFREEZE-093`) | complete same-substep energy, cold-content, bounded phase-change, refreeze/retention/routing, residual-snow, linked-ledger, selector/default/rollback, and real-consumer surfaces | Hard `IMPLEMENTATION_HOLD` until every surface closes atomically; after cutover, typed failure on incomplete/duplicate energy, dual melt owners, unavailable-ice conversion, residual-snow proxy, delayed/aliased liquid, or nonclosing ledgers | future Stage 3 melt-owner implementation and cutover gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Stage 3 evaluation-shadow authority (`INV-SNOWFREEZE-094`) | typed request/tag resolution, exact paired-arm fingerprints/reconstruction, complete sequential forcing and clone-ledger closure, clone custody, operator boundary, negative production-consumer scan, and evidence claim audit | Evaluation hard-fail on invalid enabled metadata or aliasing; governance `HOLD` on unequal/ambiguous paired arms, incomplete chronology/closure, untagged/production-readable state, cross-boundary carry, unsupported operator, overbroad claims, or runtime/default/public mutation | evaluation-shadow contract and future implementation/noninterference gates; no cutover evidence | `[DIRECT][Static] + [INFERENCE][Static]` |
| Stage 3 evaluation observability (`INV-SNOWFREEZE-095`) | typed request resolver, extracted evaluator, schema-v5 producer, and independent real-file consumer | Evaluation hard-fail on invalid/conflicting request, fingerprint or component nonclosure, incomplete coverage metadata, or typed primitive failure; governance `HOLD` on schema-v4 drift, production alias/reachability, cross-boundary carry, ground-flux relabeling, or claims beyond the selected operator | shadow-observability contract, extraction-parity, real-consumer reconstruction, anti-alias, noninterference, and typed-error gates; no seasonal, terminal, or cutover evidence | `[DIRECT][Static] + [INFERENCE][Static]` |
| Stage 3 turbulent carrier and operator reconciliation (`INV-SNOWFREEZE-096`) | shared private turbulent solver, enabled-only boxed companion, schema-v6 tuple writer, and independent version-aware consumer | Evaluation hard-fail on typed solver error, incomplete/malformed tuple, invalid identity/order/duration/applicability, fingerprint mismatch, primitive or endpoint nonclosure, or unsupported schema; governance `HOLD` on duplicated physics, v4/v5 drift, internal/external aliasing, producer-only reconstruction, support laundering, or claims beyond operator mechanics | solver-parity, schema compatibility, primitive reconstruction, state-custody, common-support `S/F/Q`, predecessor-bridge, noninterference, and four-site evidence gates; no physical-seasonal, terminal, production, or cutover evidence | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| TOL-SNOWFREEZE-007 | Frost-depth magnitude acceptance band (frost-tube authority; `INV-SNOWFREEZE-047`) | accepted: greater of `0.10 m` or `25 %` of observed seasonal-maximum depth | Observation-evaluation band, not a comparator FP tolerance or calibration target. Bounded below by frost-tube read/registration uncertainty and ratified with the 2026-06-29 frost-observation arc. |
| TOL-SNOWFREEZE-008 | Frost onset/thaw timing and frozen-duration acceptance band (`INV-SNOWFREEZE-047`) | accepted: `+/- 14 days` | Bounded by observation cadence (frost tubes/penetrometer read 2-4x/month to biweekly), so sub-fortnight timing is unresolvable from the data; not a runtime tolerance. This is the only Step 1 timing threshold ratified; the `>0.25` systematic-fraction cutoff remains diagnostic-local. |
| TOL-SNOWFREEZE-009 | Snow-insulation confound-control band for attributing a frost-depth divergence (`INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`) | accepted: paired modeled-vs-observed snow depth within greater of `0.10 m` or `30 %` | Beyond this band the frost-depth comparison routes through `INV-SNOWFREEZE-050`: magnitude-only snow residuals may be forcing-limited/report-only, while forcing-robust timing/regime/slope defects block frost attribution. Snow depth and density, not SWE, govern insulation. This band is not a snow-model calibration target; `INV-SNOWFREEZE-048` must first prove like-for-like snow-depth correspondence and anti-alias evidence. |
| TOL-SNOWFREEZE-010 | SNOTEL SWE/density fork-adjudication band (`INV-SNOWFREEZE-049`, `INV-SNOWFREEZE-050`) | accepted rubric band: SWE within greater of `0.05 m` water equivalent or `25 %`; density within greater of `60 kg m^-3` or `25 %` | Characterization band for separating over-accumulation from low-density/settling structure in the observed-data rubric. It is not a calibration objective and does not authorize SSD residual fitting, production constant edits, or PySnobal/legacy-as-target decisions; `INV-SNOWFREEZE-049` remains the anti-tuning correspondence rule. |
| TOL-SNOWFREEZE-011 | Snow/frost rubric cell pass-levels and forcing-robustness tiering (`INV-SNOWFREEZE-050`) | accepted noise-floor: forcing-robust (`R`) cells score `pass` at KGE `>= 0.6` (correlation, bias-ratio, variability-ratio each near 1) and `marginal` at `0.3-0.6`; timing cells within `+/- 14 days`; density/regime per `TOL-SNOWFREEZE-010`. Forcing-limited (`L`) magnitude cells are reported only - a peak SWE/depth median bias up to `~30 %` is uncertainty-consistent, not a defect. | Evaluation bands, not calibration objectives. Ratified with the 2026-06-29 frost-observation arc; future package-specific refinements must declare authority and sensitivity rather than tuning to fixture residuals. |
| TOL-SNOWFREEZE-012 | Daily density-process diagnostic closure (`INV-SNOWFREEZE-087`) | `abs(final_density - initial_density - sum(signed density increments)) <= 1e-9 kg m^-3` | Trace/reconstruction tolerance only; it does not authorize state canonicalization, coefficient fitting, or density-numeric changes. |
| TOL-SNOWFREEZE-013 | Hourly phase and CoE melt-component diagnostic closure (`INV-SNOWFREEZE-088`) | `abs(active_precipitation - rain - snowfall_swe) <= 1e-12 m`; `abs(accumulation - sum(hourly snowfall_swe)) <= 1e-12 m`; `abs(uncapped_raw_melt - sum(four components)) <= 1e-12 m`; `abs(applied_raw_melt - sum(four components) - cap_adjustment) <= 1e-12 m`; and `abs(daily_raw_melt - sum(hourly applied_raw_melt)) <= 1e-12 m` | Trace/reconstruction tolerance only; it does not authorize canonicalization, fitting, or process changes. |
| TOL-SNOWFREEZE-014 | Typed precipitation/snowfall presence thresholds (`INV-SNOWFREEZE-089`) | finite daily precipitation `> 1e-12 m` is independently sufficient to resolve production phase rows; finite physical `hourly snowfall_m > 1e-12 m` is independently sufficient to activate snow coupling; exactly `1e-12 m` remains sub-resolution | Control thresholds only; retained snow, frost, and cold-forcing provider triggers remain authoritative and may also require phase-row resolution. The thresholds do not alter or canonicalize a phase provider's precipitation or snowfall value. |
| TOL-SNOWFREEZE-015 | Stage-3 published liquid-closure reconstruction (`INV-SNOWFREEZE-090`) | `abs(incoming_liquid_m - routed_liquid_m - retained_liquid_delta_m - refrozen_liquid_m - liquid_closure_residual_m) <= 1e-9 m` | Matches the existing Stage-3 runtime closure guard. Trace/reconstruction tolerance only; it does not authorize clamping, canonicalization, inferred operands, or process changes. |
| TOL-SNOWFREEZE-016 | Solid-to-liquid compact-ledger reconstruction (`INV-SNOWFREEZE-091`) | `abs(liquid_handoff_m - snowpack_swe_loss_m - rain_released_m) <= 1e-9 m` | Matches the existing snow-partition arithmetic and daily snow-storage tolerance. Compact-ledger/reconstruction tolerance only; raw signed melt and redistributed positive melt remain distinct context and cannot be substituted for bounded authoritative state loss. |
| TOL-SNOWFREEZE-017 | Wet-compaction daily operand reconstruction (`INV-SNOWFREEZE-092`) | `abs(wet_compaction_liquid_input_m - sum(max(hourly melt_raw_m, 0)) - rain_retained_m - rain_released_m) <= 1e-12 m` | Exact-source reconstruction tolerance only. It does not authorize clamping, inferred routing/storage aliases, coefficient changes, or a second density input. |
| TOL-SNOWFREEZE-018 | Schema-v6 tuple reconstruction and endpoint closure (`INV-SNOWFREEZE-096`) | flux: `max(1e-10 W m^-2, 1e-12 * sum_abs_operands)`; mass: `max(1e-12 kg m^-2, 1e-12 * sum_abs_operands)`; energy/cold: `max(1e-6 J m^-2, 1e-12 * sum_abs_operands)` | Consumer reconstruction only. It does not authorize producer correction, canonicalization, dropped tuples, or replacement of exact same-state `to_bits` endpoint equality. |
| TOL-SNOWFREEZE-019 | Operator-reconciliation sign, predecessor, and support classification (`INV-SNOWFREEZE-096`) | absolute zero/sign threshold `1e-6 J m^-2`; Snowbird predecessor median tolerance `1e-7 MJ m^-2`; each contributing water-year bridge uses `max(1e-6 J m^-2, 1e-12 * sum_abs_operands)`; support materiality ratio threshold `0.05` | Frozen classification tolerances, not physics-fit thresholds. Exact-zero denominator makes the support ratio N/A; it is never coerced to zero. |

## Stage 3 Evaluation Shadow Authority Addendum

The two operators are distinct experimental designs and may not be silently
substituted:

| Rule | `same_state_paired_carrier_v1` | `sequential_resolved_shadow_v1` |
|---|---|---|
| State source | one named authoritative snapshot shared exactly by two stable named/versioned arms under one pairing ID | one named authoritative snapshot cloned once |
| State evolution | prohibited | clone-only, inside one continuous declared interval and resolved domain |
| End state | discarded after the paired evaluation | discarded at the interval/domain boundary unless later persistence authority exists |
| Required equality/closure | identical snapshot, forcing, `z_T/z_q/z_u/z_0,aero`, cadence, coverage, and non-formulation fingerprints; per-arm component/NA declaration and independent component-total reconstruction | all state-changing forcing/process chronology plus independent initial+flux-final mass, enthalpy, and liquid closure for a physical chronology claim |
| Admitted inference | same-support carrier and flux-component comparison | physical bounded within-interval resolved-state chronology only with the required completeness/closure; otherwise bounded response experiment |
| Explicitly inadmissible | state trajectory, efficacy, seasonal chronology, promotion, cutover | cross-interval chronology, accumulation season, peak/meltout/restart, promotion, cutover |

Contract-derived vectors must include: disabled/absent request with no shadow
allocation; exact two-arm allow-list/cardinality; two stable named/versioned
carriers on byte-identical snapshot/forcing/geometry/cadence/coverage and
non-formulation fingerprints; per-arm component-total reconstruction with
explicit not-applicable components; deliberate paired-operand mismatch and
third/generic-operator rejection; bounded sequential evolution; incomplete
state-changing forcing, same-substep liquid, and mass/enthalpy/liquid closure
rejection or bounded-response relabeling; cross-boundary carry rejection;
untagged and incomplete-tag rejection; attempted production
consumer, routing, publication, selector, calibration, or default use
rejection; evaluator failure with unchanged authoritative outcome; and a claim
audit that rejects seasonal, terminal, efficacy, promotion, and cutover
language from daily or independently reinitialized evidence. Producer-only
tag emission cannot close negative consumer reachability.

This addendum is authority for later implementation packages, not evidence
that either operator is currently implemented.

## Stage 3 Turbulent Carrier And Operator Reconciliation Addendum

This addendum is the canonical algorithm and schema authority for
`INV-SNOWFREEZE-096`. It is subordinate to the evaluation custody of
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-085/086/094/095`; current carrier authority
`SC-SNOWENERGY-001#INV-SNOWENERGY-015/016/017/019/020/021/022/023/025/026/031`;
the unresolved meltout/terminal boundaries of
`SC-SNOWENERGY-001#INV-SNOWENERGY-029`; and the same-substep liquid chronology
required by `SC-SNOWENERGY-001#INV-SNOWENERGY-030`. It exposes existing
calculations only; it does not satisfy either snow-energy invariant or admit a
physical seasonal trajectory.

### Exact schema-v6 surface and unit basis

The JSONL row inherits receipt-bound `site_id` and its existing top-level
`day_index` and `lane_index`; every tuple adds `operator_id`, `hour_index`,
`substep_index`, `elapsed_start_seconds`, `requested_seconds`,
`evaluated_seconds`, `duration_seconds`, `applicable`, and
`applicability_reason`. Fingerprint and projection custody consists exactly of
`source_fingerprint_fnv1a64`, `forcing_fingerprint_fnv1a64`,
`geometry_fingerprint_fnv1a64`, `effective_input_fingerprint_fnv1a64`, and
`projection_id`.

Control-volume and endpoint fields are exactly:

- `active_layer_prefix_count_before`, `total_layer_count_before`,
  `active_layer_state_fingerprint_before_fnv1a64`, and
  `total_layer_state_fingerprint_before_fnv1a64`;
- `active_layer_prefix_count_after`, `total_layer_count_after`,
  `active_layer_state_fingerprint_after_fnv1a64`, and
  `total_layer_state_fingerprint_after_fnv1a64`;
- `after_surface_applicable` and `after_surface_applicability_reason`;
- `active_ice_mass_before_kg_m2`, `active_ice_mass_after_kg_m2`,
  `total_ice_mass_before_kg_m2`, and `total_ice_mass_after_kg_m2`;
- `active_depth_before_m`, `active_depth_after_m`,
  `active_density_before_kg_m3`, and `active_density_after_kg_m3`;
- `active_cold_before_j_m2`, `active_cold_after_j_m2`,
  `total_cold_before_j_m2`, and `total_cold_after_j_m2`; and
- `surface_temperature_before_c` and `surface_temperature_after_c`.

Forcing, radiation, and albedo lineage fields are exactly
`air_temperature_c`, `dewpoint_c`, `wind_speed_m_s`, `air_pressure_pa`,
`hourly_radiation_mj_m2`, `daily_solar_radiation_mj_m2`,
`daily_extraterrestrial_radiation_mj_m2`, `daylight`,
`canopy_cover_fraction`, `rain_m`, `snowfall_geometric_m`,
`rain_mass_flux_kg_m2_s`, `snow_mass_flux_kg_m2_s`, `rain_temperature_c`,
`snow_temperature_c`, `rain_specific_heat_j_kg_k`,
`snow_specific_heat_j_kg_k`, `incoming_shortwave_w_m2`,
`snow_albedo_fraction`, `snow_albedo_source_id`, `snow_albedo_model_id`,
`snow_albedo_accumulated_positive_temperature_c_day`,
`net_shortwave_w_m2`, `actual_vapor_pressure_pa`,
`longwave_cloud_fraction`, `sky_view_fraction`,
`atmospheric_longwave_w_m2`, `canopy_longwave_w_m2`,
`subcanopy_longwave_w_m2`, `outgoing_longwave_w_m2`, and
`net_longwave_w_m2`; the exact selected operator lineage fields are
`longwave_model_id` and `sublimation_model_id`. `snow_albedo_source_id` is exactly
`snow_albedo_state` when state exists or
`stage3_default_snow_albedo_0p82` otherwise; model and accumulated-positive-
temperature fields are null only for that fallback.

Turbulent geometry and primitive fields are exactly
`air_temperature_height_m`, `vapor_pressure_height_m`,
`wind_speed_height_m`, `aerodynamic_roughness_length_m`,
`turbulent_max_iterations`, `turbulent_convergence_tolerance`,
`surface_vapor_pressure_pa`, `air_potential_temperature_k`,
`surface_temperature_k`, `specific_humidity_air_kg_kg`,
`specific_humidity_surface_kg_kg`, `air_density_kg_m3`,
`displacement_height_m`, `log_momentum`, `log_sensible`, `log_latent`,
`turbulent_termination_status`, `stability_class`, `obukhov_length_m`,
`psi_momentum`, `psi_sensible`, `psi_latent`, `turbulent_iterations`,
`friction_velocity_m_s`, `sensible_exchange_velocity_m_s`,
`latent_exchange_velocity_m_s`, `surface_latent_heat_j_kg`,
`vapor_mass_flux_kg_m2_s`, `sensible_flux_w_m2`, and `latent_flux_w_m2`.
The height fields are the virtual instruments `z_T`, `z_q`, `z_u`, and
`z_0,aero`; bare `z_0` remains a rejected alias because it names the Stage 3
thermal active depth elsewhere.

Transfer and endpoint-ledger fields are exactly
`precipitation_advected_flux_w_m2`, `complete_external_flux_w_m2`,
`vapor_mass_exchange_kg_m2`, `sublimation_kg_m2`, `deposition_kg_m2`,
`melt_kg_m2`, `active_cold_energy_change_j_m2`,
`lower_cold_energy_change_j_m2`, `cold_content_export_j_m2`,
`internal_active_lower_conduction_j_m2`,
`legacy_sequential_complete_j_m2`, and `energy_closure_residual_j_m2`.

Instantaneous temperatures use `degC` except the two explicit `*_k` fields;
`snow_albedo_accumulated_positive_temperature_c_day` uses `degC day`; pressure uses
`Pa`; lengths use `m`; velocities use `m s^-1`; density uses `kg m^-3`;
specific humidity uses `kg kg^-1`; heat capacity uses `J kg^-1 K^-1`;
latent heat uses `J kg^-1`; energy flux uses `W m^-2`; energy uses `J m^-2`;
mass uses `kg m^-2`; and mass flux uses `kg m^-2 s^-1`. Radiation source
fields retain `MJ m^-2` per hourly bin or `MJ m^-2 d^-1` as named. Positive
energy and vapor mass point toward snow. Exact conversions are
`incoming_shortwave_w_m2 = hourly_radiation_mj_m2 * 1e6 / 3600`,
`rain_mass_flux = rain_m * 1000 / 3600`, and
`snow_mass_flux = snowfall_geometric_m * 0.1 * 1000 / 3600`; dividing hourly
precipitation by substep duration is prohibited. Energy and mass integrate as
`sum(flux * duration_seconds)`.

These fields form one evaluation-only diagnostic scalar/vector exception to
the production boundary registry. They are not production state/flux symbols,
cannot reach a production/public consumer, and retain typed meteorology units
at the carrier boundary plus unit-bearing field names and independent
dimensional reconstruction at JSON egress.

### Applicability, ordering, and state closure

An evaluated tuple has `applicable=true`, reason `evaluated`, positive finite
`duration_seconds`, and finite values for every selected-equation field. A
non-evaluated hour has no tuple and exactly one hourly reason:
`no_resolved_snow_at_day_start`, `thin_pack_boundary_reached`, or
`operator_not_selected`. Same-state emits one `3600 s` tuple per evaluated
hour. Sequential emits every existing dynamic substep, with no diagnostic cap
or sampling; the current maximum is `24 * 60 = 1,440` tuples per day.
Substeps are ordered by hour then index, start at zero, are contiguous and
non-overlapping, and stay inside `[0, 3600]`.

JSON `null`, never numeric zero, represents N/A state, endpoint, stability, or
transfer. After a substep removes the resolved active surface,
`after_surface_applicable=false` with reason
`post_substep_no_resolved_surface`; every active after-surface field/count/hash
is null, while `total_layer_count_after`, `total_ice_mass_after_kg_m2`,
`total_cold_after_j_m2`, and the empty total-state fingerprint remain
applicable. Otherwise the reason is `resolved_surface`. Same-state before and
after state, counts, and fingerprints require exact `to_bits` equality.

For every sequential tuple the independent consumer enforces:

```text
total_ice_mass_after_kg_m2
  = total_ice_mass_before_kg_m2 - melt_kg_m2 - sublimation_kg_m2
    + deposition_kg_m2
total_cold_after_j_m2
  = total_cold_before_j_m2 - active_cold_energy_change_j_m2
    - lower_cold_energy_change_j_m2 - cold_content_export_j_m2
legacy_sequential_complete_j_m2
  = complete_external_flux_w_m2 * duration_seconds
    + internal_active_lower_conduction_j_m2
```

Active and lower internal-conduction contributions cancel in total cold
closure; neither may be called snow-ground heat. Reconstruction uses
`TOL-SNOWFREEZE-018` and enabled malformed/nonclosing evidence fails the
evaluation request without authoritative mutation.

### Turbulent termination and reconstruction

The shared private Monin-Obukhov solver preserves these exact exits:

| `turbulent_termination_status` | Iterations / Obukhov length | `stability_class` and retained state |
|---|---|---|
| `zero_wind` | `0` / null | `zero_wind`; corrections, friction, and fluxes zero |
| `initial_potential_temperature_neutral` | `0` / null | `neutral`; nonzero-wind neutral state and fluxes |
| `iterative_zero_buoyancy` | `>=1` / null | retained actual state; `neutral` if all corrections are exactly zero, `stable` if all are non-positive with at least one negative, `unstable` if all are non-negative with at least one positive; mixed/nonfinite corrections fail lineage |
| `iterative_invalid_obukhov` | `>=1` / null | `indeterminate_obukhov`; retain the actual last state/corrections when computed length is nonfinite or zero |
| `converged_stable` | `>=1` / positive finite | `stable`; final stable state/corrections |
| `converged_unstable` | `>=1` / negative finite | `unstable`; final unstable state/corrections |
| `did_not_converge` | typed error / N/A | no successful tuple; enabled request fails without authoritative mutation |

The consumer independently reconstructs log factors, exchange velocities,
vapor mass, sensible/latent fluxes, radiation, and advected heat from the named
primitives under `TOL-SNOWFREEZE-018`. Producer helpers and producer totals are
check-only and cannot establish closure.

### Frozen `S/F/Q` estimands and decision rule

Daily rows first join uniquely on site, day, lane, raw-source fingerprint,
forcing fingerprint, and geometry fingerprint; arrays then expand by hour and
ordered substep identity. Missing/duplicate identities or a fingerprint
mismatch are lineage failures, not delta samples. Effective fingerprints may
differ because their difference is an estimand.

`S` is the immutable whole-column same-state external carrier. `Q` is the
existing aligned-active, within-day sequential external carrier. `F`, the
consumer-only `frozen_active_projection_reference`, resets for every joined
daily source and copies Q's first applicable post-alignment active membership,
effective fingerprint, surface temperature, active mass/depth/density/cold
content, and exact selected daily albedo value/source/model/accumulated-
positive-temperature lineage. It also fixes the joined `z_T/z_q/z_u/z_0,aero`,
pressure, wind, dewpoint, canopy, daily solar/extraterrestrial radiation,
daylight, turbulent options, and longwave/sublimation selector IDs. Only hour
index, air temperature, hourly radiation, rain/snow depths and mass fluxes, and
hydrometeor temperatures vary hourly. `F` has no mutation, melt,
sublimation/deposition debit, cold change, or internal conduction and evaluates
one external-carrier `3600 s` tuple per hour.

For each joined hour, three-way common support is
`min(S_evaluated_seconds, 3600, Q_evaluated_seconds)`. Each operator integrates
only the prefix `[0, common_support)`, splitting the final tuple exactly at the
boundary without interpolation. All-evaluated and common-support summaries
remain separate. At tuple, hour, water-year window, and each median input, the
legacy bridge must close as `legacy Q complete = comparable Q external + Q
active internal conduction`. The frozen predecessor is the Python
`statistics.median` of the 35 Snowbird WY1990--2024 October-1-through-observed-
peak window sums and equals `+170.2536089 MJ m^-2` within
`TOL-SNOWFREEZE-019`; WY2025 is right-censored.

On identical three-way common support, `delta_projection = F - S` and
`delta_evolution = Q - F`. With `tol = 1e-6 J m^-2`, projection reconciles the
sign contradiction only when the Snowbird median satisfies `S < -tol`,
`F > +tol`, and `Q > +tol`; evolution reconciles it only when `S < -tol`,
`F < -tol`, and `Q > +tol`. Both require predecessor reproduction, every
bridge/reconstruction/delta closure, and invariant shortwave. Q remaining
positive does not turn a qualifying projection crossing into an evolution
crossing.

The remaining class predicates are exact. `PREDECESSOR_NOT_REPRODUCED` applies
when the reconstructed legacy-bridge Snowbird median differs from
`+170.2536089 MJ m^-2` by more than `1e-7 MJ m^-2` or any contributing
water-year bridge exceeds its `TOL-SNOWFREEZE-019` scale-aware tolerance.
`LEGACY_ESTIMAND_INTERNAL_CONDUCTION_SIGN_DIFFERENCE` applies only when the
reconstructed legacy sequential total is positive, the comparable sequential
external subset is non-positive, and the external-plus-active-conduction
bridge closes. `INITIAL_CONTROL_VOLUME_PROJECTION_DIFFERENCE` applies only
after raw-source identity passes and either the first effective-input
fingerprint differs exactly; the first active layer count, membership, or
state fingerprint differs exactly; or the first active mass, depth, density,
cold content, or temperature has different finite IEEE-754 `to_bits`.
`TOL-SNOWFREEZE-018` governs reconstruction and closure only; it cannot erase
or create an initial-projection identity difference. `MULTIFACTOR_UNRESOLVED`
is the exclusive fallback and applies only when valid evidence emits none of
the seven preceding classes, including positive Q when either ordered
predecessor lies inside the inclusive zero band `[-tol, +tol]` or neither
single ordered step uniquely crosses sign.

Decision classes are evaluated in this precedence:
`LINEAGE_OR_IDENTITY_FAILURE`, `PREDECESSOR_NOT_REPRODUCED`,
`LEGACY_ESTIMAND_INTERNAL_CONDUCTION_SIGN_DIFFERENCE`,
`INITIAL_CONTROL_VOLUME_PROJECTION_DIFFERENCE`,
`INITIAL_CONTROL_VOLUME_PROJECTION_RECONCILES_SIGN_CONTRADICTION`,
`STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION`,
`SUPPORT_CENSORING_MATERIALLY_CONTRIBUTES`, then
`MULTIFACTOR_UNRESOLVED`. Lineage failure suppresses causal interpretation;
after that precedence the six named non-lineage classes after
`LINEAGE_OR_IDENTITY_FAILURE` and before `MULTIFACTOR_UNRESOLVED` may coexist
in listed order. The exclusive fallback requires that none of the seven
preceding classes, comprising lineage failure plus those six non-lineage
classes, is emitted and cannot coexist with any preceding class.

Support censoring is material when the complete external delta changes sign
between common and all-evaluated support or when this ratio exceeds `0.05`:

```text
sum over S and Q and the five external terms
  abs(integral over [common_support, operator_evaluated_seconds))
/
sum over S and Q and the five external terms
  abs(all_evaluated_term_energy)
```

A zero denominator makes the ratio N/A. No cohort, forcing, operator,
projection, join, window, support, aggregation, tolerance, predecessor, or
decision predicate may change after result inspection. Every conclusion stays
operator-mechanics characterization; seasonal chronology, carrier validation,
persistence, terminal receipt, calibration, promotion, CoE retirement, and
cutover remain prohibited.

## Wet-Compaction Operand Authority And Duplicate-Alias Closure Addendum

1. PySnobal 0.2.3 establishes the Anderson/SNOBAL operator chronology:
   precipitation and generated melt precede H2O compaction, and runoff follows
   it. Its wet-compaction ratio consumes generated melt plus rain, not runoff,
   bounded snowpack storage loss, or retained-store change.
2. The pinned WEPP baseline independently fixes the contributing cut points.
   Positive melt and rain contact the snowpack before daily signed-melt
   redistribution and residual-rain routing. Negative melt is not liquid H2O.
   The baseline has no later Anderson/SNOBAL H2O-compaction operator, so it
   cannot authorize a routed-only substitute.
3. The openWEPP mapping is exact: gross positive generated melt is
   `sum(max(snow.hourly.melt_raw_m, 0))`; snow-contact rain is
   `rain_retained + rain_released`. These three components are summed once in
   active-snow finalization and passed through a single private daily field to
   the selected bulk or multilayer density implementation. Contact requires a
   pack at interval start; a mixed event that first establishes cover is
   excluded in that interval, as it is by PySnobal's fixed timestep
   `snowcover` gate. Raw daily rain is therefore an invalid substitute.
4. `snowpack_state_loss + routed_melt` is invalid because the compact ledger
   proves `routed_melt = snowpack_state_loss + rain_released`; the old driver
   therefore counted state loss twice. Routed-only is incomplete when generated
   melt or rain is retained, while state-loss-plus-rain loses generated melt
   retained inside the pack. A retained-liquid store level/change is state, not
   the day's liquid addition.
5. The correction is density/depth-only. Canonical and Snowbird scaled lanes
   must be labeled separately; the development fixture can characterize input
   sensitivity but cannot prove this physics correction or become forcing
   truth, calibration, a default, or transferability evidence.

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

## SNOWDENSITY-05D Opt-In CoE Melt Implementation Addendum

Status: draft (2026-06-26). This addendum ratifies opt-in routed-melt
consumption of the 05B shortwave source and 05C albedo state. It does not
default-activate the opt-in path and does not authorize parser, output-schema,
coefficient, or forcing-provider changes.

1. `legacy_coe` remains the default and rollback path. It must not require,
   update, or consume opt-in albedo state and must preserve current raw/routed
   melt behavior.
2. `coe_shortwave_albedo_v1` changes only the CoE shortwave term:
   `amelt = 0.0607 * hrrad * (1 - snow_albedo) * (1 - cancov)`.
   The radiation operand is the existing hourly `winter.hourly.rad_mj_m2_####`
   source from 05B; no snow-only source, scaling, clipping, or fitting is
   valid.
3. `bmelt`, `cmelt`, `dmelt`, the signed raw-melt identity, positive-melt
   depth cap, density gate, rain retention/release, daily negative-melt
   redistribution, runtime snow storage mutation, WB12 `S`, and WB13 `RM`
   lineage remain the same algorithmic path as `legacy_coe`.
4. The opt-in path must call the 05C albedo update before computing hourly
   melt for active snow. The positive-temperature age increment source and
   timing must be explicit at the call site. Missing required state is a typed
   error, not a fallback to `legacy_coe`.
5. Acceptance requires independent reconstruction of hourly raw melt,
   redistributed melt, routed `wmelt`, snowpack SWE loss, WB12 signed liquid
   forcing, and WB13 routed liquid forcing from typed operands. Reconstructing
   melt solely from post-day SWE/depth/density deltas is insufficient.

## SNOWDENSITY-05F Melt Closure / Density Handoff Addendum

Status: draft (2026-06-26). This addendum closes the SNOWDENSITY-05 melt
modernization ladder for opt-in downstream density work. It does not
default-activate `coe_shortwave_albedo_v1` and does not authorize parser,
runfile, CLI, output-schema, coefficient, radiation-source, or density-physics
changes.

1. Closure decision: SNOWDENSITY-05F closes the melt-modernization ladder
   without default activation. `legacy_coe` remains the default and rollback
   path. `coe_shortwave_albedo_v1` remains opt-in only.
2. Accepted density-facing interface: SNOWDENSITY-06 and later density packages
   may consume the opt-in melt boundary as a fixed upstream surface consisting
   of `snow_melt_model`, `winter.hourly.rad_mj_m2_####`, `snow_albedo`,
   `snow_albedo_model_id`, `snow_albedo_accumulated_positive_temperature_c_day`,
   `snow_albedo_fresh_snow_reset_water_equiv_m`,
   `snow_melt_shortwave_absorbed_fraction`, raw melt, redistributed melt,
   routed `wmelt`, SWE loss, WB12 `S`, WB13 liquid forcing, and runtime
   SWE/depth/density after-state.
3. Activation evidence baseline: 05E is promotion-candidate evidence relative
   to diagnostic legacy only; it is not default-activation evidence by itself.
   Any default-candidate package must report both diagnostic replay and H
   as-built context, including 05E's `robust_fail_count 13 -> 10` and
   `robust_ordinal_score 61 -> 84` improvement versus H's as-built
   `robust_fail_count=9` and `robust_ordinal_score=84` context.
   Post-review caveat: those 05E diagnostic deltas are regime-limited because
   the harness used `cancov = 0.0` and PySnobal-bridge radiation rather than
   the configured coniferous forest winter canopy cover of about `0.9` and the
   native/proven 05B shortwave source.
   They are therefore context, not an activation verdict.
4. Cold-start albedo policy: same-day future snowfall is a required opt-in
   continuity case. A producer may not clear albedo state solely because
   earlier same-day hours were snow-free if later same-day snowfall activates
   `coe_shortwave_albedo_v1`. It must apply fresh-snow reset, carry a valid
   previous opt-in state when one exists, or fail closed with a typed error.
5. Density handoff: SNOWDENSITY-06 may consume the opt-in melt boundary without
   retuning melt. Density work must not change melt coefficients, albedo
   constants, or shared radiation forcing to improve density signatures, and it
   must not use `coe_shortwave_albedo_v1` as a compensation layer for missing
   overburden/metamorphism compaction.
   Entry gate: before SNOWDENSITY-06 rubric evidence can carry a density or
   activation verdict, the snowbench/adjudication harness must drive canopy
   from the real per-day growth state, demonstrate that the configured
   coniferous forest winter `cancov` is near `0.9`, and either consume native
   openWEPP shortwave or prove the PySnobal-bridge radiation inversion
   like-for-like.
6. Deferred activation surfaces: production parser/runfile/CLI selectors,
   output-schema additions, compatibility deletion, default activation, and
   snow-influenced parity re-baselines require a later ratified activation
   package. Until then, diagnostic tools may expose opt-in selectors for
   evidence generation, while production runs remain on `legacy_coe`.
7. Brock albedo constants: the 05C constants carried in `08_snow_albedo.rs`
   were rechecked against `references/copyrighted/brock2000.pdf` during the
   05F post-review disposition. The deep-snow intercept/log coefficient,
   shallow addend/decay coefficient, `2.4 cm w.e.` transition scale (`0.024 m`
   water equivalent), and upper albedo bound `0.85` match the local reference.

## SNOWDENSITY-05G Harness Fidelity Rerun Addendum

Status: draft (2026-06-26). This addendum repairs the diagnostic evidence
surface identified by SNOWDENSITY-05F. It does not change production defaults,
production parser/runfile/CLI surfaces, output schemas, melt coefficients,
albedo constants, shared radiation forcing, density physics, or frost verdicts.

1. Canopy source: `openwepp-snowbench coe-melt` replay must consume the
   configured openWEPP runtime canopy value for the validation fixture. The
   previous `cancov = 0.0` harness constant is invalid representative-regime
   evidence for configured coniferous-forest fixtures whose winter canopy is
   expected near `0.9`.
2. Shortwave source/proof: if the replay continues to transport forcing through
   the PySnobal forcing CSV, it must publish the bridge identity proving that
   the replayed `hrrad` is the original openWEPP hourly shortwave after exact
   inversion of `net_solar = hrrad * 1_000_000 / 3600 * 0.8`. No fitted
   radiation scalar or snow-only radiation source is authorized.
3. Rerun evidence: the five-site SNOTEL rubric profile was regenerated for
   `legacy_coe` and `coe_shortwave_albedo_v1` after the canopy and shortwave
   proofs were in place. Result: `NON-PROMOTION` for default activation because
   forcing-robust failures did not improve (`9 -> 9`), even though ordinal score
   improved slightly (`84 -> 86`). The result updates diagnostic context only;
   default activation remains deferred to a later ratified activation package.
4. Density handoff: SNOWDENSITY-06 may use the 05G rerun as its melt boundary
   evidence only if the package artifacts carry the canopy and shortwave proof
   and still preserve `legacy_coe` default/rollback and opt-in-only
   `coe_shortwave_albedo_v1`.

## SNOWDENSITY-06 Density Compaction Addendum

Status: draft (2026-06-26). This addendum authorizes only offline density
compaction evidence for `physics_bulk`. It does not change production defaults,
production parser/runfile/CLI activation surfaces, output schemas, melt
coefficients, albedo constants, canopy values, shared radiation forcing, or
frost verdicts.

1. Fixed melt boundary: SNOWDENSITY-06 begins after the 05G representative
   coniferous rerun. `legacy_coe` remains default/rollback, and
   `coe_shortwave_albedo_v1` remains opt-in diagnostic context until a later
   activation package. The mixed/deciduous low-canopy melt-value fork remains
   SNOWDENSITY-05H scope and is not required for this density-only package.
2. Candidate shape: `density_compaction_v1` is an offline `physics_bulk`
   snowbench variant. It may alter fresh-snow-density and compaction-strength
   constants within `INV-SNOWFREEZE-051`, but it must preserve baseline
   candidate melt constants and may not alter radiation, canopy, albedo, or
   site-specific controls to win a density profile.
3. Named compaction constants: the producer must publish the PTM, POC, and
   liquid-water compaction constants named in `INV-SNOWFREEZE-058` in
   `physics_bulk_summary.json`. Hidden literals are not accepted evidence.
4. Evaluation surface: disposition is based on the v74/v75 rubric plus a
   density/densification robust-cell summary covering cold-season bulk density,
   densification trajectory, depth-SWE slope, and bias-sign consistency. Whole
   rubric scores remain context, not an escape hatch for melt compensation.
5. Closure: if finite evidence fails the density-cell gate, SNOWDENSITY-06 must
   close `NON-PROMOTION` with a specific follow-on; it must not route to melt
   retuning, default activation, or frost attribution.

## SNOWDENSITY-06B CoE-Bound Density Replay Addendum

Status: draft (2026-06-26). This addendum authorizes only offline CoE-bound
density replay evidence for `density_compaction_v1`. It does not change
production defaults, parser/runfile/CLI activation surfaces, output schemas,
melt coefficients, albedo constants, canopy values, shared radiation forcing,
runtime publication, or frost verdicts.

1. Fixed boundary: the replay must consume daily CoE `snow_water_m`,
   `snowpack_swe_loss_m`, `routed_melt_m`, and related melt ledger fields from
   a separately generated `coe-melt` snowbench boundary. CoE `snow_water_m`
   remains the SWE authority for every emitted replay row.
2. Density-only mutation: `density_compaction_v1` may update physical depth and
   bulk density through fresh-snow density, dry compaction, and wet compaction.
   The replay may not compute melt with the old degree-day `physics_bulk`
   surrogate and may not change CoE melt, albedo, canopy, or shortwave
   operands.
3. Evidence: the report must publish the selected CoE boundary model, daily SWE
   identity residuals, boundary SWE-loss/routed-melt totals, candidate density
   constants, no-site-tuning assertion, and whole-rubric plus
   density/densification robust-cell summaries for the five SNOTEL fixtures.
4. Closure: SNOWDENSITY-06B may close complete only with finite adjudication
   evidence. If no candidate clears the whole-rubric and density-cell profile
   gates, it closes non-promotion with the next blocker named; it must not route
   to mixed/deciduous canopy work or default activation inside this package.

## SNOWDENSITY-07 Runtime Opt-In Addendum

Status: draft (2026-06-26). This addendum authorizes a typed runtime opt-in for
the CoE-bound density-compaction result. It does not default-activate the
opt-in path, add parser/runfile/user CLI selectors, change output schema,
retune melt/albedo/radiation/canopy constants, authorize mixed/deciduous canopy
adjudication, delete compatibility snow behavior, or reopen frost attribution.

1. Selector: `snow_density_model = legacy_wepp` is the default and rollback
   path. `snow_density_model = physics_bulk_density_compaction_v1` is the first
   runtime opt-in member ratified by this amendment. Later ratified density
   members, including `physics_bulk_spring_densification_v1`, must add their own
   amendment and closure gates rather than inheriting SNOWDENSITY-07 acceptance.
2. Boundary split: CoE melt/liquid behavior remains authoritative. The runtime
   must carry the CoE boundary depth, density, and settle-day count separately
   from the opt-in publication/frost-insulation depth and density. The next CoE
   melt calculation consumes the CoE boundary carry, not the opt-in density
   surface.
3. Mutated operands: the opt-in density model may change only physical
   `snow.runtime_depth_m` and `snow.runtime_density_kg_m3`. `snow.runtime_swe`,
   signed `S`, raw melt, redistributed melt, routed `wmelt`, post-winter rain,
   snowpack SWE loss, and albedo state remain the CoE-boundary result.
4. Compaction update: the runtime opt-in uses the same `density_compaction_v1`
   fresh-snow-density, dry-compaction, and wet-compaction constants accepted by
   SNOWDENSITY-06/06B. Constants are global literature-derived defaults, not
   site-fitted parameters.
5. Acceptance: runtime evidence must prove default-disabled isolation,
   SWE/depth-density anti-aliasing, separate CoE boundary carry, direct R4G
   state mutation, downstream operands, shadow projection, runtime carry, and
   existing publication-facing snow state. If any of those surfaces still read a
   legacy/compatibility alias for an opt-in claim, SNOWDENSITY-07 must continue
   or close `HOLD`.

## SNOWDENSITY-08 Snow/Frost Gate Rerun Addendum

Status: draft (2026-06-26). This addendum authorizes a gate-rerun package after
the SNOWDENSITY-07 typed density opt-in. It is an adjudication step, not a
production activation step.

1. Two evidence surfaces: SNOTEL snow/density evidence and non-SNOTEL
   frost-site evidence are separate cells in the decision. Clearing one does
   not clear the other.
2. SNOTEL lineage: the accepted `physics_bulk_density_compaction_v1` SNOTEL
   evidence may be rerun through the CoE-bound density replay when the replay
   proves daily CoE SWE identity, fixed CoE liquid/melt boundaries, no site
   constants, and the same density update accepted by SNOWDENSITY-07.
3. Non-SNOTEL coupling: frost attribution requires a coupled WAT/publication
   run in which opt-in density mutates the runtime snow depth consumed by frost
   and the WAT `Snow-Depth` diagnostic. Offline snow-only substitutions may be
   reported as diagnostic snow-depth signals, but they cannot authorize frost
   attribution.
4. Default isolation: `legacy_wepp` remains the default. The gate rerun must not
   add parser/runfile/CLI activation, default activation, output schema changes,
   coefficient tuning, radiation/albedo/canopy/melt retuning, frost-physics
   edits, or compatibility deletion.
5. Closure: the report must publish `frost_attribution_authorized`, the SNOTEL
   robust/density-cell deltas, non-SNOTEL snow-control status counts, coupled
   opt-in WAT availability, CoE boundary anti-alias proof, and the next blocker.

## SNOWDENSITY-09 Diagnostic Coupled WAT Rerun Addendum

Status: draft (2026-06-26). This addendum authorizes the diagnostic bridge
needed to test the SNOWDENSITY-07 runtime opt-in against the non-SNOTEL
frost-site WAT rubric. It is an evidence path, not a user-facing activation
path.

1. Selector boundary: `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL` may select
   `legacy_wepp`, `physics_bulk_density_compaction_v1`, or, when governed by
   `INV-SNOWFREEZE-068`, `physics_bulk_spring_densification_v1` for
   direct-production diagnostic runs. Unset or empty means `legacy_wepp`.
   Unknown non-empty values must fail closed.
2. Coupled path: the opt-in run must execute the normal direct-production WAT
   publication path. WAT `Snow-Depth` remains the publication of
   `snow.runtime_depth_m`; replacing WAT rows after execution is invalid.
3. Trace proof: the run must publish direct-production snow trace rows that name
   the selected `snow_density_model`, so the evidence proves the opt-in was
   selected by the process that generated WAT.
4. Attribution boundary: SNOWDENSITY-09 may authorize resuming frost attribution
   only if the coupled opt-in snow-control gate passes for fixtures with
   observed snow-depth rows and the SNOWDENSITY-08 SNOTEL density gate remains
   cleared. Sites with no observed snow-depth rows, including SCAN Mandan ND and
   Reynolds Creek ID, are diagnostic-only out-of-gate evidence for this
   snow-depth gate. They must remain reported but must not be counted as gate
   pass, fail, or blocker. Otherwise the report must name the blocker and keep
   frost work held behind snow-control evidence.
5. Isolation: this package must not add parser/runfile/user CLI activation,
   default activation, output schema changes, compatibility-runtime changes,
   coefficient/canopy/radiation/albedo/melt/density/frost tuning, or new site
   constants.

## SNOWDENSITY-10.3.1a Per-Day Cancov Direct-Runtime Addendum

Status: draft (2026-06-26). This addendum resolves the SNOWDENSITY-10.3.1
blocker that current snowbench/CoE melt diagnostics archived only a scalar
runtime-surface `cancov`, while the mixed/deciduous melt question requires the
per-day canopy state used by the direct runtime.

1. Source authority: the accepted daily canopy source is the direct production
   growth-state path that computes
   `growth_state_for_publication.canopy_cover_fraction` before snow liquid
   partition and canopy interception. A separate diagnostic canopy model is not
   authorized.
2. Replay authority: `openwepp-snowbench coe-melt` must consume a date-aligned
   daily canopy series when it replays CoE melt. Repeating
   `primary_canopy_cover_fraction` across all days is stale initial-condition
   evidence and cannot carry low-canopy or seasonal-canopy verdicts.
3. Validation: the producer must fail closed on missing, duplicate,
   non-contiguous, non-finite, out-of-bounds, or length-mismatched canopy rows.
   The report must publish row count, min, max, mean, first, last, dynamic
   status, and source.
4. Compatibility: the PySnobal forcing CSV and CoE boundary CSV schemas may
   remain unchanged; the canopy trajectory is a separate diagnostic sidecar and
   JSON/Markdown summary. This amendment does not change production output
   schemas.
5. Boundaries: no canopy tuning, melt/density/albedo/radiation/frost physics
   change, fixture edit, parser/runfile/user CLI selector, default activation,
   or compatibility-runtime deletion is ratified here.

## SNOWDENSITY-10.3.5a Harder-Pomeroy Meteorology Crate Addendum

Status: draft (2026-06-27). This addendum authorizes a production-free
meteorology library so rain/snow phase candidates can be tested without
modifying the active WEPP `RST` partition.

1. Scope: the only authorized implementation product is
   `crates/openwepp-meteorology`, a pure Rust library of psychrometric
   primitives and Harder-Pomeroy hydrometeor-temperature phase-fraction
   functions. It may be a workspace member but must not be a dependency of
   production runtime, runner, parser, output, or compatibility crates in this
   package.
2. Authority: Harder and Pomeroy (2013) supplies the candidate hydrometeor
   temperature equation, vapor-density supporting equations, latent-heat/
   diffusivity/conductivity terms, and logistic rain-fraction coefficient sets.
   Standard saturation/dewpoint helper functions must be cited in code comments
   or package evidence when they extend beyond the paper's printed equations.
3. API discipline: public candidate APIs must accept typed Celsius temperature
   and unit-interval humidity inputs, return typed candidate outputs, and
   surface typed errors for non-finite, out-of-domain, or non-convergent cases.
   The solver must expose iteration metadata so future validation can separate
   numerical failure from physics failure.
4. Test discipline: tests must cover saturation-vapor-pressure reference
   values, dewpoint/RH round trips, saturated-air identity (`Ti == Ta` within
   tolerance), unsaturated ordering (`Ti < Ta` where applicable),
   Harder-Pomeroy fixed-point convergence against independent numeric
   reference values, coefficient-set monotonicity, fraction closure, and
   explicit non-convergence behavior.
5. Isolation: this package must not change `RST`, `stmtim`, daily/hourly WEPP
   partition behavior, production selectors, parser/runfile schemas, output
   schemas, fixtures, default activation, compatibility runtime, snowmelt,
   density, canopy, albedo, radiation, or frost physics.
6. Deferred work: Jennings/observed-phase corpus validation, mixed/deciduous
   production adjudication, production phase selector design, default
   activation, and any route from hydrometeor-temperature fractions into
   `hrrain`/`hrsnow` are follow-on packages requiring new contract amendments.

## SNOWDENSITY-10.3.5b Opt-In Hourly Partition And Jennings Validation Addendum

Status: draft (2026-06-27). This addendum authorizes one package-bound
production direct-runtime opt-in path for observed-phase adjudication while
leaving the default WEPP `RST` partition untouched.

1. Selector boundary: the only authorized selector values are `legacy_rst` and
   `harder_pomeroy_hourly`. `legacy_rst` is the default in all absent/empty
   cases and preserves the existing `stmtim` threshold branch. Unknown explicit
   selector values fail closed. Parser, runfile, and user CLI activation
   surfaces are out of scope.
2. Candidate wiring: when `harder_pomeroy_hourly` is explicitly selected, the
   hourly winter forcing seam may compute Harder-Pomeroy hourly fractions from
   synthesized hourly air temperature and finite relative humidity. The direct
   snow liquid-partition consumer must receive those selected hourly `hrrain`
   and `hrsnow` values; symbol-only projection is not sufficient evidence.
3. Humidity derivation: daily dew point may be used to derive each hourly RH.
   If the dewpoint-derived vapor-pressure ratio exceeds one because the daily
   dew point is warmer than a synthesized hourly air temperature, the only
   authorized normalization is exact saturation (`RH=1.0`) with evidence. This
   is a bounded physical normalization, not a calibration knob. Negative,
   non-finite, zero-saturation, or otherwise invalid humidity states fail
   closed.
4. Conservation and units: the legacy `hrsnow` surface remains snowfall depth,
   not water equivalent. For active precipitation hours and both selector
   values, `hrrain + hrsnow / 10` must reconstruct active hourly precipitation
   depth within roundoff. Fractional rain/snow coexistence is allowed only for
   the opt-in candidate.
5. Validation: Jennings et al. file2/file3 validation must be run when the full
   local file2 corpus is present. Reports must include rows/stations scored,
   accuracy, rain/snow confusion counts, per-station predicted 50% air-
   temperature thresholds, and humid/maritime-vs-dry/continental threshold
   contrast where station metadata or scored humidity supports it. Jennings
   observations are adjudication evidence only; coefficients and defaults must
   not be fitted to the corpus.
6. Isolation: this package must not change default behavior, snow density,
   snowmelt, canopy, radiation, frost physics, compatibility runtime, fixtures,
   public output schemas, or parser/runfile/user interfaces.

## SNOWDENSITY-10.3.7 Opt-In Winter-Thaw State-Loss Addendum

Status: draft (2026-06-27). This addendum authorizes one opt-in thaw-response
candidate to test the 10.3.6 finding that CoE raw melt is computed during warm
snowpack hours but is only partly realized as SWE/depth loss under the legacy
low-density gate.

1. Selector boundary: the only newly authorized melt selector is
   `coe_winter_thaw_state_loss_v1`. `legacy_coe` remains the default,
   compatibility/comparator surface, and rollback path. `coe_shortwave_albedo_v1`
   remains a separate opt-in shortwave/albedo candidate and is not activated,
   retired, or promoted by this addendum. Parser, runfile, public output, and
   user CLI activation surfaces are out of scope.
2. Authorized delta: the candidate keeps the current CoE melt-energy family and
   signed raw-melt lineage unchanged. When the snowpack is active, `wmelt > 0`,
   and the legacy post-melt density gate would retain that positive melt as
   density-only compaction below `350 kg m^-3`, the candidate may instead emit
   the positive `wmelt` as snowpack SWE state loss, routed melt, and downstream
   liquid forcing while preserving proportional snow-depth loss and non-negative
   bounded after-state.
3. Preserved boundaries: the candidate must not change `amelt`, `bmelt`,
   `cmelt`, `dmelt`, corrected negative-melt redistribution, radiation source,
   canopy attenuation, precipitation phase partition, snow-density constants,
   rain retention/release mechanics, frost physics, output schemas, or fixture
   inputs. Rain heat, sub-canopy longwave, Qwet/frzftp, and any forest-energy
   revision remain separate later levers.
4. Albedo isolation: `coe_winter_thaw_state_loss_v1` must not require, update,
   consume, synthesize, or publish an albedo state. Existing
   `coe_shortwave_albedo_v1` albedo fail-closed behavior remains unchanged.
5. Conservation and anti-alias acceptance: the package must reconstruct raw
   melt, redistributed melt, routed melt, snowpack SWE loss, modeled depth loss,
   retained/released rain, and final SWE/depth/density closure from produced
   artifacts. One-sided non-negativity or internal formula self-consistency is
   insufficient if wrong aliases could pass. The reconstruction must also show
   that state loss is bounded by available pack storage plus same-day snow/rain
   inputs and that the SWE state loss appears in routed liquid/WAT balance,
   after separating retained rain from released rain.
6. Event-window acceptance: paired Sleepers/Harvard thaw-ablation evidence must
   show lower under-ablation count and lower aggregate depth-loss deficit than
   `legacy_coe`, with no site constants or coefficient tuning. Failure to meet
   both improvement gates is a `HOLD` disposition, not activation authority.
7. Coupled WAT acceptance: the opt-in selector must be exercised through the
   real direct-production WAT path with `OPENWEPP_SNOWDENSITY1037_MELT_MODEL`.
   The report must prove the selected model reached the direct snow partition,
   compare paired snow-depth control against `legacy_coe`, and classify the
   coupled impact before any fix or activation claim. A snowbench-only
   improvement is not sufficient closure.
8. Isolation: this package must not change default behavior, compatibility
   runtime, public output schemas, parser/runfile/user interfaces, fixtures,
   coefficients, radiation, canopy, phase partition, density constants, frost,
   sub-canopy longwave, rain heat, or Qwet/frzftp behavior.

## SNOWDENSITY-10.3.8 Opt-In Liquid Holding-Capacity Addendum

Status: draft (2026-06-27). This addendum authorizes one opt-in capacity-
drainage candidate to replace the 10.3.7 all-positive-thaw release experiment
with a physically bounded retained-liquid store.

1. Selector boundary: the only newly authorized melt selector is
   `coe_liquid_holding_capacity_v1`. `legacy_coe` remains the default,
   compatibility/comparator surface, and rollback path. `coe_shortwave_albedo_v1`
   and `coe_winter_thaw_state_loss_v1` remain separate opt-in candidates and are
   not activated, retired, or promoted by this addendum. Parser, runfile, public
   output, and user CLI activation surfaces are out of scope.
2. In-repo authority: the liquid capacity default is the non-fitted in-repo
   `max_liquid_water_volume_fraction = 0.01` lineage, supported by Marks R-55,
   Anderson retained/free/excess-water semantics, SNOW-17 PLWHC precedent, and
   local SNOBAL-lineage capacity/runoff inspection. Fixture residuals may
   adjudicate the candidate but must not fit the capacity default.
3. Authorized delta: the candidate keeps the current CoE melt-energy family and
   signed raw-melt lineage unchanged. When the snowpack is active and positive
   melt/rain would be retained below the legacy `350 kg m^-3` density gate, the
   candidate may retain liquid only up to its computed holding capacity and must
   route excess liquid as snowpack SWE state loss or released rain into the
   downstream liquid-forcing path.
4. Persistent state: retained liquid is a typed snow-lane state surface, not a
   daily scratch variable. It must be finite, non-negative, bounded by current
   capacity after each snow update, and carried into the next day until released
   or the pack disappears. Pack disappearance must clear retained liquid through
   routed release or a zero-state closure proof, not silent deletion.
5. Preserved boundaries: the candidate must not change `amelt`, `bmelt`,
   `cmelt`, `dmelt`, corrected negative-melt redistribution, radiation source,
   canopy attenuation, precipitation phase partition, snow-density constants,
   frost physics, output schemas, fixture inputs, `coe_shortwave_albedo_v1`
   albedo behavior, or `coe_winter_thaw_state_loss_v1` behavior. Rain heat,
   sub-canopy longwave, Qwet/frzftp, and any forest-energy revision remain
   separate later levers.
6. Conservation and anti-alias acceptance: the package must reconstruct raw
   melt, redistributed melt, routed melt, snowpack SWE loss, modeled depth loss,
   retained/released rain, retained/released snow liquid, liquid capacity,
   retained-liquid after-state, and final SWE/depth/density/liquid closure from
   produced artifacts. The reconstruction must prove released liquid is bounded
   by incoming liquid plus prior retained liquid above current capacity and
   appears in routed liquid/WAT balance.
7. Event-window acceptance: paired Sleepers/Harvard thaw-ablation evidence must
   show lower under-ablation count and lower aggregate depth-loss deficit than
   `legacy_coe`, with no site constants or coefficient tuning. Failure to meet
   both improvement gates is a `HOLD` disposition, not activation authority.
8. Coupled WAT acceptance: the opt-in selector must be exercised through the
   real direct-production WAT path with `OPENWEPP_SNOWDENSITY1038_MELT_MODEL`.
   The report must prove the selected model reached the direct snow partition,
   compare paired snow-depth control against `legacy_coe`, and classify the
   coupled impact before any fix or activation claim. A snowbench-only
   improvement is not sufficient closure.
9. Isolation: this package must not change default behavior, compatibility
   runtime, public output schemas, parser/runfile/user interfaces, fixtures,
   coefficients, radiation, canopy, phase partition, density constants, frost,
   sub-canopy longwave, rain heat, or Qwet/frzftp behavior.

## SNOWDENSITY-10.3.11 Opt-In Spring Compaction/Densification Addendum

Status: draft (2026-06-27). This addendum authorizes one opt-in density
candidate to test whether the March/April compaction-feasible residuals from
SNOWDENSITY-10.3.10 can be reduced by physically realizing wet-snow compaction
without changing snow mass or the contract density cap.

1. Selector boundary: the only newly authorized density selector is
   `physics_bulk_spring_densification_v1`. `legacy_wepp` remains the default,
   compatibility/comparator surface, and rollback path.
   `physics_bulk_density_compaction_v1` remains the prior opt-in density member
   and is not retired or reinterpreted by this addendum. Parser, runfile,
   public output, and user CLI activation surfaces are out of scope.
2. Fixed melt/liquid boundary: the candidate consumes the selected CoE
   melt/liquid boundary exactly as supplied. For the 10.3.11 coupled gate, that
   boundary is `coe_liquid_holding_capacity_v1`; future use with another
   ratified CoE boundary requires naming that boundary in the package evidence.
3. Authorized delta: relative to `physics_bulk_density_compaction_v1`, the only
   authorized algorithmic change is wet-compaction realization. When same-day
   liquid for density compaction is positive, the candidate must apply the same
   total liquid once to the Anderson/SNOBAL liquid-compaction term and may let
   wet conditions accelerate the daily time-compaction substeps. This is a
   process-timing change, not a new fitted multiplier or hidden melt/export
   term.
4. Preserved boundaries: the candidate must not change fresh-snow-density
   constants, dry/wet compaction multipliers, liquid holding-capacity constants,
   melt coefficients, radiation source, canopy attenuation, precipitation phase
   partition, rain heat, sub-canopy longwave, frost physics, output schemas,
   fixture inputs, compatibility runtime, or Qwet/frzftp behavior.
5. Cap and conservation: runtime SWE must reconstruct the selected CoE runtime
   SWE after density mutation within roundoff. Final runtime density must remain
   `<= 522 kg m^-3`. Cap sensitivity may be reported as evidence, but this
   addendum does not authorize raising the cap.
6. Anti-fitting: the runtime calculation must not consume observed snow depth,
   observed density, fixture identity, site metadata, residual row class, or
   snow-control tolerance. Depth improvements are verdict evidence only after
   execution, not inputs to compaction.
7. Coupled WAT acceptance: the opt-in selector must be exercised through the
   real direct-production WAT path with `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL`.
   The report must prove the selected model reached the direct snow partition,
   compare against the `coe_liquid_holding_capacity_v1` plus
   `physics_bulk_density_compaction_v1` baseline, and report March/April
   compaction-feasible clearance, cap-limited residuals, under-persistence rows,
   and per-surface snow-control deltas. Worsening any observed-snow-depth paired
   surface is a non-promotion/`HOLD` outcome.
8. Closure: this addendum authorizes only an opt-in improvement claim unless
   the observed-snow-depth coupled WAT gate passes. Frost attribution remains
   blocked while paired snow-control failures remain.

## SNOWDENSITY-10.3.12 Combined Bundle Activation Adjudication Addendum

Status: draft (2026-06-27). This addendum authorizes activation adjudication
for the currently best observed-snow-depth bundle:
`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`.

1. Bundle identity: the bundle is the composition of the opt-in liquid
   holding-capacity boundary governed by `INV-SNOWFREEZE-067` and the opt-in
   density-compaction boundary governed by `INV-SNOWFREEZE-060` and
   `INV-SNOWFREEZE-062`. It adds no formula, selector, coefficient, state
   variable, fixture, or publication schema.
2. Selector boundary: the only authorized execution surface is the explicit
   package-bound selector pair `OPENWEPP_SNOWDENSITY1038_MELT_MODEL =
   coe_liquid_holding_capacity_v1` plus
   `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL =
   physics_bulk_density_compaction_v1`. Absent selectors preserve
   `legacy_coe`/`legacy_wepp` defaults; parser/runfile/user activation is not
   ratified by this addendum.
3. Direct coupling proof: activation adjudication must exercise the real
   direct-production WAT path and must prove from trace rows that both selected
   members reached the direct snow partition. Snowbench-only, offline, or
   artifact-only evidence is insufficient for activation.
4. Comparator set: the adjudication report must compare default, liquid-
   holding-only, combined-bundle, and rejected spring-densification evidence so
   the activation decision is read against the actual package ladder, not a
   single isolated metric.
5. Residual classification: any remaining paired observed snow-depth failures
   must be classified by surface, cover, month, residual sign, and March/April
   cap class. Observation-blocked surfaces remain diagnostic-only.
6. Activation Policy B: default activation no longer requires zero paired
   observed snow-depth failures, because a zero-failure target would fit to the
   validation fixtures and ignore structurally irreducible residuals such as
   patchy point-vs-areal meltout. Activation instead requires the bundle to be
   strictly better than the current default on gate-eligible paired-snow
   surfaces plus workspace-suite no-regression/conservation evidence. The
   workspace-suite gate means the existing suite passes under the bundle
   selectors, composite snow-state conservation closes, and downstream snow-
   affected output deltas are conserved-by-construction rather than separately
   diffed as regressions, or the package must close `HOLD-OPT-IN-BUNDLE` with
   the missing evidence named.
7. Frost separation: snow-control residuals do not automatically block default
   activation under Policy B, but they do keep frost attribution separately
   blocked until the snow state is good enough to isolate frost residuals.
8. Protected boundaries: this addendum does not authorize coefficient tuning,
   density-cap changes, observed-depth fitting, new compaction-rate variants,
   open-surface ablation, patchy snow-cover logic, sub-canopy longwave, rain
   heat, phase partition, canopy/radiation changes, Qwet/frzftp, fixture edits,
   public output-schema changes, compatibility-runtime changes, or parser/
   runfile/user activation.

## SNOWDENSITY-10.3.13 Residual-Tail And Policy-B Diagnostic Addendum

Status: draft (2026-06-27). This addendum authorizes a diagnostic-only package
after SNOWDENSITY-10.3.12.

1. Diagnostic identity: the package must classify residual tails from the
   current evidence ladder and must not implement or tune snow physics.
2. Evidence lineage: residual attribution must consume real direct-production
   WAT outputs from current default, liquid-holding-only, combined-bundle, and
   rejected spring-densification runs. Aggregate counts alone are insufficient.
3. Under-persistence attribution: the report must classify bundle under-
   persistence rows by date-level transition from the holding-only arm:
   persisted under-persistence, induced from a pass row, induced from an
   opposite-sign over-persistence row, or otherwise unresolved.
4. Over-persistence attribution: the report must keep March/April cap classes
   separate under the active `522 kg m^-3` cap: cap-limited depletion, patchy
   depletion, compaction-feasible, under-persistence, pass, or unresolved.
5. Policy-B basis: the report must define the workspace-suite no-regression/
   conservation evidence required for activation, including that downstream
   snow-affected output deltas are conserved-by-construction and not separately
   diffed by this diagnostic, and must distinguish missing activation evidence
   from the separate frost-attribution blocker.
6. Cap boundary: `550 kg m^-3` SNOBAL cap re-anchoring may be recommended only
   as follow-up. This diagnostic does not amend `INV-SNOWFREEZE-003`, does not
   change the cap, and does not rerun physics at a different cap.
7. Protected boundaries: this addendum does not authorize default activation,
   production physics, density-cap changes, coefficient tuning, observed-depth
   fitting, parser/runfile/user selectors, fixture edits, output-schema changes,
   open-surface ablation, new compaction-rate variants, frost attribution,
   Qwet/frzftp, or compatibility-runtime changes.

## SNOWDENSITY-10.3.14 Policy-B No-Regression And Cap Authority Addendum

Status: draft (2026-06-27). This addendum authorizes the diagnostic package
that decides whether the current opt-in bundle can move to a separate default-
activation package under the active density cap.

1. Evidence lineage: the diagnostic must consume the SNOWDENSITY-10.3.12 real
   direct-production bundle report and the SNOWDENSITY-10.3.13 residual Policy-B
   diagnostic. Reconstructed or synthetic WAT rows are not valid evidence.
2. Policy-B workspace-suite/conservation gate: the package must run and record
   the workspace-suite no-regression gate under the existing bundle selectors
   `OPENWEPP_SNOWDENSITY1038_MELT_MODEL =
   coe_liquid_holding_capacity_v1` and
   `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL =
   physics_bulk_density_compaction_v1`, plus composite snow-state conservation
   closure. Downstream snow-affected output deltas are conserved-by-construction
   and not separately diffed by this diagnostic. Missing or failed workspace-
   suite/conservation evidence blocks activation readiness even if snow-depth
   residuals improve.
3. Composite state closure: package evidence must verify that the composed
   trace state respects `SWE = depth * density / 1000` and does not exceed the
   active `522 kg m^-3` cap except within floating-point tolerance.
4. Density-cap authority: `522 kg m^-3` remains the active runtime cap for this
   package. `550 kg m^-3` SNOBAL evidence may be reported only as a same-SWE,
   cap-pinned projection to quantify potential benefit and risk. Projection
   evidence alone cannot authorize cap changes, dynamic runtime behavior, or
   default activation at `550 kg m^-3`.
5. Disposition split: if the workspace-suite gate passes and state closure
   holds, this package may close with active-cap readiness for a separate
   activation package. It must keep cap re-anchor, shallow-pack compaction guards, open-
   surface ablation, and other residual physics as follow-up unless dynamic
   evidence has been produced under a later contract amendment.
6. Protected boundaries: this addendum does not authorize default activation,
   production physics changes, density-cap changes, coefficient tuning,
   observed-depth fitting, new selectors, parser/runfile/user controls, fixture
   edits, output-schema changes, frost attribution, Qwet/frzftp, or
   compatibility-runtime changes.

## SNOWDENSITY-10.3.15 Default Activation Under Active Cap Addendum

Status: draft (2026-06-27). This addendum authorizes default activation of the
current active-cap snow-depth bundle after SNOWDENSITY-10.3.14 established
Policy-B readiness under explicit selectors.

1. Activated default: when the package-bound direct-production selector
   environment variables are absent, the direct-production snow path must select
   `snow_melt_model = coe_liquid_holding_capacity_v1` and
   `snow_density_model = physics_bulk_density_compaction_v1`.
2. Rollback/test selectors: `OPENWEPP_SNOWDENSITY1038_MELT_MODEL=legacy_coe`
   and `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=legacy_wepp` remain explicit
   rollback/test selectors. They are not parser, runfile, or user CLI
   configuration surfaces. Empty selector values select the activated default.
3. Selector fail-closed behavior: unknown values fail closed. The rejected
   `physics_bulk_spring_densification_v1`, `coe_winter_thaw_state_loss_v1`,
   and `coe_shortwave_albedo_v1` candidates are not accepted by the active
   default selector path unless a later contract amendment re-ratifies them.
4. Boundary preservation: the active density cap remains `522 kg m^-3`; the CoE
   SWE/liquid boundary split, `snow.runtime_swe` authority, WAT `Snow-Water`
   and `Snow-Depth` output schemas, fixture inputs, compatibility runtime,
   climate/radiation/canopy/phase/frost/Qwet/frzftp behavior, and user-facing
   configuration surfaces remain unchanged.
5. Consumer proof: package evidence must include real direct-production WAT and
   trace rows showing the no-env default selected both activated members. Source
   markers or producer-only state are not sufficient for activation closure.
6. Residual disposition: the activated bundle is an improvement, not snow/frost
   closure. The `498/1415` paired snow-depth residual failures from the
   Policy-B active-cap evidence remain a release-note item, and frost
   attribution remains blocked by `SNOW-CONTROL-RESIDUALS-REMAIN`.

## SNOWDENSITY-10.3.16 Open-Surface Ablation Stage A Addendum

Status: draft (2026-06-27). This addendum authorizes an opt-in diagnostic
candidate for the open-surface mass-excess tail remaining after default
activation of the active-cap snow-depth bundle.

1. Candidate selector: `coe_open_sublimation_stage_a_v1` may be selected only by
   the package-bound melt selector used for direct-production diagnostics.
   Absent or empty selector values continue to select the activated
   `coe_liquid_holding_capacity_v1` default. `legacy_coe` remains the explicit
   rollback/test selector. Unknown values fail closed.
2. Single authorized delta: Stage A may add only `snow_sublimation`, a bounded
   turbulent latent mass-loss SWE sink. It must subtract finite non-negative
   water-equivalent mass from runtime snowpack SWE as vapor export. It must not
   become routed melt, post-winter rain, liquid-water release, infiltration,
   runoff, density-only compaction, or public WAT/HBP/PASS schema.
3. Authority and constants: the candidate must use Marks/SNOBAL paper authority
   and physical constants for latent mass-loss. PySnobal/libsnobal C source is
   not implementation authority unless a non-GPL-family license is confirmed
   against `deny.toml`. Fixture-tuned constants are invalid.
4. Conservation ledger: package evidence must independently reconstruct
   `runtime_swe_before + snowfall_water + rain_retained - snowpack_state_loss -
   snow_sublimation = runtime_swe_after` within tolerance. The same evidence
   must show `snow_sublimation` does not appear in routed melt/liquid totals.
5. Coupled evidence gate: package evidence must include real direct-production
   WAT and trace rows proving the opt-in candidate reached the snow partition
   and comparing against the activated default on paired observed snow-depth
   surfaces. The package must report open-surface cap-limited over-persistence,
   under-persistence, sublimation magnitude, conservation status, and protected
   boundary scans.
6. Promotion boundary: Stage A cannot activate the candidate. If open-surface
   cap-limited over-persistence is not reduced, under-persistence worsens,
   sublimation magnitude is outside the literature-defensible range, or
   conservation fails/missing, the package must close `HOLD` or non-promotion.
7. Protected boundaries: this addendum does not authorize default activation,
   two-layer snow-surface structure, density-cap changes, density-model changes,
   albedo/radiation/canopy/phase/rain-heat/longwave/frost/Qwet/frzftp changes,
   parser/runfile/user controls, fixture edits, compatibility-runtime changes,
   public output-schema changes, or frost attribution.

## SNOWDENSITY-10.3.17 Shallow-Pack Compaction Guard Addendum

Status: draft (2026-06-27). This addendum authorizes an opt-in diagnostic
density candidate for the density-arm-induced under-persistence tail remaining
after default activation of the active-cap snow-depth bundle.

1. Candidate selector: `physics_bulk_shallow_guard_v1` may be selected only by
   the package-bound direct-production density selector. Absent or empty
   selector values continue to select the activated
   `physics_bulk_density_compaction_v1` default. `legacy_wepp` remains the
   explicit rollback/test selector. Unknown values fail closed.
2. Single authorized delta: relative to `physics_bulk_density_compaction_v1`,
   the candidate may only reduce density-compaction aggressiveness while the
   pre-compaction physical snow depth is below
   `snow_shallow_compaction_guard_depth_threshold = 0.25 m`. It may mutate only
   physical runtime snow depth and runtime snow density after conserving the
   selected CoE SWE boundary.
3. Authority and constants: the threshold is derived from the Marks/SNOBAL
   active surface-layer depth (`max_z_s_0`, approximately `0.25 m`) and
   shallow-snow layer-collapse precedent, not from paired observed fixtures.
   PySnobal/libsnobal C source is not implementation authority unless a
   non-GPL-family license is confirmed against `deny.toml`. The candidate must
   preserve existing Anderson/SNOBAL density constants, dry/wet compaction
   multipliers, wet liquid-compaction formula, fresh-snow-density constants,
   wet substep count, and the active `522 kg m^-3` cap.
4. Conservation ledger: package evidence must independently reconstruct
   `runtime_swe_after = runtime_depth_after_m * runtime_density_after_kg_m3 /
   1000` and `runtime_swe_after = selected_coe_runtime_swe` within tolerance
   for trace rows. Melt, routed liquid, rain, retained/released liquid,
   sublimation, and snowpack SWE loss must remain identical to the activated
   density baseline for the same selected melt boundary except for roundoff.
5. Coupled evidence gate: package evidence must include real direct-production
   WAT and trace rows proving the opt-in candidate reached the snow partition
   and comparing against the activated default on paired observed snow-depth
   surfaces. The package must report induced under-persistence recovery,
   `harvard_hardwood` explicitly, over-persistence non-worsening, threshold
   authority, conservation status, and protected-boundary scans.
6. Promotion boundary: this addendum cannot activate the candidate. If induced
   under-persistence is not reduced, over-persistence worsens, threshold
   authority is missing or fitted, conservation fails/missing, or protected
   boundaries drift, the package must close `HOLD` or non-promotion.
7. Protected boundaries: this addendum does not authorize default activation,
   density-cap changes, spring wet-compaction acceleration, two-layer
   snow-surface structure, sublimation, albedo/radiation/canopy/phase/
   rain-heat/longwave/frost/Qwet/frzftp changes, parser/runfile/user controls,
   fixture edits, compatibility-runtime changes, public output-schema changes,
   or frost attribution.

## SNOWDENSITY-10.3.19 Harder-Pomeroy Phase Default Addendum

Status: draft (2026-06-28). This addendum activates `harder_pomeroy_hourly` as
the direct-production no-env phase partition default when composed with the
already activated melt+density bundle.

1. Default selector: absent and empty `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL`
   values select `harder_pomeroy_hourly`. Explicit `harder_pomeroy_hourly`
   selects the same branch. Explicit `legacy_rst` remains the rollback/test
   selector. Unknown values fail closed.
2. Composition: the default phase is evaluated only with the activated
   `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`
   direct-production bundle. This addendum does not change melt, density,
   density-cap, liquid-capacity, sublimation, albedo, canopy, radiation,
   rain-heat, longwave, frost, Qwet/frzftp, or compatibility-runtime behavior.
3. Primary Policy-B gate: the cross-SNOTEL `INV-SNOWFREEZE-050`
   forcing-robust rubric is the promotion gate for this phase-default change.
   A real direct-production no-env rerun must be at least as good as the prior
   activated bundle with `legacy_rst` phase on robust fail count and robust
   ordinal score. The 10.3.18 decision basis is `15/179` for Harder-Pomeroy
   versus `17/172` for the activated bundle.
4. Conservation: package evidence must prove active-hour phase mass closure:
   selected hourly `hrrain + hrsnow / 10` reconstructs active hourly
   precipitation depth within roundoff for both the new no-env default and the
   explicit `legacy_rst` rollback branch.
5. Release notes: closure must carry forward that humid-New-England depth
   regression is a non-representative roadmap item, not a blocker under this
   cross-SNOTEL gate, and that the cross-SNOTEL density median bias rises to
   about `+23.6 kg m^-3`, with recovery tracked separately.
6. Protected boundaries: this addendum does not authorize parser/runfile/user
   selectors, a `.run` disable option, fixture edits, public output-schema
   changes, density-cap changes, default changes outside the direct-production
   phase selector, frost attribution, or site calibration.

## SNOWDENSITY-10.3.20 Sublimation Stage B Unlock Addendum

Status: draft (2026-06-28). This addendum authorizes diagnosis of the Stage A
sublimation implementation, tests the current Harder-Pomeroy default composed
with sublimation, and unlocks one opt-in Stage B candidate.

1. Current default preservation: absent direct-production selectors continue to
   select `coe_liquid_holding_capacity_v1`,
   `physics_bulk_density_compaction_v1`, and `harder_pomeroy_hourly`. Explicit
   `legacy_coe`, `legacy_wepp`, and `legacy_rst` rollback/test selectors remain
   available.
2. Diagnostic composition: `coe_open_sublimation_stage_a_v1` must be scored both
   with explicit `legacy_rst` phase for 10.3.16 lineage diagnosis and with
   `harder_pomeroy_hourly` phase for partition+sublimation composition. The
   diagnostic must decompose degradation by site, signature, SWE/depth/density
   residual component, and sublimation magnitude.
3. Stage B candidate: `coe_open_sublimation_stage_b_v1` is opt-in only. Its only
   authorized delta from Stage A is the SNOBAL/Marks active surface-layer
   temperature/cold-content gate for the sublimation vapor-pressure surface. The
   active layer depth is bounded by current snow depth and the Marks/SNOBAL
   `0.25 m` active-layer ceiling. It must not introduce observed-site fitting,
   public snow-temperature output state, density-cap changes, or default changes.
4. Provenance: reading libsnobal source for equation reference requires package
   evidence of the clone commit and CC0 declaration. The local clone used by this
   package is `/home/workdir/pysnobal` commit
   `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`; `setup.py` declares
   `license="CC0 1.0"` and `deny.toml` allow-lists `CC0-1.0`.
5. Primary gate: promotion requires a real cross-SNOTEL direct-production WAT
   rerun proving the candidate beats the current default on the
   `INV-SNOWFREEZE-050` forcing-robust rubric: robust fail count no worse and
   robust ordinal score higher. The bidirectional guardrail and conservation
   gates are binding.
6. Conservation: sublimation must remain a vapor mass export bounded by
   available snowpack SWE and excluded from routed liquid. Active-hour
   precipitation partition closure must continue to hold.
7. Protected boundaries: this addendum does not authorize fixture edits, public
   output-schema changes, density-cap changes, frost changes, parser/runfile/user
   CLI selectors, `.run` disable controls, Qwet/frzftp, compatibility-runtime
   changes, or site calibration.

## SNOWDENSITY-10.3.22 Climate-Class Density Specialization Addendum

Status: draft/HOLD non-promotion (2026-06-28). This addendum reserves
`physics_bulk_climate_class_density_v1` as an opt-in density specialization
candidate. Source thresholds are verified, but promotion and activation remain
unauthorized because the real cross-SNOTEL primary gate, bidirectional
densification flip, and persistence guardrail did not pass.

1. Candidate scope: the opt-in candidate must compose with the current
   no-env default (`coe_liquid_holding_capacity_v1`,
   `physics_bulk_density_compaction_v1`, and `harder_pomeroy_hourly`) and must
   preserve `legacy_wepp` rollback. The no-env default is unchanged.
2. Class coverage: all six Sturm 1995 class labels are in scope: tundra,
   taiga, alpine, maritime, prairie, and ephemeral. Classes absent from the
   SNOTEL/cancov corpus may be covered by reference authority only, not claimed
   as observed-rubric validated.
3. Assignment authority: runtime class assignment must be forcing-derived from
   the run's own wind, precipitation, and air-temperature climate under Sturm
   1995 numeric decision-tree authority. The verified thresholds are `Tc=10
   degC`, `CDM < 30 degC-month` for ephemeral, `CDM >= 125 degC-month` for the
   low-temperature seasonal branch, and `SPR >= 2 mm d^-1` for high
   precipitation. The 1995 source brackets wind separation between `0.5` and
   `2.0 m s^-1` and uses vegetation as a mapping proxy; direct-runtime
   actual-wind classification must fail closed inside that unresolved interval.
   NSIDC-0768 is an independent cross-check only and must not be used as
   geographic lookup, site lookup, or calibration input.
4. Density authority: Sturm 2010 Table 4 and Eq. 6 supply density trajectory
   parameters for alpine, maritime, prairie, tundra, and taiga. Local authority
   does not supply an ephemeral parameter row, and the paper notes ephemeral
   measurements were excluded. Ephemeral runtime use must therefore retain the
   existing process-first fresh-snow/Anderson compaction behavior as a
   documented fallback rather than fabricated Sturm parameters.
5. Process-first translation: the preferred implementation is a translation
   into Anderson/SNOBAL compaction coefficients that reproduces the published
   class trajectory. Any raw Sturm density-form fallback must be explicitly
   flagged and cannot be hidden as a coefficient solution.
6. Promotion gate: a real cross-SNOTEL direct-production `INV-SNOWFREEZE-050`
   run must beat the current `15/179` default profile, fix the split-sign
   densification trajectory in both directions, create no new persistence tail,
   and close whole-model snow-state conservation.
7. Protected boundaries: this addendum does not authorize fixture edits, public
   output-schema changes, density-cap changes, frost changes, parser/runfile/user
   CLI selectors, `.run` controls, Qwet/frzftp, compatibility-runtime changes,
   melt/phase/canopy/radiation changes, site calibration, or default activation.
	   If authority or evidence is missing, the package closes `HOLD` or
	   non-promotion.

## Paradigm 2 Stage 1 Layered Snow-Density Addendum

Status: draft/opt-in candidate (2026-06-28). This addendum reserves
`physics_bulk_multilayer_density_v1` for the Paradigm 2 Stage 1 density-only
candidate. It does not authorize default activation.

1. Candidate scope: the candidate composes with the current no-env default
   (`coe_liquid_holding_capacity_v1`, `physics_bulk_density_compaction_v1`, and
   `harder_pomeroy_hourly`) and preserves `legacy_wepp` rollback. The no-env
   default is unchanged.
2. Layer home: persistent snow layers live only in the ADR-0026 winter column
   as `DirectSnowLaneState.layers`, following the frost variable-length-Vec
   precedent. Fixed coarse-slot projection is invalid.
3. Authorized density delta: relative to `physics_bulk_density_compaction_v1`,
   the only process delta is applying the same Anderson/SNOBAL fresh-snow,
   destructive metamorphism, overburden compaction, and wet-compaction constants
   per layer with `snow_layer_local_overburden = sum(overlying layer mass)`.
   Constants, active density cap, melt, phase, albedo, canopy, radiation, liquid,
   and frost paths are not changed by this stage.
4. Aggregate/public surface: WAT/public output schemas remain aggregate-only.
   When the candidate is selected, aggregate `snow_runtime_swe`,
   `snow_runtime_depth`, and `snow_runtime_density` must reconstruct from the
   layer stack; invalid or inconsistent layer aggregates fail closed.
5. Layer management: new snow may be represented as a surface layer, ablation
   removes mass from the surface stack, retained boundary mass may be added to
   the surface layer, and old bottom layers may merge only to maintain a bounded
   layer count. Merges must conserve mass and thickness; no threshold or
   parameter may be fitted to SNOTEL/cancov fixtures.
6. Promotion gate: a real cross-SNOTEL+cancov direct-production
   `INV-SNOWFREEZE-050` run must beat the current `15/179` default profile,
   prove the split-sign densification trajectory improves in both directions,
   create no new persistence tail, close whole-model snow-state conservation,
   and meet the ADR-0025 performance gate.
7. Protected boundaries: this addendum does not authorize fixture edits, public
   output-schema changes, density-cap changes, frost changes, parser/runfile/user
   CLI selectors, `.run` controls, Qwet/frzftp, compatibility-runtime changes,
   melt/phase/canopy/radiation changes, site calibration, or default activation.
   If evidence is missing or gates fail, the package closes `HOLD` or
   non-promotion.

## Paradigm 2 Stage 2 Snow-Frost Insulation Profile Addendum

Status: draft/opt-in candidate (2026-06-28). This addendum reserves
`layered_resistance_v1` for the Paradigm 2 Stage 2 snow-to-frost insulation
candidate. It does not authorize default activation.

1. Candidate scope: the candidate composes only with explicit
   `physics_bulk_multilayer_density_v1` and preserves the current no-env default,
   `legacy_wepp` rollback, and the absent-selector bulk snow-to-frost handoff.
2. Entry gate: before frost coupling is evaluated, real direct-production trace
   evidence must show multi-layer Stage 1 rows where basal layer density exceeds
   surface layer density. If the gradient does not form, the package closes
   `HOLD` or repairs Stage 1 local-overburden differentiation first.
3. Authorized coupling delta: relative to the bulk handoff, the only Stage 2
   frost delta is replacing bulk snow insulation with the prior-day layer-stack
   resistance integral. The implementation uses the Sturm et al. 1997
   density-to-conductivity relation already used by WEPP frost heat paths and
   converts the resistance to an equivalent bulk density for the existing frost
   solver.
4. Protected timing: the frost solver consumes the prior-day snow layer stack.
   Same-day snow accumulation, melt, liquid routing, phase partition, and
   density changes remain decoupled from the frost step unless a later contract
   amendment changes that boundary.
5. Promotion gate: forcing-robust frost-depth observation signatures are primary
   under `INV-SNOWFREEZE-050`. Cross-SNOTEL snow results are a no-regression
   guardrail, not the promotion gate.
6. Protected boundaries: this addendum does not authorize fixture edits, public
   output-schema changes, density-cap changes, parser/runfile/user CLI selectors,
   `.run` controls, Qwet/frzftp, compatibility-runtime changes, Stage 3 per-layer
   thermal solve, Stage 3 melt/liquid routing, site calibration, or default
   activation. If evidence is missing or gates fail, the package closes `HOLD`
   or non-promotion.

## Paradigm 2 Stage 3 Liquid Routing and Meltwater Temperature Addendum

Status: draft/opt-in candidate (2026-06-29). This addendum reserves
`layered_thermal_liquid_v1` for the Paradigm 2 Stage 3 per-layer thermal,
liquid-routing, refreeze, and meltwater-temperature diagnostic candidate. It
does not authorize default activation.

1. Candidate scope: the candidate composes only with explicit
   `physics_bulk_multilayer_density_v1` layer state and preserves the current
   no-env default, rollback selectors, and disabled Stage 3 path.
2. De-risked mass boundary: the existing CoE melt/rain mass path remains
   authoritative for snow SWE, routed melt, downstream liquid forcing, and WAT
   water-balance surfaces. Stage 3 may route and temperature-tag that liquid but
   must not replace CoE melt with an energy-balance melt.
3. Authorized diagnostic delta: Stage 3 may carry per-layer temperature, cold
   content, retained liquid, and refreeze; solve inter-layer thermal exchange
   using Stage 0 surface-energy primitives and authority constants; route liquid
   through the Stage 1 stack bounded by the 10.3.8 holding-capacity authority;
   and produce a typed intensive meltwater-flux temperature.
4. Flux-temperature boundary: meltwater temperature is an internal diagnostic
   source for the future stream-temperature program. Full in-stream routing,
   public output-schema changes, and HBP production temperature publication are
   separate packages.
5. Conservation gate: mass, liquid storage, and energy ledgers must close from
   produced operands, including latent heat of refreeze, cold-content change,
   surface-energy/conduction terms, retained liquid, refrozen liquid, and routed
   liquid.
6. Promotion gate: Stage 3 is judged on conservation, physically reasonable
   meltwater temperature, forcing-robust runoff/melt timing no-worse evidence,
   snow-rubric no-regression as a guardrail only, and ADR-0025 H2637
   performance. It cannot claim snow-density promotion.
7. Protected boundaries: this addendum does not authorize fixture edits, public
   output-schema changes, density-cap changes, frost/default changes,
   parser/runfile/user CLI selectors, `.run` controls, Qwet/frzftp,
   compatibility-runtime changes, site calibration, or default activation. If
   evidence is missing or gates fail, the package closes `HOLD` or
   non-promotion.

## Paradigm 2 Stage 3-Decouple Snow-Neutral Water-Temperature Addendum

Status: draft/opt-in capability (2026-06-29). This addendum keeps
`layered_thermal_liquid_v1` as the internal Stage 3 thermal/liquid selector but
removes the requirement that the water-temperature arm depend on
`physics_bulk_multilayer_density_v1`. It does not authorize default activation.

1. Candidate scope: the decoupled arm composes with the current no-env bulk
   density default, preserves the disabled Stage 3 rollback, and retains the
   current no-env snow/frost behavior when the selector is absent.
2. Bulk-equivalent layer carrier: when the selected density model is bulk, the
   Stage 3 runtime must synthesize or carry a persistent top-to-bottom layer
   geometry whose layer masses/thicknesses reconstruct the bulk aggregate SWE
   and depth and whose per-layer densities all equal the selected aggregate
   bulk density. The layer carrier may preserve thermal/liquid/cold-content
   state, but it may not apply Stage 1 local-overburden densification.
3. Authorized diagnostic delta: the decoupled arm may run the Stage 3
   thermal/liquid/refreeze solver and assign typed meltwater-flux temperature.
   It must keep the existing CoE melt/rain mass path authoritative and must not
   replace aggregate snow SWE/depth/density, frost handoff behavior, or public
   WAT/HBP schemas.
4. Flux-temperature boundary: typed meltwater temperature remains diagnostic
   input for the future stream-temperature program. Full in-stream routing and
   public output-schema changes remain separate packages.
5. Acceptance gate: the real cross-SNOTEL/cancov forcing-robust snow rubric
   must exactly match the current bulk default (`15` robust fails / `179`
   score) with no worse robust cells; mass, liquid, and energy ledgers must
   close; meltwater temperature must be finite and physically bounded; runoff/
   melt-timing signatures must be no-worse; and ADR-0025 H2637 performance must
   pass.
6. Protected boundaries: this addendum does not authorize fixture edits, public
   output-schema changes, density-cap changes, frost/default changes,
   parser/runfile/user CLI selectors, `.run` controls, Qwet/frzftp,
   compatibility-runtime changes, site calibration, Stage 1 density dependence,
   or default activation. If evidence is missing or gates fail, the package
   closes `HOLD`.

## Paradigm 2 Multilayer Promotion Addendum

Status: production-supported opt-in capability (2026-06-29). This addendum
ratifies the Stage 3-Decouple water-temperature arm for production opt-in use.
It does not authorize default activation.

1. Activation posture: the promoted arm is opt-in only. The no-env bulk default
   remains `coe_liquid_holding_capacity_v1 +
   physics_bulk_density_compaction_v1 + harder_pomeroy_hourly`, with Stage 3
   disabled unless the supported internal selector requests it.
2. Selector exposure: `OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL` is a supported
   internal production control for `disabled` and `layered_thermal_liquid_v1`.
   Empty or absent values select disabled. Unknown or non-UTF-8 values fail
   closed. Parser/runfile/user CLI/WEPPpy exposure is deferred until the
   stream-temperature program owns the downstream consumer.
3. Supported output: the promoted arm publishes nullable hillslope WAT parquet
   `MeltwaterTemperature` in `degC`, sourced from
   `DirectSnowStage3Diagnostics.meltwater_temperature_c`. The field is null
   when the arm is disabled or no routed meltwater temperature exists. Unit
   metadata and output-registry validation are required.
4. Protected serialization boundary: HBP binary/watershed serialization and
   full in-stream temperature routing remain out of scope. This package only
   produces the hillslope source field.
5. Acceptance gate: the real cross-SNOTEL/cancov forcing-robust snow rubric
   must exactly match the current bulk default (`15` robust fails / `179`
   score) with `0` worse robust cells; mass, liquid, and energy ledgers must
   close; produced meltwater temperatures must be finite and `<= 0 degC`;
   H2637 performance must stay within ADR-0025 budget; rollback and
   fail-closed selector behavior must be verified; release notes must describe
   the new opt-in mode and unchanged default.
6. Protected boundaries: this addendum does not authorize Stage 1 density
   dependence, CoE melt/rain mass replacement, density/frost behavior changes,
   default activation, fixture/site calibration, parser/runfile/user controls,
   `.run` controls, Qwet/frzftp changes, compatibility-runtime changes,
   HBP/watershed serialization, or full stream-temperature routing. If evidence
   is missing or gates fail, the package closes `HOLD`.

## SNOW-SURFACE-EB-03 Shared Thermal and Energy Composition Addendum

Status: implementation attempted / runtime `HOLD`; no default promotion.

Real direct-production EB-03 execution established that the selected Stage 3
provider is not sufficient for the admitted composition: the S cell reaches
the provider's absolute-zero cold-content bound after 18 days while material
SWE remains. This is a hard provider/energy-composition failure, not authority
for a temperature clamp. Items 1-8 below remain the acceptance contract for
any future reopening; EB-04 is not admitted on the current implementation.

1. The thermal provider is the top layer of
   `layered_thermal_liquid_v1`. Longwave and sublimation cannot activate
   without that provider.
2. `snow_surface_longwave_model` and `snow_sublimation_model` are independent
   axes. Absent, empty, or `disabled` values retain the current behavior;
   unknown values fail closed.
3. The longwave axis consumes the exact equations, units, cadence, sky-view
   translation, `T_c=T_a` identity, derived-emissivity guards,
   `R_a,min=1e-9 MJ m^-2 d^-1`, and polar-night unavailable branch in
   `SC-SNOWENERGY-001` v2.
4. The sublimation axis reuses the admitted Marks/SNOBAL neutral exchange but
   replaces the Stage B `min(T_a,0)` reconstruction with the shared Stage 3
   `T_s`. One bounded signed vapor mass flux produces both vapor loss and
   latent heat of sublimation. The legacy Stage A/B melt variants cannot be
   selected with the new axis.
5. Stage 3 sums absorbed shortwave, optional net longwave, and optional latent
   heat before updating cold content. It retains the existing protected
   boundary that positive excess does not replace CoE melt.
6. Sublimated ice is removed from layer and aggregate snow storage. Its
   associated cold-content export is explicit; vapor never enters melt,
   retained/released liquid, routed liquid, or refreeze.
7. The implementation must publish enough internal hourly diagnostics to
   reconstruct atmospheric/sub-canopy/outgoing/net longwave, vapor mass,
   latent heat, potential/applied surface energy, cold-content export, and
   closure residuals.
8. EB-03 is implementation/campaign-readiness only. EB-04 owns scientific
   factorial execution and EB-05 owns any promotion decision.

## SNOW-SURFACE-EB-03A Active-Layer Thermal Coupling Addendum

`INV-SNOWFREEZE-086` supersedes only EB-03's failed identification of the
first snowfall-event layer as the thermal surface control volume. Persistent
depositional layers remain valid density/history state, but the energy
provider integrates them into the Marks/SNOBAL upper-`0.25 m` active volume,
splitting a boundary-crossing layer conservatively when required. A pack
shallower than `0.25 m` is wholly active.

The coupled substep uses `SC-SNOWENERGY-001` v3 `G_0`, with positive transfer
into the active volume and the identical negative transfer in the lower
volume. The complete pack ledger excludes internal `G_0` after cancellation.
Mass thresholds choose `60`, `15`, or `1 minute` evaluation, and all enabled
surface fluxes are recomputed from the updated active state. These constants
are implementation authority, not user calibration controls.

The package retains EB-03's exact-one vapor/latent rule and CoE melt boundary.
It may not use the SnowClim temperature replacement or cold-content tax,
absolute-zero clamping, observation-derived coefficients, or a user-supplied
thermal depth.

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
6. Thaw-complete single-owner closure:
   a material outcome with zero final frost preserves the original coarse
   layer basis until frost ingress applies the outcome layer projection and
   `soil_water_after_frwatc` scalar together exactly once; residual storage
   over newly unfrozen thickness is neither pre-credited nor debited.
7. Typed failure posture:
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

Status: accepted (2026-06-29 ratification). This addendum scopes the observation-anchored
validation method reopened by `GAP-SNOWFREEZE-002` and bound by
`INV-SNOWFREEZE-047`. It changes no physics; `INV-SNOWFREEZE-006` remains the
frost heat-flow formulation authority. The `TOL-SNOWFREEZE-007/008/009`
evaluation bands are accepted as observation noise-floor bands by the
2026-06-29 frost ratification package.

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

`frdp` is the bottom extent of the modeled frozen domain. The fine-layer state
machine may thaw from the surface downward and grow `thdp` while `frdp` remains
fixed. Late-thaw residuals from a bare `frdp > 0` detector must therefore be
reviewed for `thdp` correspondence before being classified as frost physics.

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

### Ratification disposition

- `TOL-SNOWFREEZE-007/008/009` are accepted as observation-evaluation bands.
- The Step 1 `>0.25` systematic-timing-fraction cutoff is not ratified; it is a
  diagnostic-script aggregation rule only.
- Frost-tube-versus-ice-front bias remains a named uncertainty, not a zero-bias
  assumption or a blocker.
- Request-only Dun-2010 Pullman/Morris acquisition and per-site climate caveat
  refinements remain evidence-depth improvements, not blockers to the accepted
  method.
- Snow-depth daily timing/stage and signed residual reporting are accepted under
  `INV-SNOWFREEZE-048`; missing paired snow rows remain inconclusive.
- `TOL-SNOWFREEZE-010` is accepted as a rubric band for SNOTEL SWE/density fork
  characterization; it remains an evaluation band, not a tuning target.

## GAP-SNOWFREEZE-002 Snow/Frost Fidelity Evaluation Rubric Addendum

Status: accepted (2026-06-29 ratification). This addendum defines how modeled snow (and frost)
fidelity to observations is *scored*, under `INV-SNOWFREEZE-050`. It changes no
physics. Thresholds are accepted noise-floor evaluation bands; they are not
calibration targets.

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
  accepted pass-levels in `TOL-SNOWFREEZE-011` and the per-quantity bands
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

Thresholds are accepted noise-floor estimates refined from the frost and SNOTEL
observation corpora. They are evaluation bands, not calibration objectives; the
`INV-SNOWFREEZE-049` anti-tuning rule applies. Default activation may use the
Policy B analog only when residuals are attributed and bounded and full
no-regression/conservation gates pass; zero residual closure is not required.

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `SNOWFREEZE-EB03-COMPOSITION` | SNOW-SURFACE-EB-03 shared thermal and energy composition addendum | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-085, OBL-SNOWFREEZE-P-059` | `none` | The addendum is consolidated into the normative contract body and introduces no free-standing binding outside these identifiers. |
| `SNOWFREEZE-EB03A-COUPLING` | SNOW-SURFACE-EB-03A active-layer thermal coupling addendum | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-086, OBL-SNOWFREEZE-P-060` | `none` | The addendum binds the active thermal control volume, coupled conduction, stability hierarchy, and prohibited limiter alternatives. |
| `SNOWFREEZE-EB04V-DENSITY-LEDGER` | SNOW-SURFACE-EB-04V density-structure mechanics package | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-087, OBL-SNOWFREEZE-P-061` | `none` | Behavior-neutral internal diagnostics only; `TOL-SNOWFREEZE-012` remains linked through the invariant/obligation. Retained observations remain diagnostic-only and cannot authorize tuning or promotion. |
| `SNOWFREEZE-EB04W-ACCUMULATION-MELT-LEDGER` | SNOW-SURFACE-EB-04W accumulation and mountain under-persistence package | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-088, OBL-SNOWFREEZE-P-062` | `none` | Behavior-neutral internal diagnostics only; `TOL-SNOWFREEZE-013` remains linked through the invariant/obligation. Formula components are not pure-energy fluxes and modeled-zero drift does not resolve physical redistribution. |
| `SNOWFREEZE-STAGE3-LIQUID-SIGNED-HOUR-TRACE` | SNOW-STAGE3-LIQUID-SIGNED-HOUR-TRACE-CLOSURE package | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-090, OBL-SNOWFREEZE-P-063` | `none` | Additive internal JSONL v4 diagnostics only; `TOL-SNOWFREEZE-015` remains linked through the invariant/obligation. Exact operands and independent reconstruction cannot authorize snow-physics, calibration, promotion, or protected-output changes. |
| `SNOWFREEZE-MASS-TRANSITION-LEDGER-PERSISTENCE` | SNOW-MASS-TRANSITION-LEDGER-PERSISTENCE package | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-091, OBL-SNOWFREEZE-P-064` | `dual-review` | One authoritative production result owns two exact linked compact ledgers; `TOL-SNOWFREEZE-015/016` remain linked through the invariant/obligation. The existing hourly schema-v4 payload becomes optional at collection without changing physics, guards, or protected outputs. |
| `SNOWFREEZE-WET-COMPACTION-OPERAND-CLOSURE` | SNOW-WET-COMPACTION-OPERAND-AUTHORITY-AND-DUPLICATE-ALIAS-CLOSURE package | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-092, OBL-SNOWFREEZE-P-065` | `dual-review` | Generated positive melt plus snow-contact rain reaches wet compaction exactly once under `TOL-SNOWFREEZE-017`; routed, state-loss, and retained-store aliases are prohibited. The Snowbird scaled lane remains development-only and cannot prove physics. |
| `SNOWFREEZE-21N-MELT-OWNER` | SNOW-COE-STAGE3-MELT-OWNER-AUTHORITY-RECONCILIATION package | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-093, OBL-SNOWFREEZE-P-066` | `dual-review` | Stage 3 is the sole future melt owner, while implementation remains under hard governance `HOLD`. CoE remains byte-identical compatibility runtime behavior until complete energy, residual-snow, same-substep liquid, linked-ledger, and cutover gates pass atomically. |
| `SNOWFREEZE-STAGE3-EVALUATION-SHADOW` | SNOW-STAGE3-EVALUATION-SHADOW-AUTHORITY package | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-094, OBL-SNOWFREEZE-P-067, OBL-SNOWFREEZE-C-009` | `dual-review` | Two tagged default-off evaluation operators are admitted with evaluator-only custody. Runtime implementation remains under hard governance `HOLD`; cross-interval persistence, seasonal/terminal claims, production consumers, and cutover remain held. |
| `SNOWFREEZE-STAGE3-SHADOW-OBSERVABILITY` | SNOW-STAGE3-SHADOW-SOLVER-EXTRACTION-AND-OBSERVABILITY package | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-095, OBL-SNOWFREEZE-P-068, OBL-SNOWFREEZE-C-010` | `dual-review` | The bounded operators may be realized behind a typed default-off request with enabled-only internal schema-v5 evidence. Exact disabled schema v4, evaluator custody, typed primitive errors, module-size closure, and real-consumer anti-alias reconstruction are binding; persistence, physical chronology, seasonal/terminal claims, production consumers, and cutover remain held. |
| `SNOWFREEZE-STAGE3-OPERATOR-RECONCILIATION` | SNOW-STAGE3-TURBULENT-CARRIER-LINEAGE-AND-OPERATOR-RECONCILIATION package and canonical reconciliation addendum | `active` | `maps-to-existing-INV` | `INV-SNOWFREEZE-096, OBL-SNOWFREEZE-P-069, OBL-SNOWFREEZE-C-011` | `flagged-binding-addition` | Additive enabled-only schema-v6 tuples and one shared turbulent solver may expose exact carrier/state lineage for independent `S/F/Q` operator reconciliation; `TOL-SNOWFREEZE-018` and `TOL-SNOWFREEZE-019` remain linked through the invariant and obligations. Exact disabled schema v4, historical schema-v5 semantics, public APIs, all production physics/defaults/outputs, CoE ownership, and seasonal/terminal/cutover holds remain binding; independent science review is mandatory before implementation. |

## Known Gaps

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SNOWFREEZE-001 | Per-invariant comparator vectors for hourly winter outputs (`hrmelt`, frost depth/thaw depth, freeze-thaw cycles) are not yet curated. | Limits immediate automated regression depth on hourly-heavy winter internals. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-SNOWFREEZE-002 | **Open but attributed/bounded (2026-06-29).** The observation arc ratified `INV-SNOWFREEZE-047/048/050` and decomposed the remaining frost-depth residuals rather than forcing zero closure. Sleepers South/W9 timing cells were attributable after current-snow rerun; dynamic residue coupling reduced the candidate-defect set from `18` to `13`; the snow-buried thaw-late majority routed to sparse snow-persistence / forcing-limited evidence with no established Sleepers spring under-melt defect; two snow-free persistent cells remain deferred Qwet/wet-heat candidates; two H1b cells are a narrow `frdp` bottom-extent metric edge with `thdp` top thaw present, plus `15` true-stall days out of `570` branch-3 warm/material days; two early-onset cells remain bounded onset residuals. Morris remains snow-control blocked for frost attribution, and Mandan/Reynolds remain inconclusive without paired snow depth. The accepted validation method is bound by `INV-SNOWFREEZE-047/048/050`; residuals are bounded under the Policy B analog rather than closed to zero. | No longer blocks frost direct-production default activation when `INV-SNOWFREEZE-084` full no-regression/conservation gates pass. Continues to block zero-residual claims, Qwet completion claims, and frost attribution for snow-control-blocked/no-paired-snow sites. | open-but-attributed/bounded | `[DIRECT][Ran] + [INFERENCE][Static]` |
| GAP-SNOWFREEZE-003 | Snow drifting equations are documented in Chapter 3 but explicitly inactive in the August 1995 lineage; active-path authority for openWEPP is unresolved. | Drift-related claims cannot be promoted as active behavior yet. | non-promotable | `[DIRECT][Static]` |
| GAP-SNOWFREEZE-004 | Cross-contract boundary ownership with `SC-SOIL-001` and `SC-RUNOFFPART-001` is explicit, but executable cross-contract comparator vectors for frost-hourly internals are still incomplete. | Promotable contract authority exists; evidence depth for coupled frost vectors remains limited pending SIMIMPL32 and SIMIMPL35. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SNOWFREEZE-005 | `Dsavail` alias is fixed (`snow.hourly.depth_available_m`) and SIMIMPL29 emits the hourly family, but comparator-tier depth/density/melt vector breadth remains limited for broad climate regimes. | Residual risk is evidence-depth, not missing alias/state publication. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SNOWFREEZE-006 | The admitted Stage 3 melt-owner target is not implemented: current authoritative production still generates melt with CoE, and its production Stage 3 path lacks the complete target carrier and residual thin-pack phase disposition. A separate default-off evaluation carrier includes sensible and precipitation-advected heat but failed its frozen Snowbird plausibility screen and is not target-runtime conformance. | Blocks any Stage 3 melt-owner activation or claim that runtime conforms to `INV-SNOWFREEZE-093`. | non-promotable until one atomic implementation/cutover package closes `OBL-SNOWFREEZE-P-066` | `[DIRECT][Ran] + [INFERENCE][Static]` |
| GAP-SNOWFREEZE-007 | **Bounded realization closed 2026-08-06.** Runtime now realizes both `INV-SNOWFREEZE-094/095` operators behind the typed default-off request, with complete operator/pairing tags, per-arm/per-term and cold-content export, typed turbulent-error custody, deterministic support/fingerprints, exact disabled schema-v4 and authoritative-output identity, and independent schema-v5 consumer reconstruction with negative production/adjacent-alias proof. The evaluation remains daily bounded: cross-interval persistence, complete same-substep phase/liquid chronology, restart/reappearance, physical seasonal chronology, and terminal/receiving-surface behavior remain intentionally outside this realization. | No longer blocks claims that runtime conforms to the bounded operators. Unadmitted authority continues to block cross-day/seasonal, physical-chronology, terminal, promotion, and cutover claims. | bounded evaluation capability realized; persistence and terminal authority require later amendments | `[DIRECT][Ran] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-08-06` | `129` | `Codex` | SNOW-STAGE3-TURBULENT-CARRIER-LINEAGE-AND-OPERATOR-RECONCILIATION amendment: added `REF-SNOWFREEZE-STAGE3-OPERATOR-RECONCILIATION`, `INV-SNOWFREEZE-096`, `OBL-SNOWFREEZE-P-069`, `OBL-SNOWFREEZE-C-011`, and `TOL-SNOWFREEZE-018/019`; admitted an additive enabled-only boxed companion and schema-v6 ordered substep observability through one shared private turbulent solver; and canonically bound exact fields/units, applicability and meltout N/A, turbulent termination, endpoint equations, legacy bridge, common support, frozen `S/F/Q` estimands, decision precedence, and sign/materiality predicates. Disabled schema v4, historical schema-v5 semantics, existing public APIs, production physics/defaults/outputs, CoE ownership, and seasonal/terminal/cutover holds remain unchanged. |
| `2026-08-06` | `128` | `Codex` | Runtime evidence closure: closed the bounded portion of `GAP-SNOWFREEZE-007` after exact disabled schema-v4 and authoritative WAT/HBP/PASS identity, typed evaluator-only custody, per-term/fingerprint/support reconstruction through the real schema-v5 consumer, typed-error preservation, module-size compliance, dual review, and independent consumer QA passed. Cross-day persistence, complete physical chronology, seasonal/terminal claims, receiving-surface disposition, production ownership, and cutover remain held. |
| `2026-08-06` | `128` | `Codex` | SNOW-STAGE3-SHADOW-SOLVER-EXTRACTION-AND-OBSERVABILITY amendment: added `REF-SNOWFREEZE-STAGE3-SHADOW-OBSERVABILITY`, `INV-SNOWFREEZE-095`, `OBL-SNOWFREEZE-P-068`, and `OBL-SNOWFREEZE-C-010`; admitted bounded runtime realization of the two v127 evaluation operators behind a typed default-off request; bound exact disabled schema-v4 identity, enabled internal schema-v5 component/fingerprint/support evidence, typed turbulent-error custody, real-consumer anti-alias reconstruction, and solver modularization. CoE remains authoritative; persistence, complete physical chronology, seasonal/terminal claims, ground-flux inference, production consumers, and cutover remain held. |
| `2026-08-06` | `127` | `Codex` | SNOW-STAGE3-EVALUATION-SHADOW-AUTHORITY amendment: scoped the v124 second-state prohibition to undeclared or production-readable parallel state while preserving one authoritative production snow calculation and linked ledgers; added `REF-SNOWFREEZE-STAGE3-EVALUATION-SHADOW-AUTHORITY`, `INV-SNOWFREEZE-094`, `OBL-SNOWFREEZE-P-067`, and `OBL-SNOWFREEZE-C-009`; and admitted only tagged, default-off, evaluator-owned same-state paired-carrier and bounded sequential resolved-shadow operators. Cross-interval persistence, restart/reappearance, seasonal and terminal claims, runtime implementation, production consumers, CoE ownership change, and cutover remain held. |
| `2026-08-04` | `126` | `Codex` | SNOW-COE-STAGE3-MELT-OWNER-AUTHORITY-RECONCILIATION amendment: admitted Stage 3 as the sole future melt owner; prospectively superseded every earlier clause only where it assigned lasting CoE target ownership, including `INV-SNOWFREEZE-080/085/086`; added `INV-SNOWFREEZE-093` and `OBL-SNOWFREEZE-P-066`; and bound cold-content-first complete energy, bounded phase conversion, same-substep liquid disposition, exact-one linked ledgers, residual-snow/terminal-energy closure, and no-dual-owner guards. The runtime is unchanged and remains on implementation hold with CoE as compatibility behavior only. |
| `2026-08-04` | `125` | `Codex` | SNOW-WET-COMPACTION-OPERAND-AUTHORITY-AND-DUPLICATE-ALIAS-CLOSURE amendment: upgraded PySnobal 0.2.3 source/chronology authority, added the exact generated-positive-melt-plus-snow-contact-rain driver, `INV-SNOWFREEZE-092`, `OBL-SNOWFREEZE-P-065`, `TOL-SNOWFREEZE-017`, aliases, guard/boundary dispositions, and the wet-compaction operand addendum. The amendment prohibits the confirmed `snowpack_state_loss + routed_melt` duplicate and all adjacent routed/storage aliases while preserving melt, SWE, routing, mass ledgers, constants/cap, defaults, and public schemas. |
| `2026-08-03` | `124` | `Codex` | SNOW-MASS-TRANSITION-LEDGER-PERSISTENCE amendment: added `REF-SNOWFREEZE-MASS-TRANSITION-LEDGER-PERSISTENCE`, compact solid-to-liquid and liquid-disposition ledgers, compact Stage-3 production outcome, optional verbose capture authority, `INV-SNOWFREEZE-091`, `OBL-SNOWFREEZE-P-064`, and `TOL-SNOWFREEZE-016`. The behavior-neutral architecture retains one mass/state authority, exact linked handoff, and both independent identities while prohibiting eager disabled-path payload carriage, schema/output drift, API evasion, physics mutation, or performance regression beyond the prospectively frozen package bounds. |
| `2026-08-03` | `123` | `Codex` | SNOW-STAGE3-LIQUID-SIGNED-HOUR-TRACE-CLOSURE amendment: added `REF-SNOWFREEZE-STAGE3-TRACE-CLOSURE`, exact Stage-3 and signed-hour diagnostic records, `INV-SNOWFREEZE-090`, `OBL-SNOWFREEZE-P-063`, and `TOL-SNOWFREEZE-015`. The additive internal JSONL v4 surface publishes exact liquid operands and existing forcing/state/cold-content context for independent real-file reconstruction while prohibiting physics, selector, default, state, protected-output, fixture, observation, calibration, or promotion changes. |
| `2026-08-01` | `121` | `Codex` | SNOW-SURFACE-EB-04W amendment: added behavior-neutral hourly phase/accumulation and exact four-term CoE melt-depth diagnostics, `INV-SNOWFREEZE-088`, `OBL-SNOWFREEZE-P-062`, and `TOL-SNOWFREEZE-013`. The amendment distinguishes snowfall depth from SWE, separates uncapped formula contributions from the pack cap, binds modeled drift as inactive without claiming physical redistribution is zero, and prohibits pure-energy relabeling, fitting, forcing compensation, physics changes, or promotion. |
| `2026-08-02` | `122` | `Codex` | SNOW-SURFACE-EB-04W2B amendment: added `INV-SNOWFREEZE-089` and `TOL-SNOWFREEZE-014`, requiring production typed-phase availability for material precipitation, making positive typed hourly snowfall sufficient for shared-consumer activation, and requiring fail-closed daily SWE reconstruction from the independent typed snowfall-depth operand. Phase-provider equations, selectors, and defaults remain unchanged. |
| `2026-08-01` | `120` | `Codex` | SNOW-SURFACE-EB-04V amendment: added the typed density-process diagnostic ledger, `INV-SNOWFREEZE-087`, `OBL-SNOWFREEZE-P-061`, and `TOL-SNOWFREEZE-012`. The amendment requires direct fresh-snow density, separate process/structural/cap increments, real-consumer publication, anti-alias tests, and additive closure without changing density physics or authorizing calibration/promotion. |
| `2026-07-30` | `119` | `Codex` | SNOW-SURFACE-EB-03A amendment: added `REF-SNOWFREEZE-EB03A`, `INV-SNOWFREEZE-086`, `OBL-SNOWFREEZE-P-060`, and the active-layer thermal coupling addendum. The amendment replaces the failed snowfall-event thermal provider with the Marks/SNOBAL upper-`0.25 m` active volume, equal-and-opposite `G_0`, conservative projection, and fixed mass-dependent substeps while retaining CoE existence/melt authority and prohibiting clamps or fitted limiters. |
| `2026-07-30` | `118` | `Codex` | SNOW-SURFACE-EB-03 amendment: added `REF-SNOWFREEZE-EB03`, `INV-SNOWFREEZE-085`, `OBL-SNOWFREEZE-P-059`, and the shared thermal/energy composition addendum. The amendment selects the Stage 3 top layer as the sole snow thermal provider for orthogonal default-off longwave/sublimation cells, binds exact-one signed vapor/latent composition and snow-state mutation, preserves CoE melt and all defaults/public boundaries, and requires direct-consumer plus independent mass/energy closure evidence. |
| `2026-07-13` | `117` | `Codex` | INTVAL-FROST-THAW-CLEAR-001 amendment: added pinned `frostn`/`frwatc(0)` egress authority and tightened `INV-SNOWFREEZE-012` so material thaw-complete outcomes retain their original coarse layer basis until the authoritative layer projection and post-`frwatc` scalar apply together exactly once. Pre-ingress synthetic zero-frost clearing against the pre-exchange scalar is prohibited; nonmaterial stale clearing remains permitted. Added the corresponding contract-derived test obligation and runtime gate posture. |
| `2026-06-29` | `115` | `Codex` | FROST DIRECT CUTOVER CORRECTION amendment: superseded the v114 compatibility-fallback carve-out for multi-OFE/Wave-2 and legacy sidecar-discovery no-env runs. `DefaultCandidate` must select direct production for current hillslope runs; direct failures are blockers to fix or hold, not accepted compatibility fallbacks. Explicit compatibility selection, if still present during staged deletion, is only a deprecated comparator/deletion seam and cannot satisfy default-activation gates. |
| `2026-07-02` | `116` | `Claude Code` | MOFEFID-B03 reconciliation: `INV-SNOWFREEZE-015` net-melt-scaling text now explicitly defers to `INV-SNOWFREEZE-019` (SNOWSCI-S1 single-source store) for the positive+negative hourly-melt COEXIST case — routed melt = authoritative storage loss (positive-parts-consistent), not legacy net-scaling; grounded in the `INV-SNOWFREEZE-002` density gate (sub-350 refreeze is in-pack, not netted against exported melt). Resolves B01 finding B10: not a defect and not a live contract conflict — INV-019 already superseded INV-015 for this branch; the text lacked the cross-reference. No production change. |
| `2026-06-29` | `114` | `Codex` | FROST RATIFICATION + DEFAULT ACTIVATION amendment: ratified `INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, and `INV-SNOWFREEZE-050` as accepted observation-validation authority, accepted `TOL-SNOWFREEZE-007..011` as noise-floor evaluation bands, added `REF-SNOWFREEZE-FROST-RATIFICATION-20260629` and `INV-SNOWFREEZE-084`, documented the H1b `frdp` bottom-extent / `thdp` top-thaw correspondence, adjudicated the Step 1 `>0.25` systematic-timing-fraction cutoff as diagnostic-local, and re-dispositioned `GAP-SNOWFREEZE-002` as open-but-attributed/bounded under the frost Policy B analog. This entry's compatibility-fallback carve-out was superseded by v115. |
| `2026-06-29` | `113` | `Codex` | FROST RESIDUE-COVER IMPLEMENTATION amendment: added `REF-SNOWFREEZE-FROST-RESIDUE-COVER-20260629`, `INV-SNOWFREEZE-083`, and `OBL-SNOWFREEZE-P-058`, binding frost thermal inputs to consume dynamic residue depth from `SC-RESIDUE-001#INV-RESIDUE-019` rather than reusing the t0 seed after surface-residue mass changes. The amendment preserves existing frost heat-flow equations and explicitly excludes snow-model, canopy leaf-on/off, Qwet, public-schema, fixture-fitting, and frost-default changes. |
| `2026-06-29` | `112` | `Codex` | Paradigm 2 multilayer promotion amendment: added `REF-SNOWFREEZE-PARADIGM2-PROMOTION`, `INV-SNOWFREEZE-082`, `OBL-SNOWFREEZE-P-057`, and the production-supported opt-in addendum. The amendment ratifies `layered_thermal_liquid_v1` as a default-off supported internal production water-temperature capability, publishes nullable hillslope WAT parquet `MeltwaterTemperature` in `degC`, keeps the bulk snow default and rollback unchanged, defers parser/runfile/user and HBP/watershed serialization to later stream-temperature work, and requires exact `15` / `179` snow-rubric equivalence, conservation, bounded meltwater temperature, release notes, and ADR-0025 performance evidence. |
| `2026-06-29` | `111` | `Codex` | Paradigm 2 Stage 3-Decouple snow-neutral water-temperature amendment: added `REF-SNOWFREEZE-PARADIGM2-STAGE3-DECOUPLE`, `INV-SNOWFREEZE-081`, `OBL-SNOWFREEZE-P-056`, boundary disposition, and the Stage 3-Decouple addendum. The amendment keeps opt-in `layered_thermal_liquid_v1` but allows it to run with the current bulk density default through bulk-equivalent layers, forbids Stage 1 density dependence for the water-temperature arm, requires exact current-default snow-rubric equivalence (`15` / `179`), preserves CoE melt/rain mass, frost behavior, public schemas, default, fixtures, and full stream-temperature routing boundary, and requires mass/liquid/energy closure plus ADR-0025 H2637 performance evidence. |
| `2026-06-29` | `110` | `Codex` | Paradigm 2 Stage 3 per-layer liquid-routing and meltwater-temperature amendment: added `REF-SNOWFREEZE-PARADIGM2-STAGE3`, Stage 3 diagnostic variables, `INV-SNOWFREEZE-080`, `OBL-SNOWFREEZE-P-055`, boundary disposition, invalid-state guards, and the Stage 3 addendum. The amendment reserves opt-in `layered_thermal_liquid_v1`, preserves CoE melt/rain mass as authoritative, requires typed meltwater-flux temperature plus mass/liquid/energy closure, keeps the disabled path as rollback, and forbids default activation, public schema changes, fixture fitting, frost changes, and full stream-temperature routing. |
| `2026-06-28` | `109` | `Codex` | Paradigm 2 Stage 2 snow-frost insulation-profile amendment: added `REF-SNOWFREEZE-PARADIGM2-STAGE2`, `snow_frost_insulation_model`, `snow_layer_insulation_resistance`, `snow_frost_effective_density`, `INV-SNOWFREEZE-079`, `OBL-SNOWFREEZE-P-054`, boundary disposition, and the Stage 2 addendum. The amendment reserves opt-in `layered_resistance_v1`, requires a real Stage 1 density-gradient entry gate, uses the Sturm et al. 1997 snow conductivity relation already mirrored by the WEPP frost heat path, preserves prior-day snow-to-frost timing and bulk rollback, makes frost observation signatures primary, and forbids default activation, schema/fixture/cap changes, and Stage 3 thermal/melt/liquid scope. |
| `2026-06-28` | `108` | `Codex` | Paradigm 2 Stage 1 layered snow-density amendment: added `REF-SNOWFREEZE-PARADIGM2-STAGE1`, `snow_layers`, `snow_layer_local_overburden`, `INV-SNOWFREEZE-078`, `OBL-SNOWFREEZE-P-053`, boundary disposition, and the Stage 1 addendum. The amendment reserves opt-in `physics_bulk_multilayer_density_v1`, keeps the current no-env default and `legacy_wepp` rollback, authorizes only persistent winter-column layer state plus per-layer Anderson/SNOBAL compaction under local overburden, preserves aggregate public outputs, active `522 kg m^-3` cap, melt/liquid/phase/frost/canopy/radiation boundaries, and requires real cross-SNOTEL+cancov rubric, bidirectional densification, persistence guardrail, conservation, consumer-persistence, and ADR-0025 performance evidence before any promotion. |
| `2026-06-28` | `107` | `Codex` | SNOWDENSITY-10.3.22 rerun authority amendment: verified Sturm 1995 thresholds from the scanned source (`Tc=10 degC`, `CDM=30/125 degC-month`, `SPR=2 mm d^-1`, and wind low/high evidence bracketed by `0.5-2.0 m s^-1`), added the Sturm/Liston 2021 cross-check with its `61 degC-month`, `4 mm d^-1`, Boreal Forest/Taiga, and Montane Forest/Alpine differences, and tightened `INV-SNOWFREEZE-077`/`OBL-SNOWFREEZE-P-052` so direct-runtime actual-wind class assignment fails closed in the unresolved interval, rare branches fail closed, and ephemeral uses the documented fresh-snow/Anderson fallback rather than fabricated Sturm 2010 parameters. |
| `2026-06-28` | `106` | `Codex` | SNOWDENSITY-10.3.22 climate-class density specialization amendment: added `REF-SNOWFREEZE-SNOWDENSITY1022`, `REF-SNOWFREEZE-STURM2010-DENSITY`, `REF-SNOWFREEZE-STURM1995-CLASSIFICATION`, `REF-SNOWFREEZE-NSIDC0768`, climate-class density variables, `INV-SNOWFREEZE-077`, `OBL-SNOWFREEZE-P-052`, boundary disposition, and the Climate-Class Density Specialization Addendum. The amendment reserves opt-in `physics_bulk_climate_class_density_v1`, requires full Sturm class coverage and forcing-derived class assignment, records that local Sturm 2010 authority covers five classes but not ephemeral, preserves current defaults and rollback, and requires HOLD/non-promotion when class thresholds, parameters, cross-SNOTEL rubric evidence, bidirectional densification flip, persistence guardrail, or conservation are missing. |
| `2026-06-28` | `105` | `Codex` | SNOWDENSITY-10.3.20 sublimation diagnosis and Stage B unlock amendment: added `REF-SNOWFREEZE-SNOWDENSITY1020`, `REF-SNOWFREEZE-LIBSNOBAL-CC0`, Stage B surface-layer sublimation variables, `INV-SNOWFREEZE-076`, `OBL-SNOWFREEZE-P-051`, boundary disposition, and the Stage B Unlock Addendum. The amendment preserves the Harder-Pomeroy activated default and rollback selectors, authorizes only opt-in `coe_open_sublimation_stage_b_v1`, requires Stage A degradation diagnosis and partition+sublimation scoring on the cross-SNOTEL forcing-robust rubric, binds libsnobal source use to CC0 provenance, and permits promotion only if the candidate beats the current default while conserving vapor and phase mass. |
| `2026-06-28` | `104` | `Codex` | SNOWDENSITY-10.3.19 Harder-Pomeroy phase default amendment: added `REF-SNOWFREEZE-SNOWDENSITY1019`, `INV-SNOWFREEZE-075`, `OBL-SNOWFREEZE-P-050`, boundary disposition, and the Harder-Pomeroy Phase Default Addendum. The amendment activates `harder_pomeroy_hourly` as the direct-production no-env phase default composed with the activated melt+density bundle, keeps explicit `legacy_rst` rollback/test selection, makes the cross-SNOTEL forcing-robust rubric the primary Policy-B gate (`15/179` vs prior bundle `17/172`), requires workspace no-regression and active-hour partition mass closure, carries forward the humid-New-England roadmap and `+23.6 kg m^-3` density-bias notes, and does not authorize `.run` disable controls or fixture/schema/cap/frost changes. |
| `2026-06-27` | `103` | `Codex` | SNOWDENSITY-10.3.17 shallow-pack compaction guard amendment: added `REF-SNOWFREEZE-SNOWDENSITY1017`, `REF-SNOWFREEZE-MARKS-SHALLOW-LAYER`, `snow_shallow_compaction_guard_depth_threshold`, `physics_bulk_shallow_guard_v1`, `INV-SNOWFREEZE-074`, `OBL-SNOWFREEZE-P-049`, boundary disposition, and the Shallow-Pack Compaction Guard Addendum. The amendment authorizes only an opt-in diagnostic reduction of density-compaction aggressiveness below the authority-derived `0.25 m` shallow-pack threshold, preserves activated defaults, rollback, SWE/liquid/melt/routed outputs, active `522 kg m^-3` cap, output schemas, fixtures, and frost posture, and requires coupled WAT/trace gates for induced under-persistence recovery, over-persistence non-worsening, threshold authority, and conservation; it does not authorize activation. |
| `2026-06-27` | `102` | `Codex` | SNOWDENSITY-10.3.16 open-surface ablation Stage A amendment: added Marks/SNOBAL turbulent latent mass-loss authority, `snow_sublimation`, `coe_open_sublimation_stage_a_v1`, `INV-SNOWFREEZE-073`, `OBL-SNOWFREEZE-P-048`, boundary disposition, and the Stage A addendum. The amendment authorizes only an opt-in diagnostic vapor mass-loss sink separate from routed liquid, preserves the activated default and rollback selectors, forbids fixture-tuned constants and PySnobal C implementation authority without permissive-license confirmation, and requires coupled WAT/trace gates for open-surface tail reduction, under-persistence non-worsening, magnitude range, and snow-state conservation; it does not authorize activation. |
| `2026-06-27` | `101` | `Codex` | SNOWDENSITY-10.3.15 default activation amendment: added `REF-SNOWFREEZE-SNOWDENSITY1015`, `INV-SNOWFREEZE-072`, `OBL-SNOWFREEZE-P-047`, boundary disposition, and the Default Activation Under Active Cap Addendum. The amendment activates `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1` as the direct-production no-env default under the active `522 kg m^-3` cap, retains `legacy_coe`/`legacy_wepp` explicit rollback/test selectors, rejects unsupported candidates in the active default selector path, and keeps `498/1415` paired snow-depth residual failures as a frost-attribution blocker. |
| `2026-06-27` | `100` | `Codex` | SNOWDENSITY-10.3.14 Policy-B no-regression and cap-authority amendment: added `INV-SNOWFREEZE-071`, `OBL-SNOWFREEZE-P-046`, boundary disposition, and the Policy-B No-Regression And Cap Authority Addendum. The amendment requires a workspace-suite no-regression gate under the current opt-in bundle selectors, composite trace closure, composite snow-state conservation closure, and active `522 kg m^-3` cap bounds before any separate default-activation package; downstream snow-affected output deltas are conserved-by-construction and not separately diffed, and `550 kg m^-3` remains projection-only until a later dynamic cap amendment and rerun. |
| `2026-06-27` | `99` | `Codex` | SNOWDENSITY-10.3.13 residual-tail and Policy-B diagnostic amendment: added `INV-SNOWFREEZE-070`, `OBL-SNOWFREEZE-P-045`, boundary disposition, and the Residual-Tail And Policy-B Diagnostic Addendum. The amendment authorizes date-level residual transition attribution and a Policy-B workspace-suite/conservation evidence matrix while explicitly forbidding production physics, default activation, density-cap changes, and frost attribution. |
| `2026-06-27` | `98` | `Codex` | Post-review SNOWDENSITY-10.3.12 activation-policy amendment: incorporated the operator-ratified Claude review by superseding zero paired snow-depth failures as the default-activation criterion for `INV-SNOWFREEZE-069`. Activation Policy B now requires strict improvement over the current default on gate-eligible paired-snow surfaces plus workspace-suite no-regression/conservation evidence; remaining snow-control residuals block frost attribution separately. |
| `2026-06-27` | `97` | `Codex` | SNOWDENSITY-10.3.12 combined bundle activation adjudication amendment: added `INV-SNOWFREEZE-069`, `OBL-SNOWFREEZE-P-044`, boundary disposition, and the Combined Bundle Activation Adjudication Addendum. The amendment records that `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1` is a named opt-in bundle requiring real direct-production WAT trace proof, comparator-ladder evidence, and residual classification. Its initial zero-paired-failure activation criterion is superseded by v98. |
| `2026-06-27` | `96` | `Codex` | SNOWDENSITY-10.3.11 spring compaction/densification amendment: added `physics_bulk_spring_densification_v1`, `INV-SNOWFREEZE-068`, `OBL-SNOWFREEZE-P-043`, and the Opt-In Spring Compaction/Densification Addendum. The amendment authorizes only an explicit opt-in wet-compaction realization candidate with default `legacy_wepp` identity, fixed CoE melt/liquid boundary, no density-cap change, no observed-depth fitting, SWE identity, and coupled WAT comparison against the 10.3.8 holding-capacity plus `physics_bulk_density_compaction_v1` baseline. |
| `2026-06-27` | `95` | `Codex` | SNOWDENSITY-10.3.8 liquid holding-capacity amendment: added in-repo retained-liquid authority anchors, `coe_liquid_holding_capacity_v1`, `INV-SNOWFREEZE-067`, `OBL-SNOWFREEZE-P-042`, and the Opt-In Liquid Holding-Capacity Addendum. The amendment authorizes only an explicit opt-in capacity-bound drainage candidate with persistent retained-liquid state, default `legacy_coe` identity, no output-schema/parser/runfile/user activation, and closure gates requiring event-window improvement, conservation/routing proof, and coupled direct-production WAT evidence. |
| `2026-06-27` | `94` | `Codex` | SNOWDENSITY-10.3.7 review disposition amendment: made conservation proof and coupled direct-production WAT snow-control rerun current-scope gates for `coe_winter_thaw_state_loss_v1`, authorized only the package-bound diagnostic selector `OPENWEPP_SNOWDENSITY1037_MELT_MODEL`, and clarified that event-window improvement without retained/released-rain closure and WAT coupling evidence is an opt-in signal, not a fix or activation authority. |
| `2026-06-27` | `93` | `Codex` | SNOWDENSITY-10.3.7 opt-in winter-thaw melt-response amendment: added `coe_winter_thaw_state_loss_v1`, qualified the legacy density gate through `INV-SNOWFREEZE-066`, added `OBL-SNOWFREEZE-P-041`, and added the Opt-In Winter-Thaw State-Loss Addendum. The amendment authorizes only an explicit opt-in positive-thaw state-loss candidate that preserves CoE melt terms, default `legacy_coe`, `coe_shortwave_albedo_v1` behavior, and all forcing/canopy/phase/density/frost/output-schema boundaries; closure requires independent operand reconstruction and paired Sleepers/Harvard event-window improvement evidence without site constants. |
| `2026-06-27` | `92` | `Codex` | SNOWDENSITY-10.3.5b opt-in hourly partition amendment: added `snow_phase_partition_model`, exact-saturation humidity normalization, `INV-SNOWFREEZE-065`, `OBL-SNOWFREEZE-P-040`, and the Opt-In Hourly Partition And Jennings Validation Addendum. The amendment authorizes only a package-bound `legacy_rst`/`harder_pomeroy_hourly` selector at the direct-runtime hourly winter partition seam, preserves default `legacy_rst`, requires precipitation reconstruction and direct-consumer evidence, and requires Jennings observed-phase validation without tuning or parser/runfile/user activation. |
| `2026-06-27` | `91` | `Codex` | SNOWDENSITY-10.3.5a meteorology-crate amendment: added `REF-SNOWFREEZE-HARDER-POMEROY-2013`, hydrometeor-temperature/psychrometric candidate variables, `INV-SNOWFREEZE-064`, `OBL-SNOWFREEZE-P-039`, candidate API aliases, invalid-state isolation guards, and the Harder-Pomeroy Meteorology Crate Addendum. The amendment authorizes only a pure `openwepp-meteorology` crate and explicitly forbids production `RST`/`stmtim` replacement, parser/runfile/user selectors, output-schema changes, default activation, fixture edits, and compatibility-runtime changes. |
| `2026-06-26` | `88` | `Codex` | SNOWDENSITY-09 diagnostic coupled WAT amendment: added `INV-SNOWFREEZE-062`, `OBL-SNOWFREEZE-P-037`, and the 09 addendum authorizing a package-bound diagnostic environment selector for direct-production non-SNOTEL WAT reruns while preserving `legacy_wepp` default behavior and forbidding parser/runfile/user CLI activation, WAT rewriting, tuning, or frost attribution unless the coupled opt-in snow-control gate passes. |
| `2026-06-26` | `89` | `Codex` | SNOWDENSITY-09 gate correction: clarified that the coupled opt-in snow-control gate is evaluated only over fixtures with observed snow-depth rows, while SCAN Mandan ND, Reynolds Creek ID, and other no-observed-snow fixtures remain reported as diagnostic-only out-of-gate evidence rather than pass/fail/blocker inputs. |
| `2026-06-26` | `90` | `Codex` | SNOWDENSITY-10.3.1a per-day canopy bridge amendment: added `cancov_daily_series`, `INV-SNOWFREEZE-063`, `OBL-SNOWFREEZE-P-038`, and the 10.3.1a addendum requiring snowbench/CoE melt replay to consume direct-production per-day growth-state canopy rather than a repeated scalar runtime-surface value before low-canopy or seasonal-canopy melt adjudication. |
| `2026-06-26` | `87` | `Codex` | SNOWDENSITY-08 gate-rerun amendment: added `INV-SNOWFREEZE-061`, `OBL-SNOWFREEZE-P-036`, and the 08 addendum requiring SNOTEL density-rubric evidence and non-SNOTEL frost-site snow-control evidence to be reported separately; frost attribution cannot resume unless an authorized coupled opt-in WAT/publication run applies `physics_bulk_density_compaction_v1` to the snow-depth state consumed by frost and WAT `Snow-Depth`. |
| `2026-06-26` | `86` | `Codex` | SNOWDENSITY-07 runtime opt-in amendment: added `INV-SNOWFREEZE-060`, `OBL-SNOWFREEZE-P-035`, `snow_density_model`, CoE boundary carry surfaces, and the 07 addendum authorizing typed opt-in `physics_bulk_density_compaction_v1` only when CoE SWE/liquid boundaries remain authoritative and depth/density mutation is projected through direct runtime state, downstream operands, shadow, carry, and publication-facing snow state. |
| `2026-06-26` | `85` | `Codex` | SNOWDENSITY-06B CoE-bound density replay amendment: added `INV-SNOWFREEZE-059`, `OBL-SNOWFREEZE-P-034`, and the 06B addendum authorizing offline replay of `density_compaction_v1` against fixed CoE SWE/liquid boundaries with daily SWE identity and density-cell/whole-rubric adjudication before runtime opt-in. |
| `2026-06-26` | `84` | `Codex` | SNOWDENSITY-06 density-compaction amendment: added `INV-SNOWFREEZE-058`, `OBL-SNOWFREEZE-P-033`, and the 06 addendum authorizing an offline `density_compaction_v1` candidate with fixed melt/radiation/albedo/canopy boundaries, named Anderson/SNOBAL PTM/POC and liquid-water compaction constants, and density/densification robust-cell evidence before any promotion claim. |
| `2026-06-26` | `83` | `Codex` | SNOWDENSITY-05G harness-fidelity amendment: added `INV-SNOWFREEZE-057`, `OBL-SNOWFREEZE-P-032`, and the 05G addendum requiring diagnostic CoE melt replay to use configured coniferous canopy rather than `cancov = 0.0`, publish native/proven shortwave lineage or PySnobal bridge inversion identity, and rerun SNOTEL rubric evidence without default activation or density-physics changes. The representative rerun closed `NON-PROMOTION` for default activation (`robust_fail_count 9 -> 9`, `robust_ordinal_score 84 -> 86`). |
| `2026-06-26` | `82` | `Codex` | Operator clarification for SNOWDENSITY-05F: the validation forest management should be configured as coniferous forest with winter `cancov` about `0.9`; therefore the 05E `cancov = 0.0` diagnostic replay is known non-representative, not merely an unverified live-canopy caveat. Updated the SNOWDENSITY-06 harness-fidelity gate accordingly. |
| `2026-06-26` | `81` | `Codex` | SNOWDENSITY-05F independent-review disposition: labeled 05E diagnostic replay evidence as regime-limited because it used `cancov = 0.0` and PySnobal-bridge radiation, added the SNOWDENSITY-06 harness-fidelity entry gate for live per-day canopy and native/proven shortwave, and recorded local Brock-2000 constant verification. |
| `2026-06-26` | `80` | `Codex` | SNOWDENSITY-05F melt closure / density handoff amendment: added `INV-SNOWFREEZE-056`, `OBL-SNOWFREEZE-P-031`, boundary disposition, and the 05F addendum. The accepted melt boundary remains opt-in only: `legacy_coe` stays default/rollback, `coe_shortwave_albedo_v1` may be consumed by density work only as a fixed typed interface with no melt/radiation retuning, same-day future snowfall must preserve opt-in albedo continuity or fail closed, and any default-candidate claim must report both 05E diagnostic replay and H as-built context. |
| `2026-06-26` | `79` | `Codex` | SNOWDENSITY-05D opt-in CoE melt implementation amendment: added `INV-SNOWFREEZE-055`, `OBL-SNOWFREEZE-P-030`, `snow_melt_shortwave_absorbed_fraction`, and the 05D addendum. The only authorized opt-in production delta is `amelt = 0.0607 * hrrad * (1 - snow_albedo) * (1 - cancov)` using the 05B radiation source and 05C albedo state; `legacy_coe` remains default/rollback, missing active opt-in state is typed fail-closed, and acceptance requires raw/routed melt, SWE loss, WB12 `S`, and WB13 liquid-forcing reconstruction. |
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
