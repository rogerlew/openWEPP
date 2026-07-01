# Progress Scan

Evidence class: Static source scan.

Command:

```bash
PAT='HillslopeWritebackSurface|HillslopeKernelRequest|KernelWritebackPayload|SymbolRegistry|IndexedWritebackSurface|HillslopeDayFrame|BoundarySymbol|BoundaryValue|WritebackField|IndexedKernelWritebackPayload|IndexedWritebackField'
for f in \
  crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_storage_reconciliation.rs \
  crates/openwepp-runner/src/hillslope/scheduler_trace/hphys_trace.rs \
  crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs
do
  printf '%s ' "$f"
  rg -o "$PAT" "$f" | wc -l
done
```

Result:

| File | Carrier/symbol/writeback refs |
| --- | ---: |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs` | 193 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs` | 59 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs` | 104 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_storage_reconciliation.rs` | 10 |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/hphys_trace.rs` | 15 |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs` | 6 |

Progress: no burn-down. The package stopped before an invalid wrapper-style
migration.
