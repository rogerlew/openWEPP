# R6F Line-Count Governance

Status: complete-for-R6F-hold.

## Rust File Ledger

| File | Lines before | Lines after | Threshold state | Disposition |
|---|---:|---:|---|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2660 | 2739 | Pre-existing oversized core runtime file; worsened by bounded structural fields. | Exception: split would be unrelated to current WAT authority hold. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs` | 520 | 548 | Moderate. | No split. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs` | 1636 | 1655 | Pre-existing oversized subsystem file; small trait conversion added. | Exception: no local split just for `From` conversion. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 2588 | 2747 | Pre-existing oversized test file; focused R6F test and helper extraction added. | Exception: test kept with direct publication capture tests; clippy function-length gate passes. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4pqz.rs` | 403 | 409 | Acceptable. | No split. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2787 | 2804 | Pre-existing oversized runner file; marker-only gate edit retained below 3000-line hard threshold. | No required split for R6F after moving WAT reducer helpers to `04_direct_publication.rs`. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 2322 | 2322 | Pre-existing oversized helper file; unit correction changed lines but not count. | Exception: no split for two-line correction. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 1051 | 1298 | Large test module; focused R6F reduction and marker guard tests added. | Exception: kept near related R6 tests; clippy function-length gate passes. |
| `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` | 376 | 600 | Acceptable. | WAT reducer helpers moved here to keep intake file below hard threshold. |
| `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs` | 85 | 89 | Acceptable. | No split. |

## Split or Exception Decisions

| File | Decision | Reason | Follow-up or sunset |
|---|---|---|---|
| oversized runtime/test files | Bounded exception | R6F is held at producer authority; splitting those files now would add churn without closing the defect. | Revisit during R6G if direct producer implementation materially expands these files. |
| runner intake file | Refactor completed | R6F reducer initially pushed `00_runner_intake_and_lane_setup.rs` over 3000 lines. | Moved WAT reducer helpers to `04_direct_publication.rs`; final count is 2804. |
