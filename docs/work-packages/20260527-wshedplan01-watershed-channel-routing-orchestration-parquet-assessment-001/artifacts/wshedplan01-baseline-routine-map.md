# WSHEDPLAN01 Baseline Routine Map

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static

### Authority anchors used
- Legacy migration comparator/authority baseline:
  - `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- HBP reader/writer contract provenance source inspected:
  - `/workdir/wepp-forest@924ab16d07edea8b904bcf64d3d7e276fc45d21e`

### Baseline routine-to-openWEPP map

| Legacy routine family | Legacy role | Current openWEPP surface | Migration state |
|---|---|---|---|
| `wshdrv.for` (`call` chain through `wshiqi`, `wshimp`, `wshcqi`, `wshirs`, `wshrun`, `chnero`, conditional `wshchr`) | Daily/event watershed execution controller and element loop | `openwepp-cli-watershed` run path + topology dispatch invocation | partial; deterministic dispatch exists but full daily/event process controller parity is not migrated |
| `wshcqi.for` | Channel inflow/runon volume and duration assembly (`rvolat`, `rvotop`, duration max logic, baseflow/subsurface interactions) | contributor payload assembly in CLI + `assemble_incoming_peak_and_duration` in kernel | partial; top/lateral decomposition and full duration lineage are not migrated |
| `wshirs.for` | Channel runoff generation/infiltration/transmission-loss cases | none in watershed orchestrator/kernel | missing |
| `wshrun.for` + `wshpek.for` | Channel runoff/peak coupling, ipeak method selection, run-duration/output updates | `run_channel_node` | partial; branch selector exists, but baseline method equations/hydrograph assembly are not migrated |
| `wshchr.for` + `chrqin.for` | `ipeak>2` wave routing (KW/MC), time-step arrays (`q1`, `qin`, `qlat`, segment coefficients) | no equivalent time-series routing state in runtime surfaces | missing |
| `chnero.for` + `chnrt.for` + `detach.for` | Channel sediment erosion/deposition routing and per-class transport closure | no channel-sediment production path in watershed orchestrator | missing |
| `wshiqi.for` | Impoundment inflow/peak assembly and source-mode logic (hillslope vs channel contributors) | contributor payload + dependency read path | partial |
| `wshimp.for` + `impmai/impflo/imphnw` lineage | Impoundment hydraulic + sediment routing, continuity/stage-discharge integration, duration/output closure | `run_impoundment_node` | partial; simplified continuity/outflow only, no RK4/adaptive retry/regime-transition/sediment closure |
| `wshout.for`, `monchn.for`, `annchn.for`, `endchn.for` | Watershed event/month/year output accumulation/publication | watershed-output crate schema scaffold only | missing output emission and reporting closure |

### Baseline reference correction
- The prepared package dependency list included
  `/workdir/wepp-forest_260430_baseline/src/chndet.for`.
- Static baseline inspection found no `chndet.for` file in the pinned baseline.
- The channel-detachment authority routine used by channel sediment routing is
  `/workdir/wepp-forest_260430_baseline/src/detach.for`, with orchestration
  through `chnrt.for` and `chnero.for`.

## Ran
- Baseline call-graph and definition extraction via `rg`, `sed`, and `nl` on:
  - `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshirs.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshrun.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshiqi.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshimp.for`
  - `/workdir/wepp-forest_260430_baseline/src/chnero.for`
  - `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
  - `/workdir/wepp-forest_260430_baseline/src/detach.for`
  - `/workdir/wepp-forest_260430_baseline/src/chrqin.for`
