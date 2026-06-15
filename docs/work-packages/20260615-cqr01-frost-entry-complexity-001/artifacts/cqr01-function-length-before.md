# CQR01 Function-Length Baseline

Status: complete

Evidence mode: static-and-ran

## Static

- Target file before edits:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`.
- Baseline file line count: `1000`.
- Baseline `compute_active_frost_coupling` span: start line `73`, end line
  `999`, length `927`.
- Baseline suppression census: one
  `#[allow(clippy::too_many_lines)]` attached to `compute_active_frost_coupling`.

## Ran

- `wc -l crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
  - exit_code: 0
  - result: `1000`
- Function-span inspection by local line parser:
  - exit_code: 0
  - result: `compute_active_frost_coupling` length `927`
- `rg -n "clippy::too_many_lines" crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
  - exit_code: 0
  - result: suppression present at baseline line `72`
