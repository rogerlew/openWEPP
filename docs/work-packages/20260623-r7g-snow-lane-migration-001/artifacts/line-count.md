# Line Count Governance

Status: COMPLETE.

Ran:

```bash
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_snow.rs crates/openwepp-hillslope-orchestrator/src/winter_column.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
```

| File | Lines | Status | Disposition |
| --- | ---: | --- | --- |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 232 | PASS | Import-only migration to include `DirectSnowLaneState`. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs` | 2160 | WARN | Existing oversized frame module; package adds localized constructor/commit bridge logic. Below 3000 hard threshold. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs` | 1133 | PASS | R4G snow mutation redirects to winter snow state. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs` | 75 | PASS | Adds split R7G snow test module binding. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_snow.rs` | 143 | PASS | New focused test module; avoids growing the inherited oversized aggregate direct-runtime test file. |
| `crates/openwepp-hillslope-orchestrator/src/winter_column.rs` | 314 | PASS | Adds snow state helpers below threshold. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1812 | PASS | Import replacement only. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 2708 | WARN | Existing runner source-test module remains below 3000 hard threshold. |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | 1787 | PASS | Direct lane seed now writes winter snow state. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 1803 | PASS | Direct publication helper remains below threshold after migration. |

Additional static note: the inherited
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
aggregate is 4091 lines, but after review the package-owned new tests were
moved into `direct_runtime_r7g_snow.rs`; the aggregate file has no final diff.
