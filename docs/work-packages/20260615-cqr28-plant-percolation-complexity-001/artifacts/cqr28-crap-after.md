# CQR28 CRAP After

Ran: after metrics were captured with:

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr28-plant-percolation-complexity-001/artifacts/lcov_after.info`
- `cargo crap --workspace --lcov docs/work-packages/20260615-cqr28-plant-percolation-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr28-plant-percolation-complexity-001/artifacts/crap_after.json`

Final target row:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Wb11HydrologyKernel::run_percolation` | 770 | 14.0 | 74.64788732394366 | 17.19373252009578 |

New helper rows:

| Function | CC | Coverage | CRAP |
|---|---:|---:|---:|
| `validate_wb18_legacy_percolation_inputs` | 8.0 | 90.625 | 8.052734375 |
| `read_wb18_percolation_layers` | 13.0 | 79.76190476190477 | 14.400862825288842 |
| `resolve_wb18_percolation_same_pass_infiltration` | 5.0 | 100.0 | 5.0 |
| `resolve_wb18_percolation_lane_config` | 14.0 | 82.79569892473118 | 14.99808418401282 |
| `run_wb18_percolation_routing` | 5.0 | 94.11764705882352 | 5.005088540606554 |
| `run_wb18_percolation_substep` | 3.0 | 94.11764705882352 | 3.0018318746183597 |
| `route_wb18_percolation_layer` | 10.0 | 83.78378378378379 | 10.426430813574713 |
| `wb18_percolation_layer_fx` | 11.0 | 53.84615384615385 | 22.896222121074196 |
| `wb18_effective_layer_conductivity` | 10.0 | 53.333333333333336 | 20.162962962962965 |
| `wb18_layer_pei_unscaled` | 4.0 | 66.66666666666666 | 4.5925925925925934 |
| `canonicalize_wb18_deep_percolation_roundoff` | 3.0 | 100.0 | 3.0 |
| `resolve_wb18_percolation_soil_water_after` | 10.0 | 69.0909090909091 | 12.952967693463561 |
| `build_wb18_percolation_response` | 5.0 | 98.21428571428571 | 5.000142356049563 |

Closure: target and all newly extracted helpers are CRAP `<= 30`.

Warnings: same-file out-of-scope rows above CRAP `30` remain
`resolve_effective_wb18_frozen_depth` at `198.68137117979634` and
`run_plant_root_uptake` at `57.39645909305284`. `cargo crap` emitted the same
126 LCOV source-map warnings as before.
