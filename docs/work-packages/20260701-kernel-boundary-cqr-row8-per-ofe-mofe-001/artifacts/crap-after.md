# CRAP After

Evidence mode: Ran.

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-row8-after.lcov
cargo crap --workspace --lcov /tmp/openwepp-row8-after.lcov --min 0 --format json > /tmp/openwepp-crap-row8-after.json
jq -r '[.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/direct_runtime/(subsurface|03_executor)\\.rs$")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row8-after.json
```

Result:

- Row #8 owned production functions above CRAP 30: `0`.
- Full workspace functions above CRAP 30, all rows/scopes: `268`.
- Row #8 before-list moved from `2` unique entries (`4` duplicated report
  rows) to `0` entries above CRAP 30.
- No ADR-0021 complete-with-warnings disposition is used for row #8.

Representative after values for row #8 trace helpers:

| Function | Location | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `maybe_write_r7h_percolation_trace` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs:107` | 2.0 | 30.77 | 3.33 |
| `maybe_write_r7h_subsurface_saturation_trace` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs:179` | 2.0 | 30.77 | 3.33 |
| `r7h_subsurface_trace_matches_filter` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs:235` | 5.0 | 100.00 | 5.00 |
| `r7h_percolation_trace_line` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs:122` | 2.0 | 98.04 | 2.00 |
| `r7h_subsurface_saturation_trace_line` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs:194` | 2.0 | 97.37 | 2.00 |

Disposition: PASS. Row #8 primary CRAP closure is complete without ADR-0021
warnings.
