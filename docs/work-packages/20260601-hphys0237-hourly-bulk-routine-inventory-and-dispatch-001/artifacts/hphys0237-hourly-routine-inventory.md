# HPHYS0237 Hourly Routine Inventory

Status: completed  
Evidence mode: mixed (`Static` + `Ran`)

## Method

Ran:
- `rg`/`sed`/`nl` scans over baseline hourly sources and openWEPP hydrology
  routines.
- Dispatch explorer-agent pass (baseline call-chain + openWEPP ownership
  cross-check) completed and merged into this inventory.

Static:
- Baseline authority:
  - `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
  - `/workdir/wepp-forest_260430_baseline/src/purk.for`
  - `/workdir/wepp-forest_260430_baseline/src/drain.for`
- openWEPP ownership:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `crates/openwepp-hillslope-orchestrator/src/phase.rs`
  - `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs`

## Baseline Authoritative Hourly Call Chain (Execution Order)

1. Initialize layer limits/state (`ul`, `fc`, `hk`, `soilw` init path).
2. Build daily available water input (`fin` + runon/upstream terms).
3. Set `ui_LFtstp=24` and enter hourly loop `ii=1..ui_LFtstp`.
4. Perform hourly infiltration partition + layer mutation (`xfin` routing).
5. Call hourly percolation (`purk`, then `perc` layer physics).
6. Accumulate hourly deep seepage (`deepSeep += sep`).
7. Execute ET only on final hour.
8. Execute post-percolation saturation clipping (upward transfer).
9. Execute hourly tile drainage trigger + `drain` withdrawal/cap logic.
10. Execute hourly lateral-flow block + saturated-zone withdrawal.
11. Execute hourly surface drainage overflow accumulation.
12. Perform end-of-day aggregation (`drainq`, `sep`, runoff addback).

## Bulk Routine Inventory (Must-Update Set)

| Family | Baseline hourly authority | openWEPP routine ownership | Migration state | Update needed |
| --- | --- | --- | --- | --- |
| Hourly control loop | `watbal_hourly.for` hourly loop `do 75 ii=1,ui_LFtstp` | No single coupled hourly WB11 loop; phase graph is daily phases | partial | Add coupled hourly substep execution boundary for remaining WB19/WB14/WB12 hourly-sensitive paths |
| Hourly infiltration-to-layer mutation | `watbal_hourly.for` lines around `xfin` redistribution into `st(i)` | `compute_coupled_infiltration_depth` integrates depth but does not perform equivalent hourly layer-state mutation before all WB19 steps | partial | Add authoritative hourly infiltration/state mutation coupling surface |
| Hourly percolation (`purk`) | `watbal_hourly.for` `call purk` each hour | `run_percolation` now has iterative lane loop (`wb18_perc_lane_substeps`) | migrated | Keep as locked baseline; no rollback |
| Hourly drainage accumulation | `watbal_hourly.for` `call drain(...,1.0)` + `tileDrainage += drainq`; `drain.for` `dhours` scaling/cap/withdrawal | `run_drainage` computes one daily pass (`Qdd`) | not-migrated | Convert WB19 drainage to iterative substep recompute + accumulation |
| Hourly lateral accumulation | `watbal_hourly.for` hourly `latqcc` computation + `sbrunf/ui_lfcrf` accumulation | `run_lateral_transfer` computes one daily pass (`q`) | not-migrated | Convert WB19 lateral to iterative substep recompute + accumulation |
| Hourly drainage/lateral ordering | `watbal_hourly.for` computes drainage section before lateral section inside each hour | `phase.rs`/`scheduler.rs` run `LateralTransfer` then `Drainage` | not-migrated | Reconcile phase ordering authority for promoted hourly lane |
| Surface drainage/runoff carryover | `watbal_hourly.for` hourly `ui_scrunf`/`surdra` accumulation and daily runoff addback | `run_runoff_reconciliation` computes runoff from daily-coupled terms before end-of-day WB19 hourly-like carryover | partial | Reconcile runoff assembly placement and carryover semantics |
| ET/read-order coupling to infiltration | Baseline ET placement is final-hour inside hourly chain after hourly infiltration/percolation updates | `run_evapotranspiration` reads `wb12_infiltration` before `run_runoff_reconciliation` writes daily infiltration | not-migrated | Reconcile scheduler/order so ET and runoff reconciliation observe authoritative same-day infiltration lineage |
| WB14 runoff reconciliation cadence | Baseline runoff response is coupled to hourly carryover accumulation path | `run_runoff_reconciliation` is single daily reconciliation pass | partial | Introduce hourly-aware reconciliation surfaces or authoritative post-hour aggregation equivalent |
| WB12 storage reconciliation cadence | Baseline storage closure reflects hourly-mutated state history before day-close publication | `run_storage_reconciliation` is single daily closure step | partial | Reconcile storage closure to consume authoritative hourly-mutated flux/state lineage |
| Hourly upstream/runon carry arrays | `watbal_hourly.for` `ui_SUrunf/ui_SCrunf/ui_LfUrf/ui_LfCrf` lane arrays | No equivalent hourly array-state family in WB11 hydrology runtime surface | not-migrated | Add explicit hourly runon/lateral carry surfaces where required for MOFE hourly parity |
| Lane-symbol coverage | `ui_LFtstp=24` lane authority applies to all hourly hydrology actions | Runner seeds only `wb18_perc_lane_substeps` | partial | Extend lane authority to WB19/BW14/WB12 coupled hourly actions |
| Lane-mode execution boundary | Baseline has executable hourly lane (`24`) semantics for coupled chain | Runner/orchestrator expose daily/hourly lanes; subhourly lane is scaffold-only, non-executable | partial | Preserve executable hourly lane closure first; explicitly defer non-executable subhourly scaffolds to separate package |

## Dispatch Queue (Bulk)

1. **Dispatch Group A: WB19 iterative substep migration (required first)**
   - `run_lateral_transfer`
   - `run_drainage`
   - shared layer-state recompute/withdrawal helpers
   - `wb19_lateral_drainage_physics_kernel_contract` iterative hourly vectors

2. **Dispatch Group B: Hourly ordering and runoff carryover authority**
   - `phase.rs` + `scheduler.rs` ordering reconciliation
   - `run_runoff_reconciliation`/support helpers for hourly carryover semantics
   - cross-phase contract vectors in `SC-WATBAL-001` + integration tests

3. **Dispatch Group C: MOFE hourly carry arrays and routing continuity**
   - explicit runtime surfaces for hourly upstream/lateral carry terms
   - runner/orchestrator seeding + publication contracts

4. **Dispatch Group D: WB14/WB12 cadence + ordering closure**
   - `run_evapotranspiration`/`run_runoff_reconciliation`/`run_storage_reconciliation`
     ordering and data-lineage reconciliation
   - scheduler dependency updates (`phase.rs`, `scheduler.rs`)
   - contract vectors proving authoritative infiltration/ET/runoff/storage
     observation ordering under hourly lane mode

## Conclusion

The remaining hourly-shape issue is not isolated to one flux. At minimum, WB19
lateral and drainage must migrate together, and scheduler/runoff authority must
be reconciled in the same bulk stream.
