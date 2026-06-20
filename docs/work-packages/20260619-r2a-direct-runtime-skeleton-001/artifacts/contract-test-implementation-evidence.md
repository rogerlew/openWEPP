# R2A Contract-Test Implementation Evidence

Status: complete.
Evidence mode: Ran.

Record focused tests for:

- direct-frame type prohibitions;
- direct skeleton selection and default-disabled inactivity;
- no-compatibility call/counter proof;
- fail-closed invalid direct skeleton inputs if such inputs exist.

No phase identity or publication identity test may be used to claim R3/R4/R6
readiness in this package.

## Focused Test Evidence

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`
  passed after review fixes: 3 tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture`
  passed after review fixes: 2 tests.

Covered tests:

- `r2a_direct_skeleton_runs_noop_and_records_only_direct_audit_counters`:
  proves the no-op direct skeleton constructs one direct run frame, one
  executor, one skeleton run, one direct day frame per lane, and 14 phase views
  per lane.
- `r2a_direct_skeleton_fails_closed_on_invalid_identity`: proves zero lane
  and zero day direct skeleton identities are rejected.
- `r2a_direct_runtime_source_excludes_compatibility_storage_tokens`: scans
  `src/direct_runtime.rs` for the forbidden compatibility storage and call
  tokens.
- `r2a_default_fixture_run_constructs_no_direct_runtime_skeleton`: proves the
  default compatibility runner path completes outputs with every direct
  skeleton audit counter still zero.
- `r2a_explicit_direct_skeleton_selection_runs_before_compatibility_outputs`:
  proves the explicit direct skeleton selection runs once before compatibility
  outputs.

Review correction: runtime counters are not used as forbidden-call proof. The
forbidden-call claim is static source/call-graph proof because adding dynamic
hooks to compatibility hot paths would itself violate the zero-cost-disabled
architecture goal.

These tests do not claim phase identity, hydrology readiness, publication
readiness, or default activation readiness.
