# PERFDEEP09 Line-Count Governance

Status: complete.
Evidence class: Ran.

Command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs \
      crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/boundaries.rs
```

Result:

```text
1682 crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs
 550 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/boundaries.rs
2232 total
```

Disposition:

- No touched Rust file reaches the 2000-line WARN threshold.
- No touched Rust file reaches the 3000-line required-refactor threshold.
- `scheduler.rs` was not touched.
