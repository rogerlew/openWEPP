# CRAP After

Evidence mode: Ran.

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-row5-after.lcov
cargo crap --workspace --lcov /tmp/openwepp-row5-after.lcov --min 0 --format json > /tmp/openwepp-crap-row5-after.json
jq -r '[.entries[] | select(.file | contains("/crates/openwepp-hillslope-orchestrator/src/hydrology/")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row5-after.json
```

Result:

- Row #5 owned production functions above CRAP 30: `0`.
- Full workspace functions above CRAP 30, all rows/scopes: `276`.
- Row #5 before-list moved from `11` unique entries (`22` duplicated report
  rows) to `0` entries above CRAP 30.
- No ADR-0021 complete-with-warnings disposition is used for row #5.

Notes:

- The only row-scope hydrology entries at the boundary were CRAP `30.00`, which
  is compliant because ADR-0021 is non-conforming above 30:
  `r7g_frost_trace_matches_filter` and
  `Sturm1995ClimateClassAssignmentError::fmt`.
- Full-workspace remaining offenders are outside the row #5 owned scope and are
  left to later rows in `kernel-boundary-cqr-burndown-execplan.md`.

Disposition: PASS. Row #5 primary CRAP closure is complete without ADR-0021
warnings.
