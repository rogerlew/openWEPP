# Layout And Type-Size Evidence

Status: complete.

## Evidence

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r7b_constructor_type_size_layout_is_bounded -- --nocapture`

Observed output:

- `DirectRunConstructorInputs=72`
- `DirectLaneConstructorInputs=608`
- `DirectDayConstructorInputs=1760`
- `DirectRunFrame=256`
- `DirectLaneFrame=608`
- `DirectDayFrame=8800`

Static:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
  is `1584` lines after R7B.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` remains `210`
  lines.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
  is `3204` lines. This is test code; line-count governance hard block applies
  to non-exempt production `.rs` files.
