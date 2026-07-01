# CRAP After

Evidence mode: Ran.

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-row3-after.lcov
cargo crap --workspace --lcov /tmp/openwepp-row3-after.lcov --min 0 --format json > /tmp/openwepp-crap-row3-after.json
jq -r '[.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management\\.rs$")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row3-after.json
```

Result:

- Row #3 owned production functions above CRAP 30: `0`.
- Full workspace functions above CRAP 30, all rows/scopes: `266`.
- Row #3 before-list moved from `1` unique entry (`2` duplicated report rows)
  to `0` entries above CRAP 30.
- No ADR-0021 complete-with-warnings disposition is used for row #3.

Representative after values for row #3 before-list:

| Function | Location | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `project_primary_drain_controls` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs:833` | 9.0 | 97.78 | 9.00 |

Note: after replacing test-only exact float comparisons with a tolerance helper
for clippy, the CRAP JSON was regenerated from the same LCOV against the final
source tree. Production code and exercised branches were unchanged.

Disposition: PASS. Row #3 primary CRAP closure is complete without ADR-0021
warnings.
