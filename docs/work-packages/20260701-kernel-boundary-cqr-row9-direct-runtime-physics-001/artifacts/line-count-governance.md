# Line-Count Governance

Evidence mode: Ran.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
```

Result:

```text
   767 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
  2535 crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs
  1736 crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs
  2654 crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs
  1731 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
  9423 total
```

## Disposition

Status: `PASS`.

All row #9 touched Rust files remain below the 3000-line governance threshold.
No line-count exception is used for this row.
