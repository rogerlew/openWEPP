# CRAP Before

Evidence mode: Ran.

Source:

- Reused final post-row-3 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row3-after.json`.

Extraction:

```text
jq -r '[.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope\\.rs$")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row3-after.json
```

Result:

- Row #1 production functions above CRAP 30: `0`.
- Worst row #1 production function in the before report:
  `project_typed_soil_wb11_runtime` at CRAP `27.83`.

Representative before values:

| Function | Location | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `project_typed_soil_wb11_runtime` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:173` | 18.0 | 68.81 | 27.83 |
| `legacy_correct_layer_moisture` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:753` | 25.0 | 85.56 | 26.88 |
| `project_typed_soil_profile_publication` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:312` | 11.0 | 56.16 | 21.19 |

Disposition: baseline recorded. Row #1 primary CRAP is already clean; this
package is a secondary-coverage restoration row.
