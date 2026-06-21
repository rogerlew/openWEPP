# R6G Line-Count Governance

Status: warn.

Ran: `wc -l` over touched `.rs` files.

| File | Line count | Status | Disposition |
|---|---:|---|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2763 | WARN | Existing monolithic direct-runtime frame file remains below 3000-line hard stop. Follow-on R work should split publication/direct frame carry into a dedicated module. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2838 | WARN | Existing runner orchestration file remains below 3000-line hard stop after prior R6 helper split. Follow-on R work should continue moving cutover gates/publication orchestration into narrow modules. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 2331 | WARN | Existing output helper file remains below 3000-line hard stop. Follow-on R6 publication work should isolate WAT/direct output builders if this surface grows. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 1325 | PASS | Below warning threshold. |
| `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` | 1015 | PASS | Below warning threshold. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs` | 549 | PASS | Below warning threshold. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4pqz.rs` | 451 | PASS | Below warning threshold. |
| `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs` | 89 | PASS | Below warning threshold. |

No touched `.rs` file is at or above the 3000-line refactor-before-closure
threshold.
