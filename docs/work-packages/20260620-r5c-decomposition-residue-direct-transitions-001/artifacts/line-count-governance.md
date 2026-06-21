# Line-Count Governance

Static baseline:

```text
2092 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 481 crates/openwepp-hillslope-orchestrator/src/direct_runtime/normalization.rs
2459 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 709 crates/openwepp-runner/src/hillslope/03_tests.rs
```

## Baseline Disposition

Static:

- `direct_runtime.rs` is already above the 2000-line WARN threshold. R5C may
  touch it for central wiring only; new phase implementation belongs in
  `direct_runtime/decomposition.rs`.
- `tests/tests_mod/direct_runtime.rs` is already above the 2000-line WARN
  threshold. R5C focused tests should use a new split test module.
- No touched non-exempt `.rs` file is expected to exceed the 3000-line closure
  block threshold.

## Final Counts

Ran:

```text
2126 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 740 crates/openwepp-hillslope-orchestrator/src/direct_runtime/decomposition.rs
2471 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 616 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r5c.rs
 712 crates/openwepp-runner/src/hillslope/03_tests.rs
```

Disposition:

- `direct_runtime.rs`: WARN. R5C central wiring added constants, exports,
  frame fields, seed initialization, lifecycle status, and executor sequencing.
  The explicit seed constructor now has a scoped clippy allowance because direct
  day-frame initialization is intentionally exhaustive and fail-closed.
- `tests/tests_mod/direct_runtime.rs`: WARN, existing aggregate tests only.
  R5C focused tests were placed in `direct_runtime_r5c.rs`.
- No touched non-exempt `.rs` file reached the 3000-line closure-block
  threshold.
