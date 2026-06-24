# Line-Count Governance

Status: COMPLETE.

Static:
- Final touched Rust line counts:
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`: 2307 WARN.
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`: 556.
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`: 647.
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`: 2071 WARN.
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`: 691.
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/mod.rs`: 5.
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`: 872.
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`: 194.
  - `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`: 2973 WARN, below 3000 after mechanical split.
  - `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3c_r4b.rs`: 796.
  - `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs`: 379.
  - `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`: 1816.
  - `crates/openwepp-runner/src/hillslope/03_tests.rs`: 2765 WARN.
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`: 3.
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`: 1892.
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs`: 1527.

Disposition:
- No touched `.rs` file is 3000+ lines after closure.
- 2000+ line WARN files are existing dense surfaces or the reduced direct
  runtime test module. No additional split is required for this package.
