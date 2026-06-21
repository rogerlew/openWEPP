# Data-Path Proof

Status: in progress.
Evidence mode: Static + Ran.

## Current Path

| Segment | Current source | R6A status |
|---|---|---|
| Direct producer | R3/R4/R5 direct runtime spans and shadows | Existing internal direct state only. |
| In-memory publication frame | narrow `DirectPublicationFrame` | Insufficient; not run-bound to promoted ledger. |
| Runner handoff | `DirectRunFrame::skeleton` in explicit direct modes | Insufficient; skeleton evidence only. |
| HBP consumer | `build_hbp_output(..., &HillslopeWritebackSurface, ...)` | Compatibility path. |
| WAT consumer | `build_hillslope_wat_rows(&execution.wb13_rows)` | Compatibility WB13 rows. |
| PASS consumer | `write_hillslope_pass_parquet(..., &execution.pass_rows, ...)` | Rows derived from compatibility WB13/outlet rows. |
| loss consumer | `build_loss_output_json(...)` from static/climate summaries | No direct frame consumer. |
| manifest consumer | `write_hillslope_run_manifest(...)` helpers | No direct frame consumer. |

## Required Closing Path

| Segment | Required R6A evidence |
|---|---|
| Direct producer | Typed direct run/lane/day state identified for each promoted ledger operand. |
| In-memory publication frame | Run-bound direct publication frame populated from direct state only. |
| Runner handoff | Explicit direct-publication path over real parsed run dimensions. |
| HBP consumer | Direct HBP projection function taking the publication frame. |
| WAT consumer | Direct WAT row projection function taking the publication frame. |
| PASS consumer | Direct PASS row projection function taking the publication frame. |
| loss consumer | Direct loss JSON projection function taking the publication frame. |
| manifest consumer | Direct manifest/provenance projection function taking the publication frame. |
| Negative proof | Source/runtime proof that direct consumers do not read compatibility WB13 rows, runtime symbols, writeback payloads, or stale logical state. |

## Implemented R6A Path

| Segment | Implemented source | Evidence |
|---|---|---|
| Direct producer | `DirectFrameExecutor::run_publication_capture` executes direct spans and captures typed `DirectDayFrame` state during each run/lane/day commit. | `cargo test -p openwepp-hillslope-orchestrator r6a_publication_capture_records_run_bound_rows_without_publication_alias -- --nocapture` passed. |
| In-memory publication frame | `DirectRunPublicationFrame` containing `DirectPublicationDayRow` and typed HBP/WAT/PASS/loss/manifest operand groups. | `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`. |
| Runner handoff | `HillslopeRuntimeSelection::DirectPublicationFrameShadow` builds the direct frame from parsed slope OFE areas and climate calendar days after compatibility execution and before public output writes. | `cargo test -p openwepp-runner r6a_ -- --nocapture` passed. |
| HBP consumer | `build_hbp_output_from_direct_publication(&DirectRunPublicationFrame)`. | Focused direct projection consumer test passed. |
| WAT consumer | `build_hillslope_wat_rows_from_direct_publication(&DirectRunPublicationFrame)`. | Focused direct projection consumer test passed. |
| PASS consumer | `build_hillslope_pass_rows_from_direct_publication(&DirectRunPublicationFrame)`. | Focused direct projection consumer test passed. |
| loss consumer | `build_loss_output_json_from_direct_publication(&DirectRunPublicationFrame)`. | Focused direct projection consumer test passed. |
| manifest consumer | `build_manifest_text_from_direct_publication(&DirectRunPublicationFrame)`. | Focused direct projection consumer test passed. |
| Negative proof | Source scans over direct builder/projection ranges found no `SimulationOwnedWb13Row`, `HillslopeWritebackSurface`, `BoundarySymbol`, `BoundaryValue`, `KernelWritebackPayload`, `SymbolRegistry`, `runtime_surface`, `wb13_rows`, or bare `pass_rows` reads. | Scan commands recorded in `no-compatibility-proof-checklist.md`. |

R6A still preserves public compatibility output writers. Full HBP/WAT/PASS/loss
and manifest writer cutover remains R6 scope after this frame and consumer path
exist.
