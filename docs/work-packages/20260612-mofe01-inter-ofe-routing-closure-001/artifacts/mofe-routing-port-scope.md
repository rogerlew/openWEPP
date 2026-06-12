# mofe routing port scope

Status: complete for increment M-A scoping

Evidence mode: Static + Ran

## Evidence

Static:
- Pinned legacy baseline: `/workdir/wepp-forest_260430_baseline` at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Baseline working tree was clean when inspected.
- openWEPP HEAD: `f4c162e45d853805b127eb08d269f7c3b7215d0a`.

Ran:
- Local current batch on isolated `/tmp/openwepp_mofe01_ma` lane.
- Local legacy WAT parse for H1-H36.
- No comparator subagent was used.

## Current openWEPP port seams

Current code has partial MOFE support but does not yet close the inter-OFE route:

- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:30-45`
  - Detects `contributor_ofe_count > 1`, enables MOFE hourly carry arrays, and forces 24 substeps.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:461-472`
  - Seeds `hydrology.wb12_runon_input` and `hydrology.flux.wb12_runoff_carryover` as zero at hillslope initialization.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:1139-1145`
  - Erosion MOFE qin reads `UpStrmQ` with a default of zero.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs:469-557`
  - `resolve_mofe_hourly_upstream_carryover` can sum upstream saturation and lateral arrays, apply area ratio, and validate against carryover.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs:627-660`
  - `resolve_runoff_carryover_input` prefers MOFE upstream carry, then `wb12_runoff_carryover`, then `wb12_runon_input`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs:268-326`
  - `runon_input` is included in runoff partition and closure.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs:92-121`
  - MOFE hourly carry arrays force 24 substeps.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs:232-240` and `:713-725`
  - Current lateral and saturation substep arrays are stored as `ui_LfCrf` and `ui_SCrunf`.
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:956-995`
  - Current WAT publication reads `SubRIn`, hard-codes `UpStrmQ = 0.0`, and sets `QOFE = Q`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs:214-282`
  - Manifest records MOFE carry metadata and publication policy.

Observed consequence:
- 1-OFE surfaces pass.
- Every 2-5 OFE H surface fails before publication.
- There are no completed current multi-OFE WAT rows to compare yet.

## Legacy routing authority map

The pinned legacy route is a coupled hydrology/erosion/publication state machine:

- `watbal.for:138-153`
  - WATBAL reads and mutates runoff/runon state, including `roffon` and `rvolon`.
- `watbal.for:198-202`
  - Local WATBAL state includes `latqcc`, `runoffin(mxplan)`, `subrin`, and `sbrin`.
- `watbal.for:273-276` and `watbal.for:329`
  - Initializes `latqcc`, `subq`, `sbrunf(iplane)`, `sbrin`, and `subrin`.
- `watbal.for:343-370`
  - Builds `fin`; for ordinary hillslope OFE `iplane != 1`, adds upstream subsurface runon and upstream/downstream surface runoff difference, sets `runoffin(iplane)`, and sets `subrin`.
  - OFE1 subtracts its own runoff and sets `runoffin=0`.
- `watbal.for:376-383`
  - Re-derives negative `fin` from upstream/downstream runoff differences.
- `watbal.for:700-707` and `watbal.for:732-785`
  - Computes lateral flow and stores `sbrunf(iplane)`.
- `watbal.for:797-841`
  - `surdra` excess can be added to `runoff(iplane)` after the drainage balance.
- `watbal.for:1074-1105`
  - WAT publication writes `Q`, `UpStrmQ`, `SubRIn`, `latqcc`, and `QOFE`.
- `irs.for:244-249`
  - IRS loops OFEs.
- `irs.for:335-346`
  - Equivalent plane length accumulates when upstream runoff continues.
- `irs.for:355-362`
  - Documents cases 1-4: no runoff, rainfall excess, upstream runoff continues, upstream runoff infiltrates out.
- `irs.for:370-446`
  - Handles rainfall-excess and no-runoff cases and sets/clears downstream runon flags.
- `irs.for:458-526`
  - `rochek` decides whether upstream runoff continues to the downstream OFE.
- `irs.for:541-558`
  - Replaces `runoff(1..iplane)` with `runtmp`, calls WATBAL, and updates `runtmp` because WATBAL may mutate runoff.
- `irs.for:566-620`
  - If WATBAL creates `surdra`, injects surplus into event shape and may set downstream runon.
- `irs.for:639-756`
  - Computes peak/runoff hydrograph and duration, including OFE scaling.
- `rochek.for:1-7`
  - Checks runoff from a plane with runon.
- `rochek.for:92-126`
  - Uses upstream runoff and current rainfall excess to decide case 3 versus case 4 and update downstream runoff.
- `xinflo.for:130-151`
  - Sets erosion `qin` from previous OFE `qout`, then computes current `qout`.
- `xinflo.for:173-186`
  - Uses `qout` for infiltration/transport arrays and shear.
- `route.for:53-57` and `route.for:139-154`
  - Sediment routing receives `qin`/`qout`; incoming particle fractions copy from the previous OFE when flow enters.
- `contin.for:723-731`, `:955-964`, `:999-1019`, `:1066-1068`, `:1164-1218`
  - Daily reset, IRS call, erosion qin setup, WATBAL fallback call, and route call sequence.

## Port scope

### M-B hydrology route closure

Primary objective:
- Make H1-H36 multi-OFE hydrology execute without `runoff_reconciliation` domain rejection.
- Populate upstream surface/subsurface carry before the downstream OFE WATBAL-equivalent phase.

Required behavior:
- OFE1 has `UpStrmQ = 0` and `SubRIn = 0`.
- OFE `i > 1` receives previous OFE surface handoff and lateral handoff.
- `runon_input` in `hydrology_phase_runoff_reconciliation.rs:268-326` must come from populated upstream carry, not a zero seed.
- Lateral carry arrays written by `hydrology_phase_lateral_drainage.rs:232-240` and `:713-725` must feed the next OFE's `SubRIn` path.

Red tests to add before implementation:
- A minimal 2-OFE runtime-surface test where upstream `QOFE`/carry is nonzero and downstream `runon_input` is nonzero.
- A minimal 2-OFE lateral handoff test where upstream `latqcc` becomes downstream `SubRIn`.
- A cohort smoke test requiring H11, H6, H9, and H1 to execute past day 2. These cover 2, 3, 4, and 5 OFE classes.

### M-C WAT publication closure

Primary objective:
- Publish per-OFE WAT semantics required for MOFE closure characterization.

Required behavior:
- Do not publish `UpStrmQ = 0.0` for downstream OFEs.
- Do not alias `QOFE = Q` on multi-OFE slopes.
- Preserve legacy-adjacent handoff to output precision:
  - current OFE `UpStrmQ` equals previous OFE `QOFE`.
  - current OFE `SubRIn` equals previous OFE `latqcc`.

Red tests to add before implementation:
- Multi-OFE WAT rows include one row per OFE per day or an explicitly contracted equivalent per-OFE publication surface.
- H1 day 1 publishes five OFE rows if the legacy WAT-compatible lane is selected.
- Downstream `UpStrmQ` and `SubRIn` are nonzero on runoff/lateral-flow days.

### M-D erosion qin and sediment coupling

Primary objective:
- Wire erosion `qin` from water routing once hydrology is stable.

Scope:
- `scheduler_seed_and_runtime.rs:1139-1145` currently defaults erosion MOFE qin to zero.
- `xinflo.for:130-151` and `route.for:139-154` show legacy coupling between upstream `qout`, current `qin`, and sediment fractions.

Do not pull sediment routing into M-B unless hydrology execution requires an explicit qin contract to avoid invalid state.

### M-E watershed/far-point follow-up

Primary objective:
- Decide whether the `pw0` 15-segment slope is a watershed wrapper, a comparable hillslope surface, or only an inventory signal.

M-A finding:
- `pw0.slp` declares 15 segments.
- Legacy output has `pass_pw0.txt`, `plot_pw0.txt`, `loss_pw0.txt`, `soil_pw0.txt`, and `ebe_pw0.txt`, but no `pw0.wat.dat`.
- No 15-OFE WAT closure calibration can be computed from the current on-disk legacy artifacts.

## Non-scope for the next implementation increment

- Do not tune openWEPP to numeric legacy WAT residual maxima.
- Do not mask missing upstream carry with default zeros.
- Do not add fallback wrappers that silently convert MOFE domain violations into single-OFE execution.
- Do not broaden into watershed orchestration until hillslope MOFE water handoff executes and publishes enough per-OFE evidence.

## Acceptance shape

The next implementation increment should first turn the current hard failure into successful multi-OFE execution, then measure WAT publication semantics. Passing legacy comparator deltas are not expected immediately; the first meaningful acceptance is:

1. H1-H36 complete locally without `HKERNEL-WB14-RUNOFF-E-*` rejection.
2. Downstream OFEs carry nonzero `UpStrmQ` and `SubRIn` on legacy-active handoff days.
3. `QOFE` is distinct from `Q` when OFE scaling requires it.
4. Per-OFE publication or a contracted equivalent exposes enough state to compute the M-A diagnostics without inference.
