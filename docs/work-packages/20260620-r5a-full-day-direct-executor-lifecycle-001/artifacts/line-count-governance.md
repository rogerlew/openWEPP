# Line-Count Governance

Ran:

`wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/hillslope/03_tests.rs`

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2054 | WARN: root direct-runtime lifecycle/reporting remains centralized for R5A; below 3000-line blocker. R5B should prefer extracting lifecycle or phase-status helpers if this grows again. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 144 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 2099 | WARN: existing shared direct-runtime test module is above 2000 lines; below 3000-line blocker. Future R5 phase tests should use dedicated `direct_runtime_r5*.rs` modules. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 707 | PASS |

No touched Rust file is at or above 3000 lines.
