# Line-Count Governance

Ran:

`wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/normalization.rs crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/hillslope/03_tests.rs`

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2092 | WARN: root direct-runtime lifecycle/reporting remains centralized for R5B; below 3000-line blocker. R5C should prefer a dedicated module for decomposition/residue phases. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/normalization.rs` | 481 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 150 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 2459 | WARN: existing shared direct-runtime test module is above 2000 lines; below 3000-line blocker. R5C should use a dedicated `direct_runtime_r5*.rs` test module. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 709 | PASS |

No touched Rust file is at or above 3000 lines.
