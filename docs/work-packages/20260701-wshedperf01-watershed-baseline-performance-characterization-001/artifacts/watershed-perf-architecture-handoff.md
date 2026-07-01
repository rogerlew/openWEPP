# Watershed Performance Architecture Handoff

Status: `UPDATED`

## Required Conclusions

- Full legacy baseline scope (`full-legacy-watershed`) remains `0:07.86` wall on pinned WEPP baseline binary (`/tmp/wshedperf01_20260701_081511/timing/legacy_pw0_timed_1.time`).
- OpenWEPP routed-stage scope (`routed-stage-from-existing-hbp`) remains `~0:00.07` wall (`/tmp/wshedperf01_20260701_081511/timing/openwepp_watershed_from_hbp_timed_1.time`) and repeat set in `/tmp/wshedperf01_20260701_083200/timing/repeat_openwepp_routed/`.
- Practical full openWEPP end-to-end scope is now validated with 3 stable repeats in `/tmp/wshedperf01_20260701_101739/repeat_{1..3}`.
  - walls: `1:02.38`, `1:01.41`, `1:01.06` (avg `1:01.62`)
  - users: `61.73`, `60.82`, `60.51`
  - system: `0.64`, `0.59`, `0.55`
  - max RSS: `16,896 KB`
  - non-empty 14-file interchange outputs each repeat.
- Full-chain perf evidence added: `/tmp/wshedperf01_20260701_101739/perf_full_e2e/timing/openwepp_watershed_end2e_full_perf_perfstat.csv`.
- No direct, single-scope ratio to legacy is valid because the legacy command executes full legacy watershed from pw0, while openWEPP chain here composes hillslope commands + routed handoff.

## Dominant execution split

Ran:
- Routed stage alone is not the bottleneck.
- Hillslope dispatch/execution (`openwepp-cli-hill`) and per-hill input/output handling dominate end-to-end time.
- Concurrency and runfile/path determinism should be the first-order optimization frontier for scaling.
- The validated full command path should be used for all subsequent comparative studies once path behavior is locked.

## Lessons to carry into next package

- Preserve and enforce deterministic runfile and output-location contracts to avoid earlier blockers (`missing / readable pass file` and `CLIWAT-E-021` families).
- Reduce or remove compatibility-path sidecar noise (`legacy-sidecar-discovery`, `LSB-W-002`) before attributing micro-architecture deltas.
- Extend this package by adding 3+ repeat matrix across run counts and thread-count variants once baseline path is stabilized; capture hillslope-only and routed-only perf split for attribution.
