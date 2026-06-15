# CQR01 CRAP After

Status: complete

Evidence mode: static-and-ran

## Static

After target coverage summary from `coverage_after_summary.json`:

- functions: `46` count, `42` covered, `91.30434782608695%`
- lines: `1259` count, `1088` covered, `86.41779189833201%`
- regions: `1414` count, `1232` covered, `87.12871287128714%`

Highest target CRAP rows after refactor from `crap_after.json`:

| Function | Line | Cyclomatic | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Wb11HydrologyKernel::require_frost_layer_water_state` | 372 | 16.0 | 92.13483146067416 | 16.12455583153302 |
| `Wb11HydrologyKernel::resolve_active_frost_coupling` | 98 | 8.0 | 51.78571428571429 | 15.173104956268219 |
| `Wb11HydrologyKernel::resolve_active_frost_soil_water_after` | 1137 | 10.0 | 82.5 | 10.535937500000001 |
| `Wb11HydrologyKernel::require_active_frost_fine_counts` | 209 | 8.0 | 70.96774193548387 | 9.566110570306467 |
| `Wb11HydrologyKernel::compute_active_frost_final_scalars` | 1226 | 9.0 | 85.07462686567165 | 9.26931504207632 |
| `Wb11HydrologyKernel::compute_active_frost_coupling` | 1453 | 8.0 | 96.07843137254902 | 8.003859752282304 |

Disposition: complete for the package quality dimension. The supporting CRAP
metric improved materially and no target row remains above `30`.

## Ran

- `cargo llvm-cov --workspace --ignore-run-fail --no-report`
  - exit_code: 0
- `cargo llvm-cov --workspace --ignore-run-fail --no-run --lcov --output-path .../lcov_after.info`
  - exit_code: 0
  - warning: `--no-run is deprecated`
- `cargo llvm-cov --workspace --ignore-run-fail --no-run --json --summary-only --output-path .../coverage_after_summary.json`
  - exit_code: 0
  - warning: `--no-run is deprecated`
- `cargo crap --workspace --lcov .../lcov_after.info --min 0 --format json --output .../crap_after.json`
  - exit_code: 0
  - warning: 124 source files had no matching LCOV entry
