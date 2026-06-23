# Line-Count Governance

Status: held.

Measured at HOLD closure:

```text
4421 crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs
2580 crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs
2064 crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs
1978 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs
1323 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs
1171 crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs
1132 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1046 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs
```

Disposition:

- This package violated its own preference to split
  `day_input_and_helpers.rs` before substantial Rust expansion.
- `00_core_frames.rs` is also now above the intended direct-runtime section
  limit.
- The follow-up frost sub-solver package should start with mechanical splits
  for these files, then implement typed stateful active frost in smaller
  sections.
