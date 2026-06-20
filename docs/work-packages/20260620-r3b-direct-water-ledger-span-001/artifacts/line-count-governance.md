# R3B Line-Count Governance

Status: complete.
Evidence mode: Static + Ran.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/hillslope/03_tests.rs
```

Result:

```text
 1147 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
   93 crates/openwepp-hillslope-orchestrator/src/lib.rs
  441 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
  638 crates/openwepp-runner/src/hillslope/03_tests.rs
 2319 total
```

Disposition:

- No touched Rust file is at or above the 2000-line WARN threshold.
- No touched Rust file is at or above the 3000-line required-refactor
  threshold.
