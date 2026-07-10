# Coverage After

Evidence label: Static/Ran.

Status: `EXECUTED-PASS`

Target module:
`crates/openwepp-runner/src/errors.rs`

## Command Provenance

Ran:

- `cargo llvm-cov --workspace --test cli01_runner_contract_derived_tests --lcov --output-path /tmp/openwepp-cqr-nightly-08-runner-errors-targeted.lcov`
  - exit `0`
- `cargo llvm-cov --workspace --test cli01_runner_contract_derived_tests --json --output-path /tmp/openwepp-cqr-nightly-08-runner-errors-targeted-llvmcov.json --no-clean`
  - exit `0`

Metric file provenance:

| Path | Bytes | SHA-256 |
|---|---:|---|
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted.lcov` | `2065951` | `cb7ae88ba17dcca138c89872ed74749911f902d347be7d0d144d953ce83baa72` |
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-llvmcov.json` | `8909904` | `7b09382aa2c85bd5c980b4518eb48da86fec75f7e997cf73237f871add562ed4` |

## Target Coverage Summary

Ran:

| Metric | Covered | Count | Percent |
|---|---:|---:|---:|
| Lines | `266` | `267` | `99.625468164794` |
| Regions | `390` | `395` | `98.73417721518987` |
| Functions | `13` | `13` | `100.0` |
| Instantiations | `13` | `13` | `100.0` |
| Branches | `0` | `0` | `0.0` |

Baseline comparison:

- Before line coverage: `56/267`, `20.973782771535582%`.
- After line coverage: `266/267`, `99.625468164794%`.

Disposition:

- ADR-0021 glue-tier line threshold (`>=85%`) is met.
- ADR-0021 glue-tier region threshold (`>=85%`) is met.
- The target module is non-regressed versus baseline.
