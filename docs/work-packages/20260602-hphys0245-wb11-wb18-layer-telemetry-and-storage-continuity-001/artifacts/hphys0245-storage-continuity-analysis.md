# HPHYS0245 Storage Continuity Analysis

Status: completed
Evidence mode: Static + Ran

## Summary
- The first large storage discontinuity occurs at the WB18
  `percolation_deep_seepage` boundary.
- WB18 writes `wb11_soil_water` from `Σtheta_after`, which drops the initial
  `wb11_soil_water - Σtheta` storage gap in addition to published `D`/`Pe`.
- WB19 lateral transfer then removes a second large day-1 amount, strongest for
  `H39`.
- WB13 publication reflects the post-scheduler aggregate storage; it is not the
  first causal boundary.
- Storage reconciliation does not introduce the observed residual.

## Day-1 Storage Evidence
| Hillslope | Seed WB11 mm | Seed theta mm | Seed gap mm | Post-WB18 WB11 mm | WB18 D mm | WB18 Pe mm | Seed-to-WB18 delta mm | WB19 lateral delta mm | WB13 Total-Soil mm | Baseline Total-Soil mm | Day-1 delta mm |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | 323.346740 | 293.945130 | 29.401610 | 249.700731 | 44.244399 | 44.244399 | -73.646009 | -19.728001 | 229.574375 | 343.070000 | -113.495625 |
| H7 | 271.241491 | 241.451354 | 29.790137 | 207.598744 | 33.852610 | 33.852610 | -63.642747 | -37.050899 | 170.149490 | 287.680000 | -117.530510 |
| H39 | 363.554235 | 323.143334 | 40.410901 | 300.162992 | 22.980342 | 22.980342 | -63.391243 | -79.515092 | 220.249544 | 386.590000 | -166.340456 |

## Interpretation
- `H1`: `44.244399 mm` WB18 `D` plus the `29.401610 mm` seed gap accounts for
  the `73.646009 mm` seed-to-WB18 aggregate drop.
- `H7`: `33.852610 mm` WB18 `D` plus the `29.790137 mm` seed gap accounts for
  the `63.642747 mm` seed-to-WB18 aggregate drop.
- `H39`: `22.980342 mm` WB18 `D` plus the `40.410901 mm` seed gap accounts for
  the `63.391243 mm` seed-to-WB18 aggregate drop.
- After WB18, `wb11_soil_water` is effectively aligned to `Σtheta`, so the
  aggregate state has lost the non-theta seed storage component.
- WB19 lateral transfer is still material on day 1, especially `H39`, but it is
  downstream of the first aggregate storage discontinuity.

## Static Source Evidence
- WB18 aggregate writeback sets `soil_water_after` from `theta.iter().sum()` at
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1186`.
- WB18 writes that value to `WB11_SYMBOL_SOIL_WATER` at
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1208`.
- WB18 writes per-layer theta at
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1214`.
- WB18 writes `D` and `Pe` from `percolation_loss` at
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1232`
  and
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1238`.
- WB19 lateral subtracts `q_lateral` from aggregate storage at
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1632`.
- WB19 drainage subtracts `q_drainage` from aggregate storage at
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:2063`.

## Generated Analysis Files
- `/tmp/hphys0245_20260602T051933Z/reports/hphys0245_trace_all_rows.tsv`
- `/tmp/hphys0245_20260602T051933Z/reports/hphys0245_trace_key_boundaries.tsv`
- `/tmp/hphys0245_20260602T051933Z/reports/hphys0245_storage_balance_first30.tsv`
- `/tmp/hphys0245_20260602T051933Z/reports/hphys0245_storage_balance_summary.tsv`
- `/tmp/hphys0245_20260602T051933Z/reports/hphys0245_storage_balance_summary.md`
- `/tmp/hphys0245_20260602T051933Z/reports/hphys0245_phase_storage_deltas.tsv`
- `/tmp/hphys0245_20260602T051933Z/reports/hphys0245_phase_storage_delta_summary.tsv`
- `/tmp/hphys0245_20260602T051933Z/reports/hphys0245_source_line_evidence.txt`
