# Watershed Profile Attribution

Status: `UPDATED`

## Attribution Summary

Ran:
- Routed-stage `perf stat` evidence for openWEPP from
  `/tmp/wshedperf01_20260701_083200/timing/openwepp_watershed_perf_routed.time`.
- Routed-stage `perf stat` metrics:
  - `task-clock: 80.46 msec`
  - `cycles: 229,855,976`
  - `instructions: 338,400,048`
  - `branches: 66,967,370`
  - `branch-misses: 3,263,638` (`4.87%` of branches)
  - `ts_user=0.07`, `ts_sys=0.02`, `ts_elapsed=0.11`, `ts_maxrss=13056`.
- Full end-to-end `perf stat` evidence from
  `/tmp/wshedperf01_20260701_101739/perf_full_e2e/timing/openwepp_watershed_end2e_full_perf_perfstat.csv` and corresponding `/usr/bin/time` wrapper.
- Full path `perf stat` counters:
  - `task-clock: 62,005.57 msec`
  - `cycles: 198,953,606,460`
  - `instructions: 376,497,201,914`
  - `branches: 60,387,052,322`
  - `branch-misses: 603,162,941` (`1.00%`)
  - `context-switches: 326`
  - `cpu-migrations: 12`
  - `page-faults: 127,347`
  - `ts_user=61.48`, `ts_sys=0.56`, `ts_elapsed=1:02.07`, `ts_maxrss=16896`.

## Coarse staged interpretation

Ran:
- Routed stage remains short and orchestration-light (`0.07–0.11 s`, ~8 MiB RSS), with most time spent in fast parse/dispatch+intake and minimal output payload.
- Full openWEPP watershed path is clearly dominated by hillslope command fanout and run execution:
  - 36 `openwepp-cli-hill` invocations plus one routed-stage handoff.
  - walltime is now stable near ~61.6 s with tight RSS and low context-switching.
- Branch-miss ratio improves on full-chain profile (`1.00%`) versus routed command (`4.87%`), consistent with longer steady compute and more regular instruction streams.
- Sidecar compatibility probing remains active (`legacy-sidecar-discovery`, `LSB-W-002` unknown sidecar warnings), indicating non-native input resolution overhead remains present in validated path.
