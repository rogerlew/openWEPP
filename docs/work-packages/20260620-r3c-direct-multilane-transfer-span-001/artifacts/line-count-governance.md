# Line-Count Governance

Status: complete.
Evidence mode: Ran.

Ran:

- `wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/hillslope/03_tests.rs`

Results:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 1526 | PASS, below 2000 WARN threshold. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 95 | PASS, below 2000 WARN threshold. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 659 | PASS, below 2000 WARN threshold. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 641 | PASS, below 2000 WARN threshold. |

No touched `.rs` file reaches the 2000-line WARN threshold. No 3000-line
refactor blocker applies.
