# CRAP After

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
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-08-runner-errors-targeted.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-08-runner-errors-targeted-crap.json`
  - exit `0`

Metric file provenance:

| Path | Bytes | SHA-256 |
|---|---:|---|
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted.lcov` | `2065951` | `cb7ae88ba17dcca138c89872ed74749911f902d347be7d0d144d953ce83baa72` |
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-llvmcov.json` | `8909904` | `7b09382aa2c85bd5c980b4518eb48da86fec75f7e997cf73237f871add562ed4` |
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-crap.json` | `2671921` | `2163d74f2e21dd8cd94ca04b8e59e9a0e0894543422480b1393e49e242a5473b` |

## Target CRAP Summary

Ran:

- Deduplicated target rows above CRAP `30`: `0`.
- Deduplicated target row count: `13`.
- Max target CRAP: `20.0`.

Deduplicated target rows:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `HillslopeCliError::code` | `359` | `20.0` | `100.0` | `20.0` |
| `HillslopeCliError::fmt` | `386` | `20.0` | `100.0` | `20.0` |
| `ReleaseLintError::fmt` | `133` | `9.0` | `100.0` | `9.0` |
| `RunnerError::code` | `232` | `7.0` | `100.0` | `7.0` |
| `RunnerError::fmt` | `245` | `7.0` | `100.0` | `7.0` |
| `ReleaseMetadataError::code` | `35` | `6.0` | `100.0` | `6.0` |
| `ReleaseMetadataError::fmt` | `47` | `6.0` | `100.0` | `6.0` |
| `HillslopeCliError::source` | `526` | `6.0` | `100.0` | `6.0` |
| `format_hillslope_topology_mismatch` | `498` | `5.0` | `95.45454545454545` | `5.002347858752818` |

## Baseline Comparison

Static/Ran:

- Before: `6` deduplicated rows above CRAP `30`; max CRAP
  `192.89940656693898`.
- After: `0` deduplicated rows above CRAP `30`; max CRAP `20.0`.

Disposition:

- ADR-0021 CRAP closure target is met for this package.
- No production decomposition was required because all target rows had CC
  `<= 30`; coverage characterization reduced CRAP to the existing CC floor.
