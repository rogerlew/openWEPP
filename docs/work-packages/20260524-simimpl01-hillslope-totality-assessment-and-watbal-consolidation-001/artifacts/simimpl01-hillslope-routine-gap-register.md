# Simimpl01 hillslope routine gap register

Status: phase-b-complete
Evidence mode: Static + Ran

## Static
- Scope covers the legacy hillslope execution surfaces sampled by SIMIMPL01:
  - `/workdir/wepp-forest_260430_baseline/src/watbal.for`
  - `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
  - `/workdir/wepp-forest_260430_baseline/src/contin.for`
  - `/workdir/wepp-forest_260430_baseline/src/winter.for`
  - `/workdir/wepp-forest_260430_baseline/src/soil.for`
  - `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
  - `/workdir/wepp-forest_260430_baseline/src/hydout.for`
- Assessment objective is implementation-driving mapping, not legacy routine-by-routine code porting in this package.
- Owner surfaces used for mapping:
  - `input-contract` (typed parser/input authority)
  - `runner` (runtime assembly + output publication)
  - `hillslope-orchestrator` (kernel scheduling + typed writeback)
  - `hillslope-output` (artifact writers)
  - `unowned-gap` (no evidenced owner in current production path)

## Ran
- Inventory extraction command:
  - `awk` extraction over baseline `watbal`, `watbal_hourly`, `contin`, `winter`, `soil`, `frsoil`, `hydout` for `subroutine` and `call` tokens.
- Mapping probes:
  - `rg -n "execute_hillslope_run|build_h5_wat_output|build_first_day_wat_projection|parse_wepp_ui_from_path" crates/openwepp-runner/src/lib.rs`
  - `rg -n "HillslopePhaseScheduler|execute_with_kernel|Wb11HydrologyKernel|HydrologyEvapotranspiration|HydrologyDrainage" crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - `rg -n "stmget" crates/openwepp-climate-runtime-adapter/src/lib.rs`

## Inventory totals
- Legacy root subroutines inventoried: `7`
  - `contin`, `watbal`, `watbal_hourly`, `winter`, `soil`, `frsoil`, `hydout`
- Downstream unique called routines inventoried from those roots: `59`

## Full downstream call inventory
`aspect`, `bighdr`, `bigout`, `close`, `decomp`, `drain`, `evap`, `evappm`, `frcfac`, `frostn`, `frsoil`, `hdreng`, `hr_tmp`, `infile`, `infpar`, `init1`, `initd`, `input`, `irrig`, `irs`, `newtil`, `nowup`, `outeng`, `outfil`, `param`, `print`, `prtcmp`, `ptgra`, `ptgrp`, `purk`, `radcur`, `range`, `res_dp`, `rngint`, `route`, `saxfun`, `scon`, `sedout`, `sedseg`, `sloss`, `snowd`, `soil`, `stmget`, `stmtim`, `strip`, `sumfrc`, `sumrun`, `sunmap`, `swu`, `tilage`, `undflo`, `watbal`, `watbal_hourly`, `wepp_observe`, `winit`, `winter`, `winthd`, `wshpas`, `xinflo`

## High-impact mapping matrix
| Legacy routine family | Legacy role | openWEPP owner surface | Status | Evidence anchors |
|---|---|---|---|---|
| `watbal` | daily water-balance wrapper and branch origin | `runner` + `hillslope-orchestrator` | `gap` (runner output path does not execute scheduler/kernel lane) | `crates/openwepp-runner/src/lib.rs:1463-1493`, `:2145-2436`; scheduler API `crates/openwepp-hillslope-orchestrator/src/lib.rs:9075-9371` |
| `watbal_hourly` | hourly branch wrapper | `runner` | `gap` (parsed mode not propagated to runtime lane selection) | `crates/openwepp-runner/src/lib.rs:1278-1289`, `:1373-1384` |
| `contin` | top-level seasonal/daily sequencing | `hillslope-orchestrator` | `partial` (typed scheduler exists but no production runner invocation evidence) | `crates/openwepp-hillslope-orchestrator/src/lib.rs:8879-9371`; runner non-invocation evidence in `crates/openwepp-runner/src/lib.rs:1112-1549` |
| `soil` / `frsoil` | soil/frozen-soil state update | `input-contract` + `hillslope-orchestrator` | `partial` (inputs adapted; full legacy coupling not closed end-to-end) | soil parser/runtime use `crates/openwepp-runner/src/lib.rs:43-45`, `:1407-1455`; hydrology/frost guards `crates/openwepp-hillslope-orchestrator/src/lib.rs:3164-3465` |
| `winter` | snow/frost hourly coupling | `input-contract` + `hillslope-orchestrator` | `partial` (snow/frost sidecars parsed; no production hourly watbal lane closure) | `crates/openwepp-runner/src/lib.rs:1220-1384`; hourly branch gap evidence above |
| `hydout` / publication surface | H.wat/WB13-like publication | `runner` + `hillslope-output` | `gap` (publication currently projection-first) | `crates/openwepp-runner/src/lib.rs:1463-1493`, `:2145-2268` |
| `decomp` | annual/perennial decomposition transition | `hillslope-orchestrator` | `mapped` (typed phase dispatch exists) | `crates/openwepp-hillslope-orchestrator/src/lib.rs:10554-10647` |
| `drain` | drainage lane | `hillslope-orchestrator` | `mapped` (typed drainage lane in WB11) | `crates/openwepp-hillslope-orchestrator/src/lib.rs:1544`, `:1659-1665`, `:1758-1810` |
| `evap` / `evappm` | ET lane | `hillslope-orchestrator` | `mapped` (typed ET lane exists; legacy naming not 1:1) | `crates/openwepp-hillslope-orchestrator/src/lib.rs:1537-1543`, `:1659`, `:1758-1810` |
| `stmget` | within-day climate breakpoint acquisition | `openwepp-climate-runtime-adapter` | `mapped` at adapter seam | `crates/openwepp-climate-runtime-adapter/src/lib.rs:22` |

## Residual routine ownership summary
- `mapped`/`partial` with concrete openWEPP owner evidence: `10` high-impact families above.
- Remaining legacy call names are treated as `unowned-gap` in SIMIMPL01 and rolled into the follow-on queue (`simimpl02` through `simimpl12`) for strict contract-first closure.
- No residual routine is marked closed solely by lexical symbol match.

## Phase B conclusion
- SIMIMPL01 now has a complete legacy call inventory for the assessed hillslope stack and an implementation-driving owner mapping.
- The dominant blocker is not missing parser presence; it is production wiring from `runner` into deterministic scheduler/kernel execution and simulation-owned output publication.
