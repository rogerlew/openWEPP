# Gate Results

Status: `UPDATED`

| Gate | Result | Evidence |
| --- | --- | --- |
| Required reading map complete | `PASS` | `artifacts/required-reading-map.md` |
| Environment/input inventory complete | `PASS` | `artifacts/environment-and-input-inventory.md` |
| Release openWEPP watershed binary built | `PASS` | `target/release/openwepp-cli-watershed`; `artifacts/baseline-command-log.md` entry #1 and #21 |
| Pinned legacy result or blocker recorded | `PASS` | `timing/legacy_pw0_timed_1.time` (Ran, exit `0`) |
| openWEPP routed-stage timing recorded | `PASS` | `/tmp/wshedperf01_20260701_081511/timing/openwepp_watershed_from_hbp_timed_1.time`; `/tmp/wshedperf01_083200/timing/repeat_openwepp_routed/openwepp_routed_{1,2,3}.time` |
| openWEPP practical full end-to-end timing recorded | `PASS` | `/tmp/wshedperf01_20260701_102200/timing/openwepp_watershed_end2end_full_validated.time` and `/tmp/wshedperf01_20260701_101739/repeat_* /timing/openwepp_watershed_end2end_full_validated_repeat*.time` |
| openWEPP full end-to-end repeats for stability | `PASS` | `/tmp/wshedperf01_20260701_101739/repeat_1` `/tmp/wshedperf01_20260701_101739/repeat_2` `/tmp/wshedperf01_20260701_101739/repeat_3` |
| Scope labels and repeat counts recorded | `PASS` | `artifacts/watershed-baseline-timing.md` |
| Profiling/coarse attribution recorded | `PASS` | `artifacts/watershed-profile-attribution.md`, `timing/openwepp_watershed_perf_routed.time`, `timing/openwepp_watershed_end2e_full_perf_perfstat.csv` |
| Architecture handoff complete | `PASS` | `artifacts/watershed-perf-architecture-handoff.md` |
| No production files edited | `PASS` | `git status --short` shows only package artifacts + `docs/work-packages/README.md` pointer |
| Final disposition written | `PASS` | `artifacts/disposition.md` |
