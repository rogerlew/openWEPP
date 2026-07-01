# Watershed Profile Attribution

Status: `UPDATED`

## Attribution Summary

Ran:
- `perf`-backed coarse attribution for routed-stage openWEPP was collected from:
  - `/tmp/wshedperf01_20260701_083200/timing/openwepp_watershed_perf_routed.time`

- `perf stat` counters (single routed run):
  - `task-clock: 80.46 msec`
  - `cycles: 229,855,976`
  - `instructions: 338,400,048`
  - `branches: 66,967,370`
  - `branch-misses: 3,263,638` (4.87% of branches)
  - `ts_user=0.07`, `ts_sys=0.02`, `ts_elapsed=0.11`, `ts_maxrss=13056`.
  - `/usr/bin/time` route command for repeats: wall `0:00.07`/`0:00.08`, max RSS `8,448 KB`, user `0.07`, system `0.00`.

## Coarse staged interpretation

Ran:
- The command is short and dominated by process-level overhead and fast pass-oriented execution rather than long compute loops.
- Routing + CLI orchestration appears lightweight:
  - very low memory (`~8 MiB` RSS)
  - low instruction count and elapsed time (`~0.07–0.11s`)
- Output writing is present but small for surfaced outputs (`~288` bytes in one repeat, 576 bytes on the short full-attempt), indicating post-processing is not the long-latency component at this stage.
- Significant extra sidecar warnings were observed in routed runs (`legacy-sidecar-discovery` and `LSB-W-002`), which adds process startup/lookup work and confirms compatibility path handling remains active.

## End-to-end attribution notes

Ran:
- Validated full-path openWEPP command:
  - `/tmp/wshedperf01_20260701_102200/timing/openwepp_watershed_end2end_full_validated.time`
- Measured timing: wall `1:02.27`, user `61.74`, system `0.53`, max RSS `16,896 KB`.
- Output validation: `14` non-empty parquet files were produced under `/tmp/wshedperf01_20260701_102200/outs/openwepp_end_to_end_validated/interchange`.
- No targeted `perf stat` run was executed for this full path in this phase; this is command-timing-only evidence.
- The practical architecture signal remains: routed-stage remains tiny, and end-to-end cost is dominated by hillslope command fanout/concurrent execution plus output aggregation surface.
