# R4E-H Line-Count Governance

Status: complete.

Evidence class: Ran.

Command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/hillslope/03_tests.rs
```

Baseline:

```text
1884 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 540 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1505 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 646 crates/openwepp-runner/src/hillslope/03_tests.rs
4575 total
```

No touched `.rs` file started at or above the 2000-line WARN threshold.

## Closure Recheck

Command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/hillslope/03_tests.rs
```

Result:

```text
1996 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 940 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
1998 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 650 crates/openwepp-runner/src/hillslope/03_tests.rs
5584 total
```

Verdict: PASS. Touched Rust files remain below the 2000-line warning threshold
and far below the 3000-line must-split threshold.
