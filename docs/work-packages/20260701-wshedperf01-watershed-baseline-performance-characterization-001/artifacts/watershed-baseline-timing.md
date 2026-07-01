# Watershed Baseline Timing

Status: `UPDATED`

## Timing Table

| Surface | Scope Label | Repeats | Walltime | User CPU | System CPU | Max RSS | Ratio Notes | Evidence |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| Pinned legacy WEPP | `full-legacy-watershed` | `1` | `0:07.86` | `6.04` | `1.81` | `2,877,312 KB` | `No direct ratio against current openWEPP scope; legacy run includes full watershed execution using WEPP_260430 baseline.` | `/tmp/wshedperf01_20260701_081511/timing/legacy_pw0_timed_1.time` |
| openWEPP watershed CLI | `routed-stage-from-existing-hbp` | `1` canonical + `3` repeats | `0:00.07` (canonical) / `0:00.07–0:00.08` (repeat range) | `0.07–0.07` | `0.00–0.00` | `8,448 KB` | `Not comparable to legacy full run; scoped to route only from pre-generated HBP/manifests.` | `/tmp/wshedperf01_20260701_081511/timing/openwepp_watershed_from_hbp_timed_1.time`, `/tmp/wshedperf01_20260701_083200/timing/repeat_openwepp_routed/openwepp_routed_1.time`, `/tmp/.../openwepp_routed_2.time`, `/tmp/.../openwepp_routed_3.time` |
| openWEPP end-to-end pipeline | `full-openwepp-if-practical` | `4` total; `3` repeats for stability + `1` profile run | `1:02.38`, `1:01.41`, `1:01.06` (`avg 1:01.62`); profile run `1:02.07` | `61.73`, `60.82`, `60.51` (`avg 60.69`) | `0.64`, `0.59`, `0.55` (`avg 0.60`) | `16,896 KB` | `Dominant cost bucket is hillslope dispatch + per-hill execution + output path handoff; routed command remains separate and tiny.` | `/tmp/wshedperf01_20260701_101739/repeat_1/timing/openwepp_watershed_end2end_full_validated_repeat1.time`, `/tmp/wshedperf01_20260701_101739/repeat_2/.../openwepp_watershed_end2end_full_validated_repeat2.time`, `/tmp/wshedperf01_20260701_101739/repeat_3/.../openwepp_watershed_end2end_full_validated_repeat3.time`, `/tmp/wshedperf01_20260701_101739/perf_full_e2e/timing/openwepp_watershed_end2e_full_perf.time` |
