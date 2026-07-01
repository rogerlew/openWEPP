# CRAP Before

Evidence mode: Ran.

Source:

- Reused final post-row-1 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row1-after.json`.

Extraction:

```text
jq -r '[.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate\\.rs$")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row1-after.json
```

Result:

- Row #2 production functions above CRAP 30: `0`.
- Worst row #2 production function in the before report:
  `HillslopeClimateRuntimeRequest::direct_day_forcing` at CRAP `5.02`.

Representative before values:

| Function | Location | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `HillslopeClimateRuntimeRequest::direct_day_forcing` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs:49` | 4.0 | 60.00 | 5.02 |
| `build_hillslope_climate_runtime_request` | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs:11` | 4.0 | 100.00 | 4.00 |

Disposition: baseline recorded. Row #2 primary CRAP is already clean; this
package is a secondary-coverage restoration row.
