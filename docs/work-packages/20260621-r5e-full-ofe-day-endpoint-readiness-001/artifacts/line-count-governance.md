# Line Count Governance

Status: complete.
Evidence mode: Ran.

Touched Rust files:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`

Measurement:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
```

Result:

```text
2158 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
2526 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
4684 total
```

Disposition:

- `direct_runtime.rs` remains in the 2000-line WARN band, below the 3000-line
  blocker. R5E adds one report field and one assignment; splitting the file
  would be unrelated to endpoint-readiness accounting.
- `tests_mod/direct_runtime.rs` remains in the 2000-line WARN band, below the
  3000-line blocker. R5E adds a focused endpoint-readiness test and extends the
  existing no-compatibility source scan.

Verdict: WARN acknowledged; no blocker.
