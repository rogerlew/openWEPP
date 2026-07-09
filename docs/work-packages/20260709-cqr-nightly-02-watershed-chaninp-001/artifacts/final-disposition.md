# Final Disposition

Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

Package:
`20260709-cqr-nightly-02-watershed-chaninp-001`

Target:
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`

Outcome:

- Target CRAP rows above `30`: `0`.
- Max target CRAP after implementation: `20.816276483846725`.
- Target LCOV line coverage after implementation: `1891 / 1975 =
  95.746835443038%`.
- Target source-region coverage after implementation: `2431 / 2536 =
  95.8596214511041%` including module-local tests; production/source-helper
  source-region coverage `1517 / 1610 = 94.22360248447205%`.
- Lowest target function coverage row:
  `sample_riser_unsubmerged_curve`, line `578`, coverage
  `76.36363636363637%`.
- Full nextest gate: `1503 tests run: 1503 passed (8 slow), 3 skipped`.
- Dual verification: PASS after accepted verification findings were resolved.

Implementation summary:

- Added module-local characterization tests for private WS12 impoundment
  projection helpers.
- Extracted cohesive active-projection and riser-regression helpers while
  preserving arithmetic order and typed guard behavior.
- Strengthened numeric and typed-error assertions after review.
- Removed stale `clippy::too_many_lines` suppressions.

Gate status:

- All package final gates passed or passed with the documented
  `cargo llvm-cov --ignore-run-fail` note.
- The final `cargo nextest run --workspace --profile full` workflow passed.
- No current-scope gate is deferred.

Completion disposition:

- Complete. Commit this package before starting CQR nightly target rank `3`.
