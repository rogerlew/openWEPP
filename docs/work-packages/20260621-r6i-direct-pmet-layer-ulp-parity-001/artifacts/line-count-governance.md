# R6I Line-Count Governance

Evidence class: Ran.

Command:

`wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/src/hillslope/04_direct_publication.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`

Result:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2902 | WARN, below 3000 hard threshold. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 1533 | PASS. |
| `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` | 1333 | PASS. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 2890 | WARN, below 3000 hard threshold. |
| `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs` | 94 | PASS. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 169 | PASS. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2855 | WARN, below 3000 hard threshold. |

No touched file crosses the 3000-line refactor-before-closure threshold.
