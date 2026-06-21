# R6H Line-Count Governance

Status: executed-warn.

Ran:

`wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/src/hillslope/04_direct_publication.rs crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs`

| File | Line count | Status | Disposition |
|---|---:|---|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2802 | WARN | Existing direct-runtime monolith remains below 3000. No R6H refactor required before held disposition; future direct-runtime packages should prefer extraction before adding broad new surfaces. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 2839 | WARN | Existing test monolith remains below 3000. R6H added one focused interleaved capture test; future test growth should move new families into split modules. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2855 | WARN | Existing runner intake file remains below 3000. R6H touched hold-marker wiring only. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 1425 | OK | No action required. |
| `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` | 1195 | OK | R6H implementation stayed in the prior publication helper split. |
| `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs` | 93 | OK | No action required. |

Files at or above 2000 lines require WARN disposition. Non-exempt files at or
above 3000 lines require refactor-before-closure unless a package-specific
exception is explicitly reviewed and time-boxed.

R6H does not claim complete publication cutover, and no touched file is at or
above the 3000-line refactor-before-closure threshold.
