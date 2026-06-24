# Line-Count Governance

Status: COMPLETE.

Static:

- Final touched `.rs` line counts:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`: 681
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`: 239
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`: 2034
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`: 2730
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`: 948
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`: 194
  - `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs`: 488
  - `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`: 1815
  - `crates/openwepp-runner/src/hillslope/03_tests.rs`: 2769
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`: 1861
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs`: 1578
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/03_frost_comparator_seam.rs`: 8
- WARN files at 2000+ lines:
  - `coupling/frost.rs` at 2034 lines. This is the existing frost fine-layer
    helper module; this package reduced production surface coupling but did not
    attempt a physics-file split.
  - `coupling/frost_entry.rs` at 2730 lines. The typed extraction necessarily
    centralized the request-adapter and typed-entry boundary here; clippy
    line-shape findings were split during execution.
  - `crates/openwepp-runner/src/hillslope/03_tests.rs` at 2769 lines. This is
    below the 3000-line mandatory split threshold; new orchestrator parity tests
    were moved out of the oversized general direct-runtime test file into the
    dedicated R7G frost test module.
- No touched production `.rs` file is at or above 3000 lines.

Ran:

- `wc -l` over the final touched `.rs` write set.
- `git diff -- crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs | wc -l`
  returned `0`, confirming the pre-existing 3000+ direct-runtime test file is
  not touched by final package changes.
