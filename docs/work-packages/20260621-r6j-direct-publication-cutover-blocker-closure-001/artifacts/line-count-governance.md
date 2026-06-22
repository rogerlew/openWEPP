# R6J Line-Count Governance

Evidence class: Ran.

Command:

`wc -l crates/openwepp-runner/src/constants.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/src/hillslope/04_direct_publication.rs crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-output/src/hillslope_pass.rs crates/openwepp-hillslope-output/src/hillslope_wat.rs`

## Counts

| File | Lines | Status | Disposition |
| --- | ---: | --- | --- |
| `crates/openwepp-runner/src/constants.rs` | 26 | PASS | No action. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2997 | WARN | Below the 3000 hard threshold but uncomfortably close. R6J added run-local counter and direct-authority guards here; the next package touching this file should split runner publication/manifest helpers before adding more scope. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 2413 | WARN | Existing output-helper WARN band. R6J added direct required-operand checks here. Follow-on split remains advisable, not closure-blocking for this package. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 1902 | PASS | No action. |
| `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` | 1919 | PASS | No action. R6J split the direct publication row converter to satisfy clippy line-count governance. |
| `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs` | 147 | PASS | No action. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2922 | WARN | Below the 3000 hard threshold. R6J added explicit zero-authority and HBP-specific erosion publication operands. Follow-on direct-runtime work should split publication-row construction before adding more scope. |
| `crates/openwepp-hillslope-output/src/hillslope_pass.rs` | 452 | PASS | No action. |
| `crates/openwepp-hillslope-output/src/hillslope_wat.rs` | 905 | PASS | No action. |

No touched Rust file is at or above the 3000-line hard threshold.
