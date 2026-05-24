# simimpl04 expected fail pass matrix

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Matrix captures SIMIMPL04 pre-implementation blocker posture.
- `Expected fail now` = contract obligation not yet implemented in production runner path.
- `Expected pass after SIMIMPL05` = obligation should pass once production path closure lands.

## Matrix
| Test | Contract authority | Expected fail now evidence | Expected pass condition |
|---|---|---|---|
| `simimpl04_runner_kernel_execution_contract` | `INV-WATBAL-018`, `INV-SYSTEM-018` | Missing manifest pointer `/execution_provenance/scheduler_kernel_executed`. | Manifest exposes execution-owned publication provenance and guard linkage for SIMPIPE closure. |
| `simimpl04_wepp_ui_mode_closure_contract` | `INV-WATBAL-019`, `INV-SYSTEM-019`, `D-WUI-005`, `G-WUI-008`, `WUI-E-005` | Missing manifest pointer `/mode_selection/wepp_ui/requested`. | Manifest publishes requested/effective/selected-lane tuple with deterministic lane closure and guard linkage. |
| `simimpl04_wb13_publication_contract` | `INV-WATBAL-020`, `INV-SYSTEM-020`, `G-WUI-009` | Missing manifest pointer `/wb13_publication/source`. | Manifest publishes simulation-owned WB13 provenance (`source=simulation-owned`) and explicit no-projection-fallback posture. |

## Ran
- Fail-state evidence captured by explicit ignored-test execution commands recorded in `gate-results.md`.
