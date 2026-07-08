# Operand Lineage

Evidence mode: Static + Ran.

## Annual Pass-Sediment Surface

| Operand | Source | Units | Use in this package |
|---|---|---|---|
| `tdep` | `H1.pass.parquet` | kg | Annual pass-sediment judged surface. The failing value is the sum of daily `tdep` rows where `year == 4`. |
| `tdet` | `H1.pass.parquet` | kg | Daily erosion-response companion used to check whether the implicated day is an erosion-consumer response day. |
| `runvol` | `H1.pass.parquet` | m3 | Published daily surface-water magnitude operand in the pass row. Identical between candidate/reference on day 1126. |
| `sbrunv` | `H1.pass.parquet` | m3 | Published daily lateral/subsurface delivery magnitude operand. Identical between candidate/reference on day 1126. |
| `peakro` | `H1.pass.parquet` | m3/s | Published peak runoff diagnostic. Identical between candidate/reference on day 1126. |

## Routed-Water Trace Surface

| Operand | Source | Units | Use in this package |
|---|---|---|---|
| `source_m3` | `laned_active_trace.jsonl` | m3 | Active routed source mass by lane-day. Day 1126 aggregate delta is zero. |
| `outlet_m3` | `laned_active_trace.jsonl` | m3 | Active routed lane outlet mass. Day 1126 aggregate delta is `0.01827025610623423 m3`. |
| `terminal_day_outlet_m3` | `laned_active_trace.jsonl` | m3 | Terminal routed hillslope outlet mass. Day 1126 delta is `-0.003359397088388505 m3` on a `4594 m3` event. |
| `routed_hourly_weights` | `laned_active_trace.jsonl` | fraction | D13 routed-hydrograph erosion-shape input. Day 1126 terminal lane L1 is `0.0006352335679617539`; max lane L1 is `0.0007414490157977821`. |
| `mesh_end_storage_m3` | `laned_active_trace.jsonl` | m3 | End-window storage evidence. Day 1126 aggregate delta is `0.0033593971208620843 m3`. |
| `tail_fold_m3` | `laned_active_trace.jsonl` | m3 | D13 tail-fold evidence. Day 1126 delta is zero. |
| `clamp_m3` | `laned_active_trace.jsonl` | m3 | Positivity-clamp evidence. Day 1126 candidate/reference are both zero. |

## Consumer Boundary

D13 authorizes the active erosion consumer to read the routed hourly shape when
Lane D routing owns the water path. This package does not change that consumer
or amend `SC-OFEROUTE-001`; it only attributes the failed annual sediment
adequacy surface against the already-recorded routed-water trace.
