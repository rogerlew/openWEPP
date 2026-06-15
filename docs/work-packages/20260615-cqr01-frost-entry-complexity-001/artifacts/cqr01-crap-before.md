# CQR01 CRAP Before

Status: complete

Evidence mode: static-and-ran

## Static

Tool versions:

- `cargo crap --version`: `cargo-crap 0.2.2`
- `cargo llvm-cov --version`: `cargo-llvm-cov 0.8.7`

Baseline target coverage summary from `coverage_before_summary.json`:

- functions: `13` count, `8` covered, `61.53846153846154%`
- lines: `874` count, `694` covered, `79.40503432494279%`
- regions: `1090` count, `920` covered, `84.40366972477065%`

Baseline target CRAP rows from `crap_before.json`:

| Function | Line | Cyclomatic | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Wb11HydrologyKernel::compute_active_frost_coupling` | 73 | 132.0 | 81.72839506172839 | 238.28646229402713 |
| `Wb11HydrologyKernel::resolve_active_frost_coupling` | 8 | 8.0 | 51.78571428571429 | 15.173104956268219 |

Note: `cargo crap` emitted duplicate target rows and a warning that 124 source
files had no matching LCOV entry. The target LCOV rows were present and used for
before/after comparison.

## Ran

- `cargo llvm-cov --workspace --ignore-run-fail --no-report`
  - exit_code: 0
- `cargo llvm-cov --workspace --ignore-run-fail --no-run --lcov --output-path .../lcov_before.info`
  - exit_code: 0
  - warning: `--no-run is deprecated`
- `cargo llvm-cov --workspace --ignore-run-fail --no-run --json --summary-only --output-path .../coverage_before_summary.json`
  - exit_code: 0
  - warning: `--no-run is deprecated`
- `cargo crap --workspace --lcov .../lcov_before.info --min 0 --format json --output .../crap_before.json`
  - exit_code: 0
  - warning: 124 source files had no matching LCOV entry
