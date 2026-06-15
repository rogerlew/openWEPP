# CQR01 Function-Length After

Status: complete

Evidence mode: static-and-ran

## Static

- Target file after refactor line count: `1507`, below the 2000-line warning
  threshold.
- `compute_active_frost_coupling` after refactor: start line `1453`, length
  `55` by line-span parser (`53` source-body lines observed during focused
  inspection).
- No `clippy::too_many_lines` suppression remains in the target file.
- Largest target helpers by line-span parser:
  - `require_frost_layer_water_state`: `98`
  - `finalize_active_frost_coupling`: `86`
  - `advance_active_frost_hour`: `82`
  - `compute_active_frost_final_scalars`: `79`
  - `apply_active_frost_thaw_step`: `75`

## Ran

- `wc -l crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
  - exit_code: 0
  - result: `1507`
- `rg -n "clippy::too_many_lines|allow\\(" .../frost_entry.rs`
  - exit_code: 0
  - result: only pre-existing wildcard import allowances at lines `1` and `4`
- Function-span parser:
  - exit_code: 0
  - result: largest function `98`; `compute_active_frost_coupling` line `1453`
