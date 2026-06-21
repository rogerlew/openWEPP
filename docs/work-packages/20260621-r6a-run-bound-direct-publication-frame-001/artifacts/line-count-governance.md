# Line Count Governance

Status: warn.
Evidence mode: Ran.

Command:

```bash
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/api.rs crates/openwepp-runner/src/bin/openwepp-cli-hill.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs crates/openwepp-runner/src/hillslope/03_tests.rs
```

Result:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2609 | WARN: existing large executor file, below 3000-line block threshold. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 168 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 2588 | WARN: existing large test file, below 3000-line block threshold. |
| `crates/openwepp-runner/src/api.rs` | 59 | PASS |
| `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` | 121 | PASS |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2660 | WARN: existing large runner file, below 3000-line block threshold. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 2310 | WARN: existing large output helper file, below 3000-line block threshold. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 859 | PASS |

No touched Rust file crosses the 3000-line block threshold.
