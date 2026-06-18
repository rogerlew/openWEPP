# PERFARRAY02 Request/Accessor Seam

Evidence: Static + Ran.

## What Changed

`HillslopeKernelRequest` now carries optional `ArrayHotState` authority beside the
existing logical and indexed surfaces. The new constructor
`with_transition_context_and_indexed_array` preserves the default constructor path by
passing `None` for the array state.

WB11 scalar accessors now select array reads first when `request.has_array_hot_state()`
is true:

- direct scalar symbols resolve through hot scalar tables;
- `_0001` series suffixes resolve through hot series tables;
- `_0001_0002` grid suffixes resolve through hot grid tables;
- array mode does not fall back to the logical maps for WB11 scalar reads.

Static anchors:

- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs:27`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs:47`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs:87`

## Default Path

Static: normal request construction still calls `with_transition_context_and_indexed`,
which delegates to the array-capable constructor with `array_hot_state: None`.

Ran:

```text
cargo test --test wb14_infiltration_hyetograph_kernel_contract perfarray02_wb14_runoff_reads_from_array_hot_state -- --nocapture
```

Result: pass.

Ran:

```text
cargo test --workspace
```

Result: pass.
