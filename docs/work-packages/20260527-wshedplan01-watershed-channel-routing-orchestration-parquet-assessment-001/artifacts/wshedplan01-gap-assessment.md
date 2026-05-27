# WSHEDPLAN01 Gap Assessment

Status: complete

Evidence mode: static+ran

Date: 2026-05-26

## Static
| gap_id | Severity | Gap | Evidence | Closure target |
|---|---|---|---|---|
| WSHED-GAP-001 | critical | Watershed parquet writer is intentionally blocked, so watershed CLI cannot produce non-placeholder parquet outputs. | `openwepp-watershed-output/src/writers.rs:12-35`; CLI wrapper `CLIWAT-E-034` at `openwepp-cli-watershed.rs:1373-1375`; behavior test `watershed_cli_behavior_contract.rs:78-113`. | Implement data-backed watershed row models + parquet emission path for all required outputs, remove `OWSOUT-E-004` guard. |
| WSHED-GAP-002 | high | Python orchestration surface has no watershed runfile generation or execution API. | `open_wepp_runner/open_wepp_runner.py:558-587`. | Implement `make_watershed_*` and `run_watershed` surfaces with typed errors and contract-aligned output handling. |
| WSHED-GAP-003 | high | WS10 channel routing is scaffolded scalar branch logic, not baseline-authoritative `wshcqi/wshirs/wshrun/wshpek/wshchr/chrqin` migration. | WS10 kernel branches at `lib.rs:625-840`; baseline routine chain in `wshdrv.for:930-943,1098-1128`, `wshpek.for:151-301`, `wshchr.for:125-255`. | Migrate baseline routing routine families and state topology into canonical contracts + runtime kernels. |
| WSHED-GAP-004 | high | Channel hydraulics/erosion chain (`chnero -> chnrt`) is missing from openWEPP watershed execution path. | Baseline `chnero.for:139-142`; no equivalent watershed erosion runtime path in current orchestrator/CLI. | Add contract authority and kernel/runtime integration for channel erosion chain sequencing. |
| WSHED-GAP-005 | high | Impoundment path is present but simplified; not yet baseline-authoritative `wshimp` hydraulic/sediment process migration. | WS10 impoundment scaffold `lib.rs:972-1062`; baseline `wshimp.for:78-260`. | Contract-derived migration of impoundment mass/particle process families and downstream publication surfaces. |
| WSHED-GAP-006 | medium | Watershed orchestration chronology in Rust is split across CLI + orchestrator but does not yet represent full baseline event chronology contract. | CLI sequencing `openwepp-cli-watershed.rs:186-440`; baseline chronology `wshdrv.for:891-1133`. | Add explicit chronology contracts/tests and align runtime phase sequencing across channel/impoundment branches. |
| WSHED-GAP-007 | medium | Required output paths are contract-validated, but there is no produced watershed row-set model mapped to legacy publication files (`ebe_pw0`, `chan_out`, `chanwb`, etc.). | `contracts.rs:5-97` defines required outputs; writer has schemas only and no data emission path `writers.rs:15-34`. | Implement row model builders and wire writer to runtime execution report/state surfaces. |
| WSHED-GAP-008 | low | Package dependency list initially referenced nonexistent baseline file `chndet.for`. | Baseline source listing lacks `chndet.for`; `chnrt.for` exists and is called by `chnero.for:141`. | Correct dependency references to `chnrt.for`; retain trace in package artifacts. |

## Closure criteria for "fully scaffolded + parquet capable"
1. Watershed CLI and Python wrapper can execute end-to-end without placeholder
   writer failure for valid fixtures.
2. Watershed output writer emits all required parquet datasets with schema and
   metadata parity guards.
3. Contract-first queue completes channel routing + impoundment + channel
   erosion migration under canonical `SC-*` authority.
4. Watershed parity/disposition package records explicit GO/HOLD decision with
   residuals and ownership.

## Ran
- `nl -ba crates/openwepp-watershed-output/src/writers.rs | sed -n '1,240p'`
- `nl -ba crates/openwepp-watershed-output/src/contracts.rs | sed -n '1,280p'`
- `nl -ba crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs | sed -n '320,760p'`
- `nl -ba crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs | sed -n '860,930p'`
- `nl -ba crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs | sed -n '1350,1408p'`
- `nl -ba open_wepp_runner/open_wepp_runner.py | sed -n '520,620p'`
- `nl -ba crates/openwepp-watershed-orchestrator/src/lib.rs | sed -n '600,840p'`
- `nl -ba crates/openwepp-watershed-orchestrator/src/lib.rs | sed -n '980,1365p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshdrv.for | sed -n '860,1160p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshpek.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshchr.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/wshimp.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/chnero.for | sed -n '1,260p'`
