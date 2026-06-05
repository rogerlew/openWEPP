# Worker Handoff

Status: complete
Evidence mode: Static + Ran

## Completed Work

Static:

- Added canonical post-winter rain publication authority:
  `SC-WATBAL-001#INV-WATBAL-065`,
  `SC-RUNOFFPART-001#INV-RUNOFFPART-020`, and
  `SC-SNOWFREEZE-001#INV-SNOWFREEZE-023`.
- Published `snow.post_winter_rain_m` from runoff reconciliation as the
  post-winter direct rain equivalent.
- Updated WB13 `RM` to consume
  `snow.post_winter_rain_m + snow.routed_melt_m + Irr`.
- Required `snow.post_winter_rain_m` from the flux surface so state-only
  defaults cannot satisfy publication.
- Registered `snow.post_winter_rain_m` in the unit registry and tests.

## Validation

Ran:

- Focused HPHYS0290 runner tests: `6 passed`.
- Source contract test: `3 passed`.
- Unit-registry metadata test: `1 passed`.
- Final required gates:
  `/tmp/hphys0290_final_gates_20260605T013019Z_after_nan/status.tsv`, all
  return codes `0`.
- Full H1..H39 run:
  `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix`, runtime
  `39/39`, semantic `0/39`.
- Target traces:
  `/tmp/hphys0290_target_traces_current_20260605T011834Z_postfix`.

## Key Metrics

Ran:

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | ---: | ---: | ---: | ---: |
| Ep | 0/39 | 45401 | 0.727061 | 7.242659 |
| Total-Soil | 0/39 | 52521 | 57.069194 | 348.886998 |
| SoilWaterTotal | 0/39 | 52521 | 57.069194 | 348.886998 |
| Dp | 1/39 | 9220 | 0.042845 | 0.244800 |
| latqcc | 0/39 | 36003 | 0.373461 | 11.865076 |
| Q | 0/39 | 2108 | 0.552220 | 38.472185 |
| RM | 0/39 | 7097 | 0.256086 | 27.960000 |
| Snow-Water | 0/39 | 10391 | 2.899431 | 65.506840 |

## Continuation Focus

Static:

- Continue upstream of WB13 publication: snowpack timing/state and
  runoff/storage partition lineage.
- Preserve HPHYS0290 fail-closed flux requirement for
  `snow.post_winter_rain_m`.
- Add a producer/scheduler lifecycle regression in a follow-up if touching the
  daily kernel lifecycle.
- Clean up inert WB13 row-builder `_runtime_swe_before_m` when the WB13
  publication helper is next refactored.

Handoff status: ready for next work-package scaffolding.
