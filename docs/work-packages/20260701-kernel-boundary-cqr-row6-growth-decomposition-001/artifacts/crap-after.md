# CRAP After

Evidence mode: Ran.

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-row6-after.lcov
cargo crap --workspace --lcov /tmp/openwepp-row6-after.lcov --min 0 --format json > /tmp/openwepp-crap-row6-after.json
jq -r '[.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/direct_runtime/(growth|decomposition)\\.rs$")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row6-after.json
```

Result:

- Row #6 owned production functions above CRAP 30: `0`.
- Full workspace functions above CRAP 30, all rows/scopes: `272`.
- Row #6 before-list moved from `2` unique entries (`4` duplicated report
  rows) to `0` entries above CRAP 30.
- No ADR-0021 complete-with-warnings disposition is used for row #6.

Representative after values for row #6 validators:

| Function | Location | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `DirectGrowthInputs::validate_growth_shape_inputs` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:551` | 12.0 | 100.00 | 12.00 |
| `DirectGrowthInputs::validate_weather_and_thermal_inputs` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:535` | 9.0 | 100.00 | 9.00 |
| `DirectGrowthInputs::validate_perennial_schedule_domain` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:474` | 8.0 | 100.00 | 8.00 |
| `DirectGrowthInputs::validate_annual_schedule_domain` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:436` | 8.0 | 100.00 | 8.00 |
| `DirectGrowthInputs::validate_monthly_gddmax_inputs_if_needed` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:578` | 6.0 | 100.00 | 6.00 |
| `DirectGrowthInputs::validate_equation_inputs` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:524` | 5.0 | 100.00 | 5.00 |
| `DirectGrowthInputs::validate_root_growth_inputs` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:566` | 4.0 | 100.00 | 4.00 |
| `DirectGrowthInputs::validate_schedule_domain` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:426` | 3.0 | 100.00 | 3.00 |

Disposition: PASS. Row #6 primary CRAP closure is complete without ADR-0021
warnings.
