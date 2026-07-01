# Worker Handoff

Status: `UPDATED`

Dispatched runner: `comparator_suite_runner` (subagent) `019f1e3c-8794-7da3-87e7-bd21878c4c56` (`Ramanujan`).

## Baseline numbers

- Pinned legacy full watershed (canonical scope): `0:07.86` wall, user `6.04`, system `1.81`, Max RSS `2,877,312 KB`.
  - Evidence: `/tmp/wshedperf01_20260701_081511/timing/legacy_pw0_timed_1.time`.
- openWEPP routed-stage from existing pass (`HBP/manifest` scope): `0:00.07` wall, user `0.07`, system `0.00`, Max RSS `8,448 KB`.
  - Evidence: `/tmp/wshedperf01_20260701_081511/timing/openwepp_watershed_from_hbp_timed_1.time`.
- openWEPP routed-stage repeats (N=3): walls `0:00.07`, `0:00.07`, `0:00.08`.
  - Evidence: `/tmp/wshedperf01_20260701_083200/timing/repeat_openwepp_routed/openwepp_routed_1.time`, `_2.time`, `_3.time`.
- openWEPP practical full pipeline (validated single run): `1:02.27` wall, user `61.74`, system `0.53`, Max RSS `16,896 KB`.
  - Evidence: `/tmp/wshedperf01_20260701_102200/timing/openwepp_watershed_end2end_full_validated.time`.

## Blockers and status

- Active blockers: `NONE` (command-surface now produces valid non-empty full end-to-end outputs).
- Historical blockers retained for traceability:
  - `CLIHILL-E-010` path resolution failures.
  - `CLIWAT-E-021` missing required output validation failures in earlier variants.
  - `pipeline_output/H1.hbp` pathability failures in first pipeline variant.

## Profiling attribution

- Coarse routed-stage profile (existing): `/tmp/wshedperf01_20260701_083200/timing/openwepp_watershed_perf_routed.time`.
  - `task-clock=80.46msec`, `instructions=338,400,048`, `branch-misses=3,263,638`, `ts_elapsed=0.11`, `ts_maxrss=13056`.
- Full end-to-end profile (`perf`) was not collected in this phase; command-timing validation confirms full run dominates outside routed-stage CLI.

## Next package recommendation

- Run a 3+ repeat full end-to-end suite using the validated command path:
  - `runfiles/p{1..36}_end2end3.run` for hillslope fanout.
  - `wshedperf01_openwepp_watershed_end2end_final.run` for watershed handoff.
- Add full-path `perf stat` around both hill dispatch and watershed routing once repeats are stable.

## Artifact paths

- `.../artifacts/required-reading-map.md`
- `.../artifacts/environment-and-input-inventory.md`
- `.../artifacts/baseline-command-log.md`
- `.../artifacts/watershed-baseline-timing.md`
- `.../artifacts/watershed-profile-attribution.md`
- `.../artifacts/watershed-perf-architecture-handoff.md`
- `.../artifacts/gate-results.md`
- `.../artifacts/disposition.md`
- `.../artifacts/worker-handoff.md`
