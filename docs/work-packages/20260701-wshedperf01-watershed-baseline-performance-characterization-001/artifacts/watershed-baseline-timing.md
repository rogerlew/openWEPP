# Watershed Baseline Timing

Status: `UPDATED`

## Timing Table

| Surface | Scope Label | Repeats | Walltime | User CPU | System CPU | Max RSS | Ratio Notes | Evidence |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| Pinned legacy WEPP | `full-legacy-watershed` | `1` | `0:07.86` | `6.04` | `1.81` | `2,877,312 KB` | `No comparable ratio: legacy run includes full watershed execution from WEPP_260430 baseline.` | `/tmp/wshedperf01_20260701_081511/timing/legacy_pw0_timed_1.time` |
| openWEPP watershed CLI | `routed-stage-from-existing-hbp` | `1` canonical + `3` repeated (`openwepp_watershed_from_hbp_timed_1 + routed_1..3`) | `0:00.07` (canonical) / `0:00.07–0:00.08` (repeat range) | `0.07–0.07` | `0.00–0.00` | `8,448 KB` | `Non-comparable to legacy full run without fresh per-hill simulation stage; route-only scope excludes hillslope execution and pass generation.` | `/tmp/wshedperf01_20260701_081511/timing/openwepp_watershed_from_hbp_timed_1.time`, `/tmp/wshedperf01_20260701_083200/timing/repeat_openwepp_routed/openwepp_routed_1.time`, `/tmp/.../openwepp_routed_2.time`, `/tmp/.../openwepp_routed_3.time` |
| openWEPP end-to-end pipeline | `full-openwepp-if-practical` | `1` validated full end-to-end run | `1:02.27` | `61.74` | `0.53` | `16,896 KB` | `Not directly comparable to legacy full run without equivalent legacy scope, but this run includes fresh hillslope generation + routing handoff in one command chain.` | `/tmp/wshedperf01_20260701_102200/timing/openwepp_watershed_end2end_full_validated.time` |
