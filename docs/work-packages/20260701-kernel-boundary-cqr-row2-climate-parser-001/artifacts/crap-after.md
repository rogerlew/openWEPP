# CRAP After

Evidence mode: Ran.

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-row2-after.lcov
cargo crap --workspace --lcov /tmp/openwepp-row2-after.lcov --min 0 --format json > /tmp/openwepp-crap-row2-after.json
jq -r '[.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate\\.rs$")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row2-after.json
```

Result:

- Row #2 owned production functions above CRAP 30: `0`.
- Full workspace functions above CRAP 30, all rows/scopes: `266`.
- Row #2 remained `0 -> 0`; this row is a secondary-coverage restoration row.
- No ADR-0021 complete-with-warnings disposition is used for row #2.

Representative after values for row #2 watched functions:

| Function | Location | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `HillslopeClimateRuntimeRequest::direct_day_forcing` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs:49` | 4.0 | 100.00 | 4.00 |
| `build_hillslope_climate_runtime_request` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs:11` | 4.0 | 100.00 | 4.00 |

Disposition: PASS. Row #2 primary CRAP status is clean without ADR-0021
warnings.
