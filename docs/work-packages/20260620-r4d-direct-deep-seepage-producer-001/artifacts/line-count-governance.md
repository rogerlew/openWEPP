# R4D Line-Count Governance

Status: complete.
Evidence mode: Ran.

R4D must record line counts for every touched `.rs` file.

Policy:

- 2000+ lines: WARN, record explicit disposition and split/sunset plan.
- 3000+ lines: blocking for non-exempt files; split before closure.

Current scaffold expectation:

- Keep `direct_runtime.rs` below the 2000-line WARN band if practical.
- Keep storage-producer additions inside the existing narrow storage module
  unless a pre-implementation review finds a better direct-runtime module
  boundary.

Results:

```text
1884 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 540 crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
 105 crates/openwepp-hillslope-orchestrator/src/lib.rs
1505 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
 646 crates/openwepp-runner/src/hillslope/03_tests.rs
```

Disposition:

- No touched Rust source file exceeds the 2000-line WARN band.
- No touched Rust source file approaches the 3000-line blocker.
- R4D stayed inside the existing storage direct-runtime module; no additional
  module split is required for closure.
