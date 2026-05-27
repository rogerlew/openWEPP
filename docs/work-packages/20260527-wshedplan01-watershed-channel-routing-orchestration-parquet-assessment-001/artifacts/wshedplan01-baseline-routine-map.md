# WSHEDPLAN01 Baseline Routine Map

Status: complete

Evidence mode: static+ran

Date: 2026-05-26

## Static
- Baseline authority anchor:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- HBP contract discovery source commit used for this package:
  `/workdir/wepp-forest@924ab16d07edea8b904bcf64d3d7e276fc45d21e`.
- Baseline watershed event flow in `wshdrv` (channel + impoundment loop):
  - call `wshiqi` then `wshimp` for impoundment elements:
    `wshdrv.for:906-915`
  - call `wshcqi` and `wshirs` for channel inflow/runoff:
    `wshdrv.for:930-943`
  - call `wshrun` then `chnero` for runoff events:
    `wshdrv.for:1098-1113`
  - call `wshchr` for no-runoff branch when `ipeak>2`:
    `wshdrv.for:1119-1128`.

## Baseline-to-openWEPP mapping
| Baseline routine | Baseline responsibility | openWEPP surface | Status |
|---|---|---|---|
| `wshdrv.for` | master watershed orchestration chronology and per-element event loop | `openwepp-cli-watershed` intake + dispatch wrapper (`crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`) | partial |
| `wshcqi.for` | computes channel inflow volumes/duration and baseflow composition | WS10 runtime symbols + CLI runtime seeding (`runtime_inputs.rs`, `openwepp-cli-watershed.rs`) | partial |
| `wshirs.for` | channel runoff production under rainfall/runon chronology | no baseline-authoritative channel runoff routine migration yet | missing |
| `wshrun.for` | channel/impoundment runoff routing entry; calls `wshpek` | WS10 channel kernel entry (`run_channel_node`) | partial |
| `wshpek.for` | peak-flow logic for `ipeak` families; calls `wshchr` for routed branches | WS10 `ipeak` branch selection and scalar computations | partial |
| `wshchr.for` | channel routing hydrograph transform (`KW`/`MC`) with `chrqin` inflow assembly | no baseline-authoritative segment/time-step routing migration | missing |
| `chrqin.for` | builds channel inflow/lateral inflow hydrograph over `ntchr` | no direct migrated equivalent; only aggregate scalar contributor ingestion | missing |
| `wshimp.for` | impoundment hydraulic+sediment routing and WEPP-state conversion | WS10 impoundment scalar continuity/stage-discharge scaffold | partial |
| `chnero.for` + `chnrt.for` | channel hydraulics/erosion routine chain called after runoff routing | no migrated channel erosion runtime in openWEPP watershed kernel | missing |
| `wshout.for` | watershed reporting/publication | output contract declared; writer path blocked by `OWSOUT-E-004` | missing |

## Dependency-path correction
- `chndet.for` is not present in the pinned baseline source tree.
- Channel erosion/routing closure evidence should reference `chnrt.for` (called
  by `chnero.for`) instead:
  `chnero.for:141`.

## Ran
- `git -C /workdir/wepp-forest rev-parse HEAD`
- `ls /workdir/wepp-forest_260430_baseline/src | rg '^chn|^wsh|^chrq|^wshc|^wshi'`
- `rg -n "call +wsh|call +chr|call +chn|ipeak|nqs" /workdir/wepp-forest_260430_baseline/src/wshdrv.for /workdir/wepp-forest_260430_baseline/src/wshrun.for /workdir/wepp-forest_260430_baseline/src/wshpek.for /workdir/wepp-forest_260430_baseline/src/wshchr.for /workdir/wepp-forest_260430_baseline/src/wshimp.for /workdir/wepp-forest_260430_baseline/src/chnero.for /workdir/wepp-forest_260430_baseline/src/chrqin.for /workdir/wepp-forest_260430_baseline/src/wshcqi.for /workdir/wepp-forest_260430_baseline/src/wshirs.for`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshdrv.for | sed -n '860,1160p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshrun.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshpek.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshchr.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshimp.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshcqi.for | sed -n '1,240p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshirs.for | sed -n '1,240p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/chrqin.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/chnero.for | sed -n '1,260p'`
