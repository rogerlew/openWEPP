# refactor014-kernel-implementation-and-test-evidence

Status: in-progress
Evidence mode: Ran

## Static:
- Files touched:
  - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel.rs`
  - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/kernel_core.rs`
  - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/constants.rs`
  - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/types.rs`
  - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`
  - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`
  - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`
  - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs`
- Implementation is mechanical extraction from `kernel_core.rs` (backup
  source: `/tmp/kernel_core.rs.bak`) into bounded module chunks with preserved
  function boundaries.

## Ran:
- `cargo check -p openwepp-watershed-orchestrator` passed after the split.
- `cargo clippy --workspace --all-targets -- -D warnings` initially failed on
  `clippy::too_many_lines` for moved `run_channel_node`, then passed after restoring
  the existing allow attribute in `validation.rs`.
