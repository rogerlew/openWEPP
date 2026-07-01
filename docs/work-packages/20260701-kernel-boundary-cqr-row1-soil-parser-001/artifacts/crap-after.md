# CRAP After

Evidence mode: Ran.

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-row1-after.lcov
cargo crap --workspace --lcov /tmp/openwepp-row1-after.lcov --min 0 --format json > /tmp/openwepp-crap-row1-after.json
jq -r '[.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope\\.rs$")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row1-after.json
```

Result:

- Row #1 owned production functions above CRAP 30: `0`.
- Full workspace functions above CRAP 30, all rows/scopes: `266`.
- Row #1 remained `0 -> 0`; this row is a secondary-coverage restoration row.
- No ADR-0021 complete-with-warnings disposition is used for row #1.

Representative after values for row #1 watched functions:

| Function | Location | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `project_typed_soil_wb11_runtime` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:173` | 18.0 | 68.81 | 27.83 |
| `legacy_correct_layer_moisture` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:753` | 25.0 | 85.56 | 26.88 |
| `project_typed_soil_profile_publication` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:312` | 11.0 | 56.16 | 21.19 |
| `map_corrected_layer_runtime_symbols_to_parser_layers` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:648` | 13.0 | 87.65 | 13.32 |

Disposition: PASS. Row #1 primary CRAP status is clean without ADR-0021
warnings.
