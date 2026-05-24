# Simimpl01 watbal consolidation and timestep architecture

Status: phase-d-complete
Evidence mode: Static + Ran

## Static
- This artifact records the final SIMIMPL01 consolidation architecture requirements and ordering constraints.
- Authority stack:
  - process contracts (`SC-WATBAL-001`, `SC-SYSTEM-001`, `SC-INFILE-WEPPUI-001`)
  - baseline comparator provenance (`wepp-forest_260430_baseline` pinned)
  - selective architecture intake from `/workdir/wepp-forest/fpm-src`

## Ran
- Source probes:
  - `rg -n "wbk|requested_mode|effective_mode|scheduler_mode|wbk09_hourly_qcap_policy" /workdir/wepp-forest/fpm-src/watbal_process_kernels.f90 /workdir/wepp-forest/fpm-src/watbal_daily_adapter.f90 /workdir/wepp-forest/fpm-src/watbal_hourly_adapter.f90`
- openWEPP probes:
  - `rg -n "build_h5_wat_output|build_hillslope_wat_rows|build_first_day_wat_projection" crates/openwepp-runner/src/lib.rs`
  - `rg -n "HillslopePhaseScheduler|execute_with_kernel|HydrologyEvapotranspiration|HydrologyDrainage|HydrologyStorageReconciliation|HydrologyPeakRunoff" crates/openwepp-hillslope-orchestrator/src/lib.rs`

## Consolidation requirements (final)
1. Shared kernel family with explicit phase ownership
- Maintain a unified process kernel family for daily/hourly lanes.
- Minimum explicit phase classes: ET, percolation/deep seepage, lateral transfer,
  drainage, runoff reconciliation, storage reconciliation, closure diagnostics.

2. Adapter split by timestep context only
- Keep separate daily and hourly adapter entrypoints.
- Adapter-specific behavior must be limited to timestep and mode context
  (`dt`, scheduler mode, requested/effective mode), not duplicate process
  physics.

3. Runtime mode closure from `wepp_ui`
- Parsed `wepp_ui` intent must govern runtime lane selection.
- Requested/effective mode divergence must be typed and surfaced; no silent
  fallback.

4. Extensible timestep policy surface
- Required representable policy values:
  - daily (`24 h`)
  - hourly (`1 h`)
  - reserved sub-hourly (`0.25 h`, `6/60 h`) as scaffolded typed states.
- Sub-hourly physics enablement is explicitly out of SIMIMPL01 scope.

5. Guard posture
- No silent defaults or silent clamping in production execution.
- Candidate policy modules (for example hourly qcap soft-limiter) are not
  admissible until contract + tests + pre-implementation gate authorize them.

6. Publication authority
- H.wat/WB13 surfaces must be generated from scheduler/kernel execution state,
  not first-day projection synthesis.

## Candidate-to-openWEPP mapping
| Candidate pattern | Candidate anchor | openWEPP target | Intake posture |
|---|---|---|---|
| Shared process kernels (`wbk01..wbk09`) | `watbal_process_kernels.f90` | `openwepp-hillslope-orchestrator` hydrology lanes | adopt-by-triage |
| Daily adapter mode context | `watbal_daily_adapter.f90:222-223` | runner/orchestrator daily lane binding | adopt |
| Hourly adapter mode context | `watbal_hourly_adapter.f90:325-326`, `:475-477` | runner/orchestrator hourly lane binding | adopt after `wepp_ui` runtime closure |
| Hourly qcap policy | `watbal_process_kernels.f90:2045-2150` | policy module intake | defer pending contract-first disposition |

## SIMIMPL01 architecture decision
- Consolidation is feasible and should use shared-kernel plus adapter pattern.
- The blocker is production wiring and branch governance closure, not absence of
  kernel abstractions.
- Queue sequencing in `simulation-implementation-wp-queue.md` is aligned to this
  conclusion.
