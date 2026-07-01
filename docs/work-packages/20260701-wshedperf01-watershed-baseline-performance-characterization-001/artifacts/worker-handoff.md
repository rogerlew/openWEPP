# Worker Handoff

Status: `UPDATED`

Initial dispatched runner: `comparator_suite_runner` (subagent)
`019f1e3c-8794-7da3-87e7-bd21878c4c56` (`Ramanujan`). The dispatch errored
because the selected model was at capacity, so benchmark execution completed
locally with command-level evidence.

## Baseline numbers

- Pinned legacy full watershed (canonical scope): `0:07.86` wall, user `6.04`, system `1.81`, Max RSS `2,877,312 KB`.
  - Evidence: `/tmp/wshedperf01_20260701_081511/timing/legacy_pw0_timed_1.time`.
- openWEPP routed-stage from existing pass (`HBP/manifest` scope): `0:00.07` wall, user `0.07`, system `0.00`, Max RSS `8,448 KB`.
  - Evidence: `/tmp/wshedperf01_20260701_081511/timing/openwepp_watershed_from_hbp_timed_1.time`.
- openWEPP routed-stage repeats (N=3): walls `0:00.07`, `0:00.07`, `0:00.08`.
  - Evidence: `/tmp/wshedperf01_20260701_083200/timing/repeat_openwepp_routed/openwepp_routed_1.time`, `_2.time`, `_3.time`.
- openWEPP full end-to-end validated repeats (N=3): walls `1:02.38`, `1:01.41`, `1:01.06` (`avg 1:01.62`), user `60.82–61.73`, system `0.55–0.64`, Max RSS `16,896 KB`.
  - Evidence:
    - `/tmp/wshedperf01_20260701_101739/repeat_1/timing/openwepp_watershed_end2end_full_validated_repeat1.time`
    - `/tmp/wshedperf01_20260701_101739/repeat_2/timing/openwepp_watershed_end2end_full_validated_repeat2.time`
    - `/tmp/wshedperf01_20260701_101739/repeat_3/timing/openwepp_watershed_end2end_full_validated_repeat3.time`
- one full-chain profile run:
  - `1:02.07` wall, user `61.48`, system `0.56`, RSS `16,896 KB`.
  - perf counters in `/tmp/wshedperf01_20260701_101739/perf_full_e2e/timing/openwepp_watershed_end2e_full_perf_perfstat.csv`.

## Blockers and status

- Historical blockers retained for traceability:
  - `CLIHILL-E-003` and path-copy staging mistakes during early repeat harness bootstrap.
  - `CLIHILL-E-010` path resolution failures.
  - `CLIWAT-E-021` missing required output validations.
  - `pipeline_output/H1.hbp` pathability failures in first pipeline variant.
- Active blockers: `NONE` (validated full end-to-end command now produces stable valid non-empty outputs with current scope).

## Profiling attribution

- Routed-stage profile: `/tmp/wshedperf01_20260701_083200/timing/openwepp_watershed_perf_routed.time`.
- Full-chain profile: `/tmp/wshedperf01_20260701_101739/perf_full_e2e/timing/openwepp_watershed_end2e_full_perf_perfstat.csv`.
- Both profiles and timing show routed stage is not dominant.

## Next package recommendation

- Preserve this validated command contract and run a 3x3 matrix in the next package:
  - hill concurrency levels × command repeats.
  - attribute separately: hillslope fanout, pass ingestion, and routing handoff.
- Keep scope labels explicit when comparing to legacy; no legacy-equivalent full command has yet been introduced in this package.

## Artifact paths

- `artifacts/required-reading-map.md`
- `artifacts/environment-and-input-inventory.md`
- `artifacts/baseline-command-log.md`
- `artifacts/watershed-baseline-timing.md`
- `artifacts/watershed-profile-attribution.md`
- `artifacts/watershed-perf-architecture-handoff.md`
- `artifacts/gate-results.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`
